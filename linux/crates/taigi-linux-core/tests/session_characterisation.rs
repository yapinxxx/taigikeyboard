//! What one key does through the whole Linux session path —
//! `process_raw_key` over a runtime on the repo's real dictionaries — pinned
//! before the key-intent executor moves into `taigi-desktop-core`
//! (maintainability roadmap R3(c)). Every expected value here was read by
//! running HEAD, not derived.
//!
//! Hermetic: nothing is learned (the data directory cannot be created), so a
//! commit never reorders a later test's list; and the tests take one lock,
//! because the lexicon and the engine's user data are one per process.

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use taigi_desktop_core::composing::ContextToken;
use taigi_desktop_core::settings::keys;
use taigi_desktop_storage::SettingsFileStore;
use taigi_linux_core::session::{end_session, process_raw_key, EngineState, CAP_SURROUNDING_TEXT};
use taigi_linux_core::{Emit, LookupTableContent, Runtime};
use taigi_linux_platform::key_translation::state;
use taigi_linux_platform::{RawKeyEvent, UserDirectories};

const SPACE: u32 = 0x20;
const COMMA: u32 = 0x2c;
const PERIOD: u32 = 0x2e;
const RETURN: u32 = 0xff0d;
const ESCAPE: u32 = 0xff1b;
const BACKSPACE: u32 = 0xff08;

/// A session over the real dictionaries with auto-space on or off, and a
/// client that can (or cannot) delete surrounding text.
struct Session {
    _directory: tempfile::TempDir,
    runtime: Runtime,
    token: ContextToken,
    state: EngineState,
}

/// Ends the composition like a focus-out, so the engine singleton carries
/// nothing into the next test.
impl Drop for Session {
    fn drop(&mut self) {
        end_session(&self.runtime, self.token, &mut self.state);
    }
}

/// One test at a time: the lexicon and the user data are one per process.
/// Held for the whole test, so a test may open several sessions.
fn serial() -> MutexGuard<'static, ()> {
    static SERIAL: Mutex<()> = Mutex::new(());
    SERIAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl Session {
    fn new(is_auto_space_enabled: bool, can_delete_surrounding: bool) -> Self {
        let directory = tempfile::tempdir().expect("tempdir");
        let config = directory.path().join("config");
        std::fs::create_dir_all(&config).expect("config directory");
        SettingsFileStore::new(&config)
            .update(|document| {
                document.set_bool(&keys::IS_AUTO_SPACE_ENABLED, is_auto_space_enabled);
                // These characterisations are written around the §34 literal
                // leading the list; Show Typed Text First ships OFF since
                // 2026-10-02, so it is pinned ON here.
                document.set_bool(&keys::IS_LITERAL_ROMAN_CANDIDATE_ENABLED, true);
            })
            .expect("settings written");
        // A data directory under a FILE cannot be created: no learning.
        let blocker = directory.path().join("not-a-directory");
        std::fs::write(&blocker, "").expect("blocker file");
        let runtime = Runtime::from_directories(
            Some(UserDirectories {
                config,
                data: blocker.join("data"),
            }),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../assets/dictionaries"),
        );
        let state = EngineState {
            capabilities: if can_delete_surrounding {
                CAP_SURROUNDING_TEXT
            } else {
                0
            },
            ..EngineState::default()
        };
        let token = runtime.allocate_token();
        Self {
            _directory: directory,
            runtime,
            token,
            state,
        }
    }

    fn press_with(&mut self, keyval: u32, modifiers: u32) -> (bool, Vec<Emit>) {
        self.press_key(keyval, 0, modifiers)
    }

    fn press_key(&mut self, keyval: u32, keycode: u32, modifiers: u32) -> (bool, Vec<Emit>) {
        let reply = process_raw_key(
            &self.runtime,
            self.token,
            &mut self.state,
            RawKeyEvent {
                keyval,
                keycode,
                state: modifiers,
            },
        );
        (reply.handled, reply.emits)
    }

    /// A consumed key's emits.
    fn press(&mut self, keyval: u32) -> Vec<Emit> {
        let (is_handled, emits) = self.press_with(keyval, 0);
        assert!(is_handled, "key {keyval:#x} is consumed");
        emits
    }

    /// Types `letters` into a composition and answers the last key's list.
    fn type_word(&mut self, letters: &str) -> LookupTableContent {
        let mut last = Vec::new();
        for letter in letters.chars() {
            last = self.press(letter as u32);
        }
        match last.as_slice() {
            [Emit::Preedit { text, .. }, Emit::LookupTable(table)] => {
                assert_eq!(text, letters);
                table.clone()
            }
            other => panic!("typing shows the preedit and its list, got {other:?}"),
        }
    }
}

fn commit(text: &str) -> Emit {
    Emit::Commit(text.to_owned())
}

