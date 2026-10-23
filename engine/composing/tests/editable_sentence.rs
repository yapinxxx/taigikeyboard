//! IThuan-style selected combinations reopen their reading, not a Hanji cell.
use composing::{api::CaretDirection, CommitScript, Engine, Intent, Phase};
use protos::engine::{effect::Kind, AppConfig};

fn pick(
    engine: &mut Engine,
    config: &AppConfig,
    hanji: &str,
    tl: &str,
    bytes: usize,
    syllables: u8,
) {
    let response = engine.apply(
        Intent::CommitContinuous {
            canonical_text: hanji.into(),
            association_tl: tl.into(),
            hanji: Some(hanji.into()),
            consumed_bytes: bytes,
            syllable_count: syllables,
            script: Some(CommitScript::Hanji),
            roman: tl.into(),
        },
        config,
    );
    assert!(response.is_composing);
    assert!(!response
        .effect
        .iter()
        .any(|e| matches!(e.kind, Some(Kind::CommitTextReplacingPreedit(_)))));
}

#[test]
fn selected_phrase_reopens_as_a_reading_with_candidates_bounded_by_the_caret() {
    for mode in ["tl", "poj"] {
        let config = AppConfig {
            input_mode: mode.into(),
            editable_sentence_enabled: true,
            is_hanji_first: true,
            ..Default::default()
        };
        let mut engine = Engine::new();
        engine.apply(
            Intent::Start {
                text: "tak8keho2".into(),
            },
            &config,
        );
        pick(&mut engine, &config, "逐家好", "ta̍k-ke-hó", 9, 3);
        let opened = engine.apply(
            Intent::MoveCaret {
                direction: Some(CaretDirection::Left),
            },
            &config,
        );
        assert_eq!(opened.preedit.unwrap().raw_input, "tak8keho2");
        assert_eq!(engine.pending_snapshot(&config).listed_raw, "tak8keho");
        for _ in 0..2 {
            engine.apply(
                Intent::MoveCaret {
                    direction: Some(CaretDirection::Left),
                },
                &config,
            );
        }
        assert_eq!(engine.pending_snapshot(&config).listed_raw, "tak8ke");
        // Select a shorter combination, then select the next reading.
        pick(&mut engine, &config, "逐", "ta̍k", 4, 1);
        assert_eq!(engine.pending_snapshot(&config).listed_raw, "ke");
        pick(&mut engine, &config, "雞", "ke", 2, 1);
        let Phase::Continuous { raw, caret, .. } = engine.snapshot_state().phase else {
            panic!("sentence must stay editable")
        };
        assert_eq!(raw, "ho2");
        assert_eq!(caret, 0);
        engine.apply(
            Intent::MoveCaret {
                direction: Some(CaretDirection::End),
            },
            &config,
        );
        pick(&mut engine, &config, "好", "hó", 3, 1);
        let committed = engine.apply(Intent::CommitAsShown, &config);
        assert!(committed.effect.iter().any(
            |e| matches!(&e.kind,Some(Kind::CommitTextReplacingPreedit(t)) if t.text=="逐雞好")
        ));
    }
}

#[test]
fn shorter_combination_keeps_the_remaining_reading_and_reset_cancels_everything() {
    let config = AppConfig {
        input_mode: "poj".into(),
        editable_sentence_enabled: true,
        is_hanji_first: true,
        ..Default::default()
    };
    let mut engine = Engine::new();
    engine.apply(
        Intent::Start {
            text: "tak8keho2".into(),
        },
        &config,
    );
    pick(&mut engine, &config, "逐家", "ta̍k-ke", 6, 2);
    assert_eq!(engine.pending_snapshot(&config).listed_raw, "ho2");
    pick(&mut engine, &config, "好", "hó", 3, 1);
    let cancelled = engine.apply(Intent::Reset, &config);
    assert!(!cancelled.is_composing);
    assert!(!cancelled
        .effect
        .iter()
        .any(|e| matches!(e.kind, Some(Kind::CommitTextReplacingPreedit(_)))));
}
