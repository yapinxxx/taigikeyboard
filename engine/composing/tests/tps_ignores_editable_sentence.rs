//! Editable Sentence is a TL / POJ workflow (`AppConfig::editable_sentence_active`).
//! Desktop sends the stored flag under every input mode, so a user who turned
//! it on in TL / POJ and then switched to TPS still carries it. The TPS
//! composition must answer exactly as it does with the flag off: the final
//! pick commits, a pick past the caret is honoured, Left at the start of the
//! tail follows the conversion rules, and the candidate order is unchanged.
//!
//! The fixture of `tps_hanji_conversion.rs`.

use composing::api::CaretDirection;
use composing::{requests, Engine, Intent};
use protos::engine::composing_request::Method;
use protos::engine::{AppConfig, ComposingResponse, FetchAtPos};
use test_support::engine_install_lock;

use crate::common::{config, config_converting, req};
use crate::tps_hanji_conversion::{install_fixture_with, pick};

enum Step {
    Fetch,
    Move(CaretDirection),
    Pick(&'static str, &'static str, usize),
}

fn run(raw: &str, steps: &[Step], config: &AppConfig) -> Vec<String> {
    let mut engine = Engine::new();
    let start = engine.apply(Intent::Start { text: raw.into() }, config);
    let mut trace = vec![format!("{start:?}")];
    for step in steps {
        let response: ComposingResponse = match step {
            Step::Fetch => requests::handle(
                &req(Method::FetchAtPos(FetchAtPos::default())),
                &mut engine,
                config,
            )
            .expect("FetchAtPos"),
            Step::Move(direction) => engine.apply(
                Intent::MoveCaret {
                    direction: Some(*direction),
                },
                config,
            ),
            Step::Pick(hanji, tl, bytes) => engine.apply(pick(hanji, tl, *bytes), config),
        };
        trace.push(format!("{response:?}"));
        trace.push(format!("{:?}", engine.snapshot_state().phase));
    }
    trace
}

fn assert_parity(raw: &str, steps: &[Step]) {
    for base in [config("tps"), config_converting("tps")] {
        let on = AppConfig {
            editable_sentence_enabled: true,
            ..base.clone()
        };
        assert!(!on.editable_sentence_active());
        let (off, on) = (run(raw, steps, &base), run(raw, steps, &on));
        for (index, (off, on)) in off.iter().zip(&on).enumerate() {
            assert_eq!(off, on, "step {index} differs with Editable Sentence on");
        }
    }
}

// Pick the first word, then the whole remaining tail: the last pick is the
// final commit (transition.rs nail_segment) and its candidates come from the
// same fetch (requests.rs ordering).
#[test]
fn the_final_pick_commits_as_with_the_setting_off() {
    let _lock = engine_install_lock();
    install_fixture_with(&[]);
    assert_parity(
        "ㄒㄧˋㄒㄧ ",
        &[
            Step::Fetch,
            Step::Pick("死", "sí", 8),
            Step::Fetch,
            Step::Pick("詩", "si", 7),
        ],
    );
}

// Move the caret into the tail, then pick a word reaching past it, and step
// Left from the start of the tail to reopen the nailed word.
#[test]
fn caret_moves_and_a_pick_past_the_caret_match_the_setting_off() {
    let _lock = engine_install_lock();
    install_fixture_with(&[]);
    assert_parity(
        "ㄒㄧˋㄒㄧ ",
        &[
            Step::Fetch,
            Step::Pick("死", "sí", 8),
            Step::Move(CaretDirection::Left),
            Step::Fetch,
            Step::Pick("詩", "si", 7),
        ],
    );
    assert_parity(
        "ㄒㄧˋㄒㄧ ",
        &[
            Step::Pick("死", "sí", 8),
            Step::Move(CaretDirection::Left),
            Step::Move(CaretDirection::Left),
            Step::Move(CaretDirection::Left),
            Step::Move(CaretDirection::Right),
            Step::Fetch,
        ],
    );
}