#[test]
fn typing_shows_the_preedit_and_a_labelled_list_led_by_the_literal() {
    let _serial = serial();
    let mut session = Session::new(false, false);
    let table = session.type_word("ho");
    // trace: cell 0 is the literal roman; the first hanji cell is 好 hó; the
    // literal takes no key (§34, as on macOS / Windows), so the Standard slot
    // keys label the page from cell 1; vertical, cursor on the literal.
    assert_eq!(&table.candidates[..2], ["ho", "好 hó"]);
    assert_eq!(table.labels, ["", "q", "w", "d", "f", "z", "x", "v", "y"]);
    assert_eq!((table.cursor, table.cursor_visible), (0, true));
    assert_eq!((table.page_size, table.vertical), (9, true));
}

#[test]
fn a_slot_key_commits_its_cell_and_closes_the_list() {
    let _serial = serial();
    for is_auto_space_enabled in [false, true] {
        let mut session = Session::new(is_auto_space_enabled, true);
        session.type_word("ho");
        // The first slot key picks the first dictionary cell, not the literal.
        assert_eq!(
            session.press('q' as u32),
            [Emit::ClearPreedit, commit("好"), Emit::HideLookupTable],
            "auto_space={is_auto_space_enabled}: a Hanji commit takes no space"
        );
        assert!(!session.state.armed_auto_space);
    }
}

#[test]
fn return_commits_the_literal_and_auto_space_follows_a_romanization() {
    let _serial = serial();
    let mut session = Session::new(false, true);
    session.type_word("ho");
    assert_eq!(
        session.press(RETURN),
        [Emit::ClearPreedit, commit("ho"), Emit::HideLookupTable]
    );
    assert!(!session.state.armed_auto_space);

    let mut session = Session::new(true, true);
    session.type_word("ho");
    assert_eq!(
        session.press(RETURN),
        [
            Emit::ClearPreedit,
            commit("ho"),
            commit(" "),
            Emit::HideLookupTable
        ]
    );
    assert!(session.state.armed_auto_space, "the space is swappable");
}

#[test]
fn escape_abandons_the_composition() {
    let _serial = serial();
    let mut session = Session::new(true, true);
    session.type_word("ho");
    assert_eq!(
        session.press(ESCAPE),
        [Emit::ClearPreedit, Emit::HideLookupTable]
    );
    assert!(!session.state.armed_auto_space);
}

/// trace: Space ships on Confirm Key (USER 2026-10-15) — it commits the
/// highlighted cell, here the literal `tsiah`, like Return. Before, Space
/// wrote the other script, which the literal cell has none of, so the list
/// only redrew.
#[test]
fn space_inside_a_composition_confirms_the_highlighted_cell() {
    let _serial = serial();
    let mut session = Session::new(true, true);
    session.type_word("tsiah");
    assert_eq!(
        session.press(SPACE),
        [
            Emit::ClearPreedit,
            commit("tsiah"),
            commit(" "),
            Emit::HideLookupTable
        ]
    );
    assert!(session.state.armed_auto_space, "the space is swappable");
}

#[test]
fn punctuation_inside_a_composition_commits_the_literal_with_the_full_width_mark() {
    let _serial = serial();
    let mut session = Session::new(false, true);
    session.type_word("tsiah");
    assert_eq!(
        session.press(COMMA),
        [Emit::ClearPreedit, commit("tsiah，"), Emit::HideLookupTable]
    );
    assert!(!session.state.armed_auto_space);

    let mut session = Session::new(true, true);
    session.type_word("tsiah");
    assert_eq!(
        session.press(COMMA),
        [
            Emit::ClearPreedit,
            commit("tsiah， "),
            Emit::HideLookupTable
        ]
    );
    assert!(session.state.armed_auto_space);
}

#[test]
fn bare_punctuation_after_hanji_is_full_width() {
    let _serial = serial();
    let mut session = Session::new(true, true);
    session.type_word("ho");
    session.press('w' as u32);
    assert_eq!(session.press(PERIOD), [commit("。")]);
}

#[test]
fn the_auto_space_swap_needs_the_surrounding_text_capability() {
    let _serial = serial();
    // Without it the space stays and the mark is typed after it, full width.
    let mut session = Session::new(true, false);
    session.type_word("ho");
    session.press(RETURN);
    assert!(session.state.armed_auto_space);
    assert_eq!(session.press(COMMA), [commit("，")]);
    assert!(!session.state.armed_auto_space);

    // With it the space goes and the mark takes its half-width romanized form
    // plus the space.
    let mut session = Session::new(true, true);
    session.type_word("ho");
    session.press(RETURN);
    assert_eq!(
        session.press(COMMA),
        [
            Emit::DeleteSurrounding {
                offset: -1,
                count: 1
            },
            commit(", ")
        ]
    );
    // trace (read by running HEAD): the swapped mark ends in a space of its
    // own, so the next mark may swap again.
    assert!(session.state.armed_auto_space);
}

#[test]
fn the_armed_swap_lasts_exactly_one_key() {
    let _serial = serial();
    let mut session = Session::new(true, true);
    session.type_word("ho");
    session.press(RETURN);
    assert!(session.state.armed_auto_space);
    session.press('x' as u32);
    assert!(!session.state.armed_auto_space, "any key spends the arm");
    session.press(ESCAPE);
    assert_eq!(session.press(COMMA), [commit("，")], "no swap two keys on");
}

