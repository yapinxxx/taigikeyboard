//! Explicit sentence selection keeps complete alternative splits and excludes
//! predictive spelling extensions once the typed reading is complete.
use crate::common::{config, fetch_at_pos_response, production_lexicon_ready, Fetch};
use phonetics::KeyFamily;
use protos::engine::CandidateMessage;

fn candidates(raw: &str, mode: &str, sentence: bool) -> Vec<CandidateMessage> {
    let mut settings = config(mode);
    settings.editable_sentence_enabled = sentence;
    fetch_at_pos_response(
        &settings,
        raw,
        Fetch {
            literal_roman_candidate_disabled: true,
            ..Default::default()
        },
    )
    .continuous
    .expect("candidate carrier")
    .candidates
}

#[test]
fn sentence_keeps_bo_before_kahi_without_splitting_a_closed_bok_syllable() {
    assert!(production_lexicon_ready(), "production lexicon required");
    for mode in ["poj", "tl"] {
        let rows = candidates("bokahi", mode, true);
        assert!(
            rows.iter().any(|row| row.hanji.as_deref() == Some("無")
                && row.canonical_tl == "bô"
                && row.consumed_span_end == 2),
            "{mode}: {rows:?}"
        );
        // The longer bok is still offered. Only the display suppression changes;
        // the underlying syllable inventory/lattice remains authoritative.
        assert!(rows.iter().any(|row| row.consumed_span_end == 3
            && KeyFamily::Poj.toneless_face(&row.canonical_tl) == "bok"));
        for raw in ["bok", "bok8"] {
            let closed = candidates(raw, mode, true);
            assert!(
                !closed
                    .iter()
                    .any(|row| row.consumed_span_end == 2 && row.hanji.as_deref() == Some("無")),
                "{mode} {raw}: {closed:?}"
            );
        }
        let original = candidates("bokahi", mode, false);
        assert!(!original
            .iter()
            .any(|row| row.hanji.as_deref() == Some("無") && row.consumed_span_end == 2));
    }
}

#[test]
fn complete_in_and_inn_sentence_queries_do_not_expand_into_other_spellings() {
    assert!(production_lexicon_ready(), "production lexicon required");
    for mode in ["poj", "tl"] {
        let family = if mode == "poj" {
            KeyFamily::Poj
        } else {
            KeyFamily::Tl
        };
        for raw in ["in", "inn"] {
            let rows = candidates(raw, mode, true);
            assert!(!rows.is_empty());
            assert!(
                rows.iter()
                    .all(|row| row.consumed_span_end as usize == raw.len()
                        && family.toneless_face(&row.canonical_tl) == raw),
                "{mode} {raw}: {rows:?}"
            );
        }
        let rows = candidates("in", mode, true);
        for reading in ["in", "ìn", "ín"] {
            assert!(
                rows.iter().any(|row| row.canonical_tl == reading),
                "missing {mode} {reading}"
            );
        }
    }
}

#[test]
fn incomplete_sentence_input_and_original_predictive_mode_keep_completions() {
    assert!(production_lexicon_ready(), "production lexicon required");
    let incomplete = candidates("b", "poj", true);
    assert!(incomplete.iter().any(|row| row.canonical_tl == "bô"));
    let abbreviation = candidates("ss", "poj", true);
    // Upstream 3.8.0 ranks a different top-30 abbreviation list. Compare
    // the public query against its normal-mode result rather than requiring
    // the old dictionary's specific 鎖匙 row to survive that result cap.
    let original_abbreviation = candidates("ss", "poj", false);
    assert!(!original_abbreviation.is_empty());
    assert_eq!(abbreviation, original_abbreviation);
    assert!(
        abbreviation
            .iter()
            .any(|row| row.syllable_count > 1 && row.consumed_span_end == 2),
        "{abbreviation:?}"
    );
    let original = candidates("in", "poj", false);
    assert!(original
        .iter()
        .any(|row| KeyFamily::Poj.toneless_face(&row.canonical_tl) == "inn"));
}

#[test]
fn a_tone_closed_syllable_without_a_word_is_offered_whole_as_romanization() {
    assert!(production_lexicon_ready(), "production lexicon required");
    // IThuan reads `boe9` as the one syllable bŏe; a 無 for its `bo` would
    // let Space write 無ĕ.
    for (raw, mode, poj) in [
        ("boe9", "poj", Some("bŏe")),
        ("koe9", "poj", Some("kŏe")),
        ("bue9", "tl", None),
    ] {
        let rows = candidates(raw, mode, true);
        assert_eq!(rows.len(), 1, "{mode} {raw}: {rows:?}");
        let row = &rows[0];
        assert_eq!(row.hanji, None, "{mode} {raw}: {rows:?}");
        assert_eq!(
            row.consumed_span_end as usize,
            raw.len(),
            "{mode} {raw}: {rows:?}"
        );
        if let Some(poj) = poj {
            assert_eq!(row.roman, poj);
        }
    }
    // Outside an editable sentence the list is the original one.
    assert!(candidates("boe9", "poj", false)
        .iter()
        .any(|row| row.hanji.as_deref() == Some("無")));
}

#[test]
fn candidates_never_cut_a_tone_closed_syllable() {
    assert!(production_lexicon_ready(), "production lexicon required");
    // `li2` ends on a closed boundary and keeps its words; nothing reaches
    // into `boe9`.
    let rows = candidates("li2boe9", "poj", true);
    assert!(rows
        .iter()
        .any(|row| row.hanji.as_deref() == Some("你") && row.consumed_span_end == 3));
    assert!(
        rows.iter()
            .all(|row| matches!(row.consumed_span_end, 3 | 7)),
        "{rows:?}"
    );
    // The leading closed syllable is offered alone; `li2` follows it.
    let rows = candidates("boe9li2", "poj", true);
    assert_eq!(rows.len(), 1, "{rows:?}");
    assert_eq!(
        (rows[0].roman.as_str(), rows[0].consumed_span_end),
        ("bŏe", 4)
    );
}