#[test]
fn the_symbol_picker_neither_swaps_nor_keeps_the_arm() {
    let _serial = serial();
    let mut session = Session::new(true, true);
    session.type_word("ho");
    session.press(RETURN);
    assert!(session.state.armed_auto_space);
    let (is_handled, emits) = session.press_with(COMMA, state::CONTROL | state::MOD1);
    assert!(is_handled);
    let [Emit::LookupTable(picker)] = emits.as_slice() else {
        panic!("Ctrl+Alt+, opens the picker, got {emits:?}");
    };
    let first = picker.candidates[0].clone();
    assert_eq!(
        session.press(RETURN),
        [commit(&first), Emit::HideLookupTable]
    );
    assert!(!session.state.armed_auto_space);
}

#[test]
fn backspace_with_nothing_composed_is_the_clients() {
    let _serial = serial();
    let mut session = Session::new(true, true);
    assert_eq!(session.press_with(BACKSPACE, 0), (false, Vec::new()));
}

#[test]
fn a_refetching_switch_obeys_the_candidate_window_setting() {
    let _serial = serial();
    // trace (read by running): Ctrl+Alt+H = CycleCandidateDisplayMode, which
    // re-fetches the open list under the new mode.
    let mut session = Session::new(true, true);
    session.type_word("ho");
    let (is_handled, emits) = session.press_with('h' as u32, state::CONTROL | state::MOD1);
    assert!(is_handled);
    assert!(
        matches!(emits.first(), Some(Emit::LookupTable(_))),
        "window on: the refetched list is shown, got {emits:?}"
    );

    // The window switched off (in the settings window) while the list was
    // up: the next refetch fetches nothing and takes the list down.
    drop(session);
    let mut session = Session::new(true, true);
    session.type_word("ho");
    let config = session._directory.path().join("config");
    SettingsFileStore::new(&config)
        .update(|document| document.set_bool(&keys::IS_CANDIDATE_WINDOW_ENABLED, false))
        .expect("settings written");
    let (is_handled, emits) = session.press_with('h' as u32, state::CONTROL | state::MOD1);
    assert!(is_handled);
    assert_eq!(
        emits,
        [Emit::HideLookupTable, Emit::ModeChanged, Emit::AnnounceMode]
    );
    assert!(session.state.candidates.is_empty());
}

#[test]
fn the_literal_stays_unkeyed_when_a_switch_presents_the_list_again() {
    let _serial = serial();
    // Ctrl+Alt+H re-fetches and re-presents the open list outside the key
    // path that builds a list; the literal must still take no key there.
    let mut session = Session::new(false, false);
    session.type_word("ho");
    let (_, emits) = session.press_with('h' as u32, state::CONTROL | state::MOD1);
    let Some(Emit::LookupTable(table)) = emits.first() else {
        panic!("the refetched list is shown, got {emits:?}");
    };
    assert_eq!(table.candidates[0], "ho");
    assert_eq!(table.labels[..2], ["", "q"]);

    // Show Typed Text First OFF: the list leads with a dictionary cell, which
    // takes the first key.
    let config = session._directory.path().join("config");
    SettingsFileStore::new(&config)
        .update(|document| document.set_bool(&keys::IS_LITERAL_ROMAN_CANDIDATE_ENABLED, false))
        .expect("settings written");
    let (_, emits) = session.press_with('h' as u32, state::CONTROL | state::MOD1);
    let Some(Emit::LookupTable(table)) = emits.first() else {
        panic!("the refetched list is shown, got {emits:?}");
    };
    assert_ne!(table.candidates[0], "ho");
    assert_eq!(table.labels[0], "q");
}

const CTRL_ALT: u32 = state::CONTROL | state::MOD1;

/// Desktop TPS P3: Ctrl+Alt+P enters TPS from TL; a second press returns to
/// TL with the glyphs still composing, and the next key commits them as
/// shown before it starts the TL composition. Read by running HEAD
/// (2026-10-03): `e` ㄍ, `8` ㄚ (D2 table).
#[test]
fn switch_tps_round_trip_commits_the_glyphs_on_the_next_key() {
    let _serial = serial();
    let mut session = Session::new(false, false);
    let (_, emits) = session.press_with('p' as u32, CTRL_ALT);
    assert_eq!(emits, [Emit::ModeChanged, Emit::AnnounceMode]);
    session.press('e' as u32);
    session.press('8' as u32);
    let (_, emits) = session.press_with('p' as u32, CTRL_ALT);
    assert_eq!(
        emits,
        [Emit::ModeChanged, Emit::AnnounceMode],
        "the switch commits nothing itself; TPS typing put no table up (D7)"
    );
    let emits = session.press('a' as u32);
    assert_eq!(
        emits[..3],
        [
            Emit::ClearPreedit,
            commit("ㄍㄚ"),
            Emit::Preedit {
                text: "a".to_owned(),
                caret: 1
            }
        ]
    );
}

/// Negative control: TL ↔ POJ crosses no TPS, so the composition carries on.
#[test]
fn switch_romanization_keeps_the_composition() {
    let _serial = serial();
    let mut session = Session::new(false, false);
    session.press('t' as u32);
    session.press('a' as u32);
    session.press_with('c' as u32, CTRL_ALT);
    let emits = session.press('i' as u32);
    assert!(
        matches!(emits.as_slice(), [Emit::Preedit { text, .. }, Emit::LookupTable(_)] if text == "tai"),
        "{emits:?}"
    );
}

/// The other way: a TL composition left under TPS is committed by the first
/// glyph key, which then starts the TPS composition.
#[test]
fn switch_into_tps_commits_the_romanization_on_the_next_key() {
    let _serial = serial();
    let mut session = Session::new(false, false);
    session.type_word("tai");
    session.press_with('p' as u32, CTRL_ALT);
    let emits = session.press('e' as u32);
    // trace, read by running: the commit, then `e` as ㄍ (D2).
    assert_eq!(
        emits[..3],
        [
            Emit::ClearPreedit,
            commit("tai"),
            Emit::Preedit {
                text: "ㄍ".to_owned(),
                caret: 1
            }
        ]
    );
}

/// Desktop TPS D7 on Linux: typing puts no table up; ↓ opens it; the
/// number-row `2` (X keycode 11) then picks the second cell; Escape over it
/// closes it and keeps the glyphs. The pick nails the word and closes the
/// table, the composition stays up; Enter writes it (Hanji conversion H4,
/// B2).
#[test]
fn tps_opens_the_table_on_demand_and_the_number_row_picks() {
    const DOWN: u32 = 0xff54;
    let _serial = serial();
    let mut session = Session::new(false, false);
    session.press_with('p' as u32, CTRL_ALT);
    session.press('e' as u32);
    let emits = session.press('8' as u32);
    assert!(
        !emits
            .iter()
            .any(|emit| matches!(emit, Emit::LookupTable(_))),
        "no table while typing: {emits:?}"
    );
    let opened = session.press(DOWN);
    let [Emit::LookupTable(table)] = opened.as_slice() else {
        panic!("↓ opens the table: {opened:?}");
    };
    let second = table.candidates[1].clone();
    assert_eq!(session.press(ESCAPE), [Emit::HideLookupTable]);
    let reopened = session.press(DOWN);
    assert!(
        matches!(reopened.as_slice(), [Emit::LookupTable(_)]),
        "↓ opens it again: {reopened:?}"
    );
    let (is_handled, emits) = session.press_key('2' as u32, 11, 0);
    assert!(is_handled);
    assert_eq!(
        emits,
        [
            Emit::Preedit {
                text: second.clone(),
                caret: 1
            },
            Emit::HideLookupTable
        ],
        "the number row picks the second cell"
    );
    assert_eq!(session.press(RETURN), [Emit::ClearPreedit, commit(&second)]);
}

/// Hanji conversion on Linux: `e` `8` `4` (ㄍㄚˋ) shows converted; plain ←
/// steps the caret over the word; Shift+Enter writes the glyphs, Enter the
/// Hanji. trace, read by running: ㄍㄚˋ → 絞 (E1 P5a corpus pick: 絞/ká 66 > 假/ká 25).
#[test]
fn tps_converts_a_closed_reading_and_the_commit_keys_part_ways() {
    const LEFT: u32 = 0xff51;
    let _serial = serial();
    let mut session = Session::new(false, false);
    session.press_with('p' as u32, CTRL_ALT);
    session.press('e' as u32);
    session.press('8' as u32);
    let typed = session.press('4' as u32);
    assert_eq!(
        typed,
        [Emit::Preedit {
            text: "絞".to_owned(),
            caret: 1
        }]
    );
    assert_eq!(
        session.press(LEFT),
        [Emit::Preedit {
            text: "絞".to_owned(),
            caret: 0
        }]
    );
    let (is_handled, emits) = session.press_with(RETURN, state::SHIFT);
    assert!(is_handled);
    assert_eq!(emits, [Emit::ClearPreedit, commit("ㄍㄚˋ")]);
    session.press('e' as u32);
    session.press('8' as u32);
    session.press('4' as u32);
    assert_eq!(session.press(RETURN), [Emit::ClearPreedit, commit("絞")]);
}

fn sentence_settings(session: &Session, mode: taigi_desktop_core::settings::InputMode) {
    SettingsFileStore::new(&session._directory.path().join("config"))
        .update(|document| {
            document.set_choice(&keys::INPUT_MODE, mode);
            document.set_bool(&keys::EDITABLE_SENTENCE_ENABLED, true);
            document.set_bool(&keys::IS_LITERAL_ROMAN_CANDIDATE_ENABLED, false);
        })
        .unwrap();
}

fn latest_table(emits: &[Emit]) -> LookupTableContent {
    emits
        .iter()
        .find_map(|emit| match emit {
            Emit::LookupTable(table) => Some(table.clone()),
            _ => None,
        })
        .expect("candidate combinations")
}