#[test]
fn a_tone_closed_syllable_with_words_keeps_them() {
    assert!(production_lexicon_ready(), "production lexicon required");
    let rows = candidates("boe7", "poj", true);
    assert_eq!(rows[0].hanji.as_deref(), Some("袂"), "{rows:?}");
    let rows = candidates("goa2beh4khi3", "poj", true);
    assert_eq!(rows[0].hanji.as_deref(), Some("我欲去"), "{rows:?}");
    // Toneless typing has no closed syllable: 無 stays offered in bokahi.
    let rows = candidates("bokahi", "poj", true);
    assert!(rows
        .iter()
        .any(|row| row.hanji.as_deref() == Some("無") && row.consumed_span_end == 2));
}

#[test]
fn next_syllable_separates_an_unmatched_tone_closed_reading_without_committing() {
    use crate::common::commit_text;
    use composing::{Engine, Intent, Phase};

    assert!(production_lexicon_ready(), "production lexicon required");
    let mut settings = config("poj");
    settings.editable_sentence_enabled = true;
    for suffix in ["khi", "khi3"] {
        let mut engine = Engine::new();
        for ch in format!("boe9{suffix}").chars() {
            let response = engine.apply(Intent::Append { ch: ch.to_string() }, &settings);
            assert_eq!(commit_text(&response), None, "typing must stay editable");
        }
        let Phase::Continuous { raw, nailed, .. } = engine.snapshot_state().phase else {
            panic!("composition must remain active");
        };
        assert_eq!(raw, suffix);
        assert_eq!(nailed.len(), 1);
        assert_eq!(nailed[0].raw_text, "boe9");
        assert_eq!(nailed[0].display_text, "bŏe");
        assert!(
            !nailed[0].is_picked,
            "automatic segmentation is not a selection"
        );
        let response = composing::requests::query(
            &Intent::FetchAtPos {
                now_ms: 0,
                enabled_sources_bitmask: u32::MAX,
                literal_roman_candidate_disabled: true,
                user_rows: Default::default(),
                context: Default::default(),
            },
            &engine,
            &settings,
        );
        assert!(response
            .continuous
            .unwrap()
            .candidates
            .iter()
            .any(|row| row.hanji.as_deref() == Some("去")
                && row.consumed_span_end as usize == suffix.len()));
    }
}

#[test]
fn automatic_roman_boundary_keeps_known_words_and_non_sentence_input_together() {
    use composing::{Engine, Intent, Phase};
    assert!(production_lexicon_ready(), "production lexicon required");
    for (raw, sentence) in [
        ("boe7khi3", true),
        ("goa2beh4khi3", true),
        ("bokahi", true),
        ("boe9khi3", false),
    ] {
        let mut settings = config("poj");
        settings.editable_sentence_enabled = sentence;
        let mut engine = Engine::new();
        for ch in raw.chars() {
            engine.apply(Intent::Append { ch: ch.to_string() }, &settings);
        }
        let Phase::Continuous {
            raw: pending,
            nailed,
            ..
        } = engine.snapshot_state().phase
        else {
            panic!("continuous")
        };
        assert_eq!(pending, raw);
        assert!(nailed.is_empty(), "{raw}");
    }
}

#[test]
fn automatic_roman_boundary_preserves_user_dictionary_readings() {
    use composing::{ConversionFrequency, Engine, Intent, Phase, UserRows};
    struct UserWord;
    impl ConversionFrequency for UserWord {
        fn rows_for_words(&self, _: &[String]) -> ranking::FrequencyMap {
            Default::default()
        }
        fn now_ms(&self) -> i64 {
            0
        }
        fn reading_rows(&self, _: &str, _: &protos::engine::AppConfig) -> UserRows {
            UserRows {
                custom: vec![lexicon::CustomEntry {
                    roman: "bŏe".into(),
                    hanji: None,
                }],
                ..Default::default()
            }
        }
    }
    assert!(production_lexicon_ready(), "production lexicon required");
    let mut settings = config("poj");
    settings.editable_sentence_enabled = true;
    let mut engine = Engine::new();
    engine.apply(
        Intent::Start {
            text: "boe9".into(),
        },
        &settings,
    );
    engine.apply_ranked(
        Intent::Append { ch: "k".into() },
        &settings,
        Some(&UserWord),
    );
    let Phase::Continuous { raw, nailed, .. } = engine.snapshot_state().phase else {
        panic!("continuous")
    };
    assert_eq!(raw, "boe9k");
    assert!(nailed.is_empty());
}

#[test]
fn automatic_roman_boundary_can_be_deleted_and_reopened_without_changing_raw_keys() {
    use crate::common::commit_text;
    use composing::{Engine, Intent, Phase};
    assert!(production_lexicon_ready(), "production lexicon required");
    let mut settings = config("poj");
    settings.editable_sentence_enabled = true;
    let mut engine = Engine::new();
    engine.apply(
        Intent::Start {
            text: "boe9".into(),
        },
        &settings,
    );
    engine.apply(Intent::Append { ch: "k".into() }, &settings);
    for _ in 0..2 {
        let response = engine.apply(Intent::DeleteBackward, &settings);
        assert_eq!(commit_text(&response), None);
    }
    let Phase::Continuous { raw, nailed, .. } = engine.snapshot_state().phase else {
        panic!("continuous")
    };
    assert_eq!(raw, "boe9");
    assert!(nailed.is_empty());
}