fn choose_hanji_with_letter(
    session: &mut Session,
    mut table: LookupTableContent,
    hanji: &str,
) -> Vec<Emit> {
    let position = table
        .candidates
        .iter()
        .position(|candidate| candidate.split_whitespace().next() == Some(hanji))
        .unwrap_or_else(|| panic!("missing {hanji}: {table:?}"));
    while position / 6 != table.cursor as usize / 6 {
        table = latest_table(&session.press(0xff56)); // PageDown
    }
    let letter = table.labels[position % 6].chars().next().unwrap();
    session.press(letter as u32)
}

#[test]
fn ithuan_letters_choose_whole_or_shorter_combinations_without_committing() {
    let _serial = serial();
    for mode in [
        taigi_desktop_core::settings::InputMode::Tl,
        taigi_desktop_core::settings::InputMode::Poj,
    ] {
        let mut session = Session::new(false, false);
        sentence_settings(&session, mode);
        let mut emits = Vec::new();
        for ch in "tak8keho2".chars() {
            emits = session.press(ch as u32);
        }
        let table = latest_table(&emits);
        assert_eq!(table.page_size, 6);
        assert_eq!(table.labels, ["q", "w", "d", "f", "z", "x"]);
        assert!(table.candidates[0].starts_with("逐家好 "), "{table:?}");
        assert!(table.candidates[1].starts_with("逐家 "), "{table:?}");
        let selected = session.press('w' as u32);
        assert!(!selected.iter().any(|e| matches!(e, Emit::Commit(_))));
        let selected = session.press('q' as u32);
        assert!(
            selected
                .iter()
                .any(|e| matches!(e,Emit::Preedit{text,..} if text=="逐家好")),
            "{selected:?}"
        );
        assert!(!selected.iter().any(|e| matches!(e, Emit::Commit(_))));
        assert!(session.press(RETURN).contains(&commit("逐家好")));
    }
}

#[test]
fn ithuan_reopens_a_selected_combination_and_reselects_it_with_letters() {
    let _serial = serial();
    let mut session = Session::new(false, false);
    sentence_settings(&session, taigi_desktop_core::settings::InputMode::Poj);
    for ch in "tak8keho2".chars() {
        session.press(ch as u32);
    }
    session.press('q' as u32);
    let mut moved = Vec::new();
    for _ in 0..3 {
        moved = session.press(0xff51);
    }
    let table = latest_table(&moved);
    // The list describes the prefix combination, not just 家's homophones.
    assert!(table.candidates[0].starts_with("逐家 "), "{table:?}");
    let prefix = choose_hanji_with_letter(&mut session, table, "逐");
    let changed = choose_hanji_with_letter(&mut session, latest_table(&prefix), "雞");
    assert!(!changed.iter().any(|e| matches!(e, Emit::Commit(_))));
    session.press(0xff57); // End: include the untouched final reading.
    let selected = session.press('q' as u32);
    assert!(
        selected
            .iter()
            .any(|e| matches!(e,Emit::Preedit{text,..} if text=="逐雞好")),
        "{selected:?}"
    );
    assert!(session.press(RETURN).contains(&commit("逐雞好")));
}

#[test]
fn ithuan_six_letter_slots_follow_the_candidate_page_and_enter_accepts_the_preview() {
    let _serial = serial();
    let mut session = Session::new(false, false);
    sentence_settings(&session, taigi_desktop_core::settings::InputMode::Poj);
    let mut typed = Vec::new();
    for ch in "ho".chars() {
        typed = session.press(ch as u32);
    }
    let table = latest_table(&typed);
    let expected = table.candidates[6]
        .split_whitespace()
        .next()
        .unwrap()
        .to_string();
    session.press(0xff56); // PageDown
    let picked = session.press('q' as u32);
    assert!(
        picked
            .iter()
            .any(|e| matches!(e,Emit::Preedit{text,..} if text==&expected)),
        "{picked:?}"
    );
    assert!(session.press(RETURN).contains(&commit(&expected)));
    for ch in "tak8keho2".chars() {
        session.press(ch as u32);
    }
    assert!(session.press(RETURN).contains(&commit("逐家好")));
}

#[test]
fn sentence_poj_hyphen_separates_syllables_and_hanji_shows_the_reading() {
    use taigi_desktop_core::settings::{CandidateDisplayMode, InputMode};
    let _serial = serial();
    for display in [
        CandidateDisplayMode::Combined,
        CandidateDisplayMode::RomanOnly,
    ] {
        let mut session = Session::new(false, false);
        sentence_settings(&session, InputMode::Poj);
        SettingsFileStore::new(&session._directory.path().join("config"))
            .update(|document| document.set_choice(&keys::CANDIDATE_DISPLAY_MODE, display))
            .unwrap();
        for ch in "tai5".chars() {
            session.press(ch as u32);
        }
        let separated = session.press('-' as u32);
        assert!(!separated.iter().any(|e| matches!(e, Emit::Commit(_))));
        assert!(
            separated
                .iter()
                .any(|e| matches!(e, Emit::Preedit { text, .. } if text == "tâi-")),
            "{separated:?}"
        );
        let mut typed = Vec::new();
        for ch in "gi2".chars() {
            typed = session.press(ch as u32);
        }
        let table = latest_table(&typed);
        let roman_slot = if display == CandidateDisplayMode::Combined {
            assert_eq!(table.candidates[0], "台語 tâi-gí");
            'w'
        } else {
            'q'
        };
        let selected = session.press(roman_slot as u32);
        assert!(!selected.iter().any(|e| matches!(e, Emit::Commit(_))));
        assert!(session.press(RETURN).contains(&commit("tâi-gí")));
        // Outside a composition, Space still belongs to the application.
        assert!(!session.press_with(SPACE, 0).0);
    }
}

#[test]
fn no_hyphens_formats_the_whole_poj_candidate_for_one_selection() {
    use taigi_desktop_core::settings::{CandidateDisplayMode, InputMode, SyllableSeparator};
    let _serial = serial();
    for display in [
        CandidateDisplayMode::Combined,
        CandidateDisplayMode::RomanOnly,
    ] {
        for (separator, expected) in [
            (SyllableSeparator::Hyphen, "lí-hó"),
            (SyllableSeparator::Space, "lí hó"),
        ] {
            let mut session = Session::new(false, false);
            sentence_settings(&session, InputMode::Poj);
            SettingsFileStore::new(&session._directory.path().join("config"))
                .update(|document| {
                    document.set_choice(&keys::CANDIDATE_DISPLAY_MODE, display);
                    document.set_choice(&keys::SYLLABLE_SEPARATOR, separator);
                })
                .unwrap();
            let mut typed = Vec::new();
            for ch in "li2ho2".chars() {
                typed = session.press(ch as u32);
            }
            let table = latest_table(&typed);
            let slot = if display == CandidateDisplayMode::Combined {
                assert_eq!(table.candidates[0], format!("你好 {expected}"));
                assert_eq!(table.candidates[1], expected);
                'w'
            } else {
                assert_eq!(table.candidates[0], expected);
                'q'
            };
            let selected = session.press(slot as u32);
            assert!(
                selected
                    .iter()
                    .any(|e| matches!(e, Emit::Preedit { text, .. } if text == expected)),
                "{selected:?}"
            );
            assert!(session.press(RETURN).contains(&commit(expected)));
        }
    }
}

#[test]
fn sentence_space_matches_ithuan_selection_and_final_commit() {
    use taigi_desktop_core::settings::{CandidateDisplayMode, InputMode};
    let _serial = serial();
    for mode in [InputMode::Poj, InputMode::Tl] {
        for auto_space in [false, true] {
            let mut session = Session::new(auto_space, false);
            sentence_settings(&session, mode);
            SettingsFileStore::new(&session._directory.path().join("config"))
                .update(|document| {
                    document.set_choice(
                        &keys::CANDIDATE_DISPLAY_MODE,
                        CandidateDisplayMode::Combined,
                    )
                })
                .unwrap();
            for ch in "ho2".chars() {
                session.press(ch as u32);
            }
            let whole = session.press(SPACE);
            assert!(whole.contains(&commit("好")), "{whole:?}");
            assert!(!whole.contains(&commit(" ")), "{whole:?}");

            let mut typed = Vec::new();
            for ch in "tak8keho2".chars() {
                typed = session.press(ch as u32);
            }
            assert!(latest_table(&typed).candidates[2].starts_with("逐家 "));
            session.press(0xff54); // Down: whole POJ.
            session.press(0xff54); // Down: shorter Hanji combination.
            let partial = session.press(SPACE);
            assert!(
                !partial.iter().any(|e| matches!(e, Emit::Commit(_))),
                "{partial:?}"
            );
            assert!(
                partial
                    .iter()
                    .any(|e| matches!(e, Emit::Preedit { text, .. } if text.starts_with("逐家"))),
                "{partial:?}"
            );
            assert!(latest_table(&partial).candidates[0].starts_with("好 "));
            assert!(session.press(SPACE).contains(&commit("逐家好")));

            for ch in "ho2".chars() {
                session.press(ch as u32);
            }
            session.press(0xff54); // Highlight POJ without selecting it.
            let roman = session.press(SPACE);
            assert!(roman.contains(&commit("hó")), "{roman:?}");
            assert_eq!(roman.contains(&commit(" ")), auto_space, "{roman:?}");

            for ch in "ho2".chars() {
                session.press(ch as u32);
            }
            session.press('q' as u32); // A letter pick stays editable.
            let already_selected = session.press(SPACE);
            assert!(
                already_selected.contains(&commit("好")),
                "{already_selected:?}"
            );
            assert!(!session.press_with(SPACE, 0).0);
        }
    }
}

#[test]
fn sentence_selects_bo_then_kah_then_i_from_bokahi() {
    use taigi_desktop_core::settings::{CandidateDisplayMode, InputMode};
    let _serial = serial();
    let mut session = Session::new(false, false);
    sentence_settings(&session, InputMode::Poj);
    SettingsFileStore::new(&session._directory.path().join("config"))
        .update(|document| {
            document.set_choice(
                &keys::CANDIDATE_DISPLAY_MODE,
                CandidateDisplayMode::Combined,
            )
        })
        .unwrap();
    let mut typed = Vec::new();
    for ch in "bokahi".chars() {
        typed = session.press(ch as u32);
    }
    let bo = choose_hanji_with_letter(&mut session, latest_table(&typed), "無");
    assert!(!bo.iter().any(|e| matches!(e, Emit::Commit(_))));
    let kah = choose_hanji_with_letter(&mut session, latest_table(&bo), "佮");
    let i = choose_hanji_with_letter(&mut session, latest_table(&kah), "意");
    assert!(
        i.iter()
            .any(|e| matches!(e, Emit::Preedit { text, .. } if text == "無佮意")),
        "{i:?}"
    );
    assert!(session.press(SPACE).contains(&commit("無佮意")));
}

#[test]
fn sentence_in_pages_stay_in_the_exact_reading_and_inn_remains_available() {
    use taigi_desktop_core::settings::{CandidateDisplayMode, InputMode};
    let _serial = serial();
    let mut session = Session::new(false, false);
    sentence_settings(&session, InputMode::Poj);
    SettingsFileStore::new(&session._directory.path().join("config"))
        .update(|document| {
            document.set_choice(
                &keys::CANDIDATE_DISPLAY_MODE,
                CandidateDisplayMode::Combined,
            )
        })
        .unwrap();
    session.press('i' as u32);
    let typed = session.press('n' as u32);
    let table = latest_table(&typed);
    assert!(table.candidates.iter().any(|row| row.starts_with("因 ")));
    assert!(table.candidates.iter().all(|row| !row.contains('ⁿ')));
    for _ in 0..table.candidates.len().div_ceil(6) + 2 {
        let paged = latest_table(&session.press(0xff56));
        assert_eq!(paged.candidates, table.candidates);
        assert!((paged.cursor as usize) < paged.candidates.len());
    }
    session.press(ESCAPE);
    for ch in "inn".chars() {
        session.press(ch as u32);
    }
    assert!(session
        .state
        .candidates
        .cells()
        .iter()
        .any(|row| row.text.contains('ⁿ')
            || row
                .annotation
                .as_deref()
                .is_some_and(|reading| reading.contains('ⁿ'))));
}

#[test]
fn sentence_backtick_switches_candidate_output_and_removes_standalone_roman_duplicates() {
    use taigi_desktop_core::settings::{CandidateDisplayMode, InputMode, SyllableSeparator};
    let _serial = serial();
    for mode in [InputMode::Poj, InputMode::Tl] {
        for accept in ['q' as u32, SPACE, RETURN] {
            let mut session = Session::new(false, false);
            sentence_settings(&session, mode);
            SettingsFileStore::new(&session._directory.path().join("config"))
                .update(|document| {
                    document.set_choice(
                        &keys::CANDIDATE_DISPLAY_MODE,
                        CandidateDisplayMode::Combined,
                    );
                    document.set_choice(&keys::SYLLABLE_SEPARATOR, SyllableSeparator::Space);
                    document.set_bool(&keys::IS_HANJI_FIRST, true);
                })
                .unwrap();
            let mut typed = Vec::new();
            for ch in "li2ho2".chars() {
                typed = session.press(ch as u32);
            }
            let hanlo = latest_table(&typed);
            assert!(hanlo.candidates[0].ends_with("lí hó"), "{hanlo:?}");
            assert!(hanlo.candidates.iter().any(|row| row == "lí hó"));
            // Changing from a late page must also reset the selection for
            // the shorter POJ-first menu, without accepting any text.
            for _ in 0..hanlo.candidates.len().div_ceil(6) {
                session.press(0xff56);
            }
            let switched = session.press('`' as u32);
            assert!(!switched.iter().any(|emit| matches!(emit, Emit::Commit(_))));
            let poj = latest_table(&switched);
            assert!(poj.candidates[0].starts_with("lí hó "), "{poj:?}");
            assert!(!poj.candidates.iter().any(|row| row == "lí hó"));
            assert_eq!(poj.cursor, 0);
            session.press_with('`' as u32, state::RELEASE);
            // LiveSettings uses mtime and size; separate physical presses
            // so a revision gaining a digit cannot cancel true/false size.
            std::thread::sleep(std::time::Duration::from_millis(2));
            let restored = latest_table(&session.press('`' as u32));
            assert_eq!(restored.candidates, hanlo.candidates);
            session.press_with('`' as u32, state::RELEASE);
            std::thread::sleep(std::time::Duration::from_millis(2));
            let poj = latest_table(&session.press('`' as u32));
            assert!(poj.candidates[0].starts_with("lí hó "));
            let accepted = session.press(accept);
            if accept == 'q' as u32 {
                assert!(accepted
                    .iter()
                    .any(|emit| matches!(emit, Emit::Preedit { text, .. } if text == "lí hó")));
                assert!(session.press(SPACE).contains(&commit("lí hó")));
            } else {
                assert!(accepted.contains(&commit("lí hó")), "{accepted:?}");
            }
        }
    }
}

#[test]
fn sentence_shift_enter_commits_typed_romanization_without_confirming_hanji() {
    use taigi_desktop_core::settings::{CandidateDisplayMode, InputMode, SyllableSeparator};
    let _serial = serial();
    for mode in [InputMode::Poj, InputMode::Tl] {
        for auto_space in [false, true] {
            for selected in [None, Some("好"), Some("你好"), Some("你")] {
                let mut session = Session::new(auto_space, false);
                sentence_settings(&session, mode);
                SettingsFileStore::new(&session._directory.path().join("config"))
                    .update(|document| {
                        document.set_choice(
                            &keys::CANDIDATE_DISPLAY_MODE,
                            CandidateDisplayMode::Combined,
                        );
                        document.set_choice(&keys::SYLLABLE_SEPARATOR, SyllableSeparator::Space);
                        document.set_bool(&keys::IS_HANJI_FIRST, true);
                    })
                    .unwrap();
                let reading = if matches!(selected, Some("你好" | "你")) {
                    "li2-ho2"
                } else {
                    "ho2"
                };
                let expected = if reading == "ho2" { "hó" } else { "lí-hó" };
                let mut typed = Vec::new();
                for ch in reading.chars() {
                    typed = session.press(ch as u32);
                }
                if let Some(hanji) = selected {
                    let table = latest_table(&typed);
                    let picked = choose_hanji_with_letter(&mut session, table, hanji);
                    assert!(!picked.iter().any(|emit| matches!(emit, Emit::Commit(_))));
                }
                let (handled, emits) = session.press_with(RETURN, state::SHIFT);
                assert!(handled);
                let commits: Vec<_> = emits
                    .iter()
                    .filter_map(|emit| match emit {
                        Emit::Commit(text) => Some(text.as_str()),
                        _ => None,
                    })
                    .collect();
                let mut wanted = vec![expected];
                if auto_space {
                    wanted.push(" ");
                }
                assert_eq!(commits, wanted, "{mode:?} selected={selected:?}: {emits:?}");
                assert!(session.state.candidates.is_empty());
            }
        }
    }
}

#[test]
fn sentence_mode_keeps_roman_candidates_and_shift_script_selection() {
    use taigi_desktop_core::settings::{CandidateDisplayMode, InputMode};
    let _serial = serial();
    for (display, hanji_first, key, modifiers) in [
        (CandidateDisplayMode::Combined, true, 'w', 0),
        (CandidateDisplayMode::RomanOnly, true, 'q', 0),
        (CandidateDisplayMode::SideBySide, false, 'q', 0),
        (CandidateDisplayMode::SideBySide, true, 'Q', state::SHIFT),
    ] {
        let mut session = Session::new(false, false);
        sentence_settings(&session, InputMode::Poj);
        SettingsFileStore::new(&session._directory.path().join("config"))
            .update(|document| {
                document.set_choice(&keys::CANDIDATE_DISPLAY_MODE, display);
                document.set_bool(&keys::IS_HANJI_FIRST, hanji_first);
            })
            .unwrap();
        for ch in "ho2".chars() {
            session.press(ch as u32);
        }
        let (handled, selected) = session.press_with(key as u32, modifiers);
        assert!(handled);
        assert!(
            selected
                .iter()
                .any(|e| matches!(e, Emit::Preedit { text, .. } if text == "hó")),
            "{display:?}: {selected:?}"
        );
        assert!(!selected.iter().any(|e| matches!(e, Emit::Commit(_))));
        assert!(session.press(RETURN).contains(&commit("hó")));
    }
}

#[test]
fn sentence_mode_opens_symbols_before_and_after_candidate_selection() {
    use taigi_desktop_core::settings::{CandidateDisplayMode, InputMode};
    let _serial = serial();
    for selected in [None, Some('q'), Some('w')] {
        let mut session = Session::new(false, false);
        sentence_settings(&session, InputMode::Poj);
        SettingsFileStore::new(&session._directory.path().join("config"))
            .update(|document| {
                document.set_choice(
                    &keys::CANDIDATE_DISPLAY_MODE,
                    CandidateDisplayMode::Combined,
                )
            })
            .unwrap();
        for ch in "ho2".chars() {
            session.press(ch as u32);
        }
        if let Some(letter) = selected {
            session.press(letter as u32);
        }
        let (_, opened) = session.press_with(COMMA, state::CONTROL | state::MOD1);
        let expected = if selected == Some('w') { "hó" } else { "好" };
        assert_eq!(
            opened
                .iter()
                .filter(|emit| matches!(emit, Emit::Commit(_)))
                .count(),
            1
        );
        assert!(opened.contains(&commit(expected)), "{opened:?}");
        let table = latest_table(&opened);
        assert_eq!(table.page_size, 6);
        assert_eq!(table.labels, ["q", "w", "d", "f", "z", "x"]);
        let next_page_first = table.candidates[6].clone();
        session.press(0xff56);
        assert!(session
            .press('q' as u32)
            .contains(&commit(&next_page_first)));
        assert!(session.state.symbol_picker.is_none());
    }
}
