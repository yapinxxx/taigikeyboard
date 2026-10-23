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
                rows.iter().all(|row| row.consumed_span_start == 0
                    && row.consumed_span_end as usize == raw.len()
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
    assert!(abbreviation
        .iter()
        .any(|row| row.hanji.as_deref() == Some("鎖匙")));
    let original = candidates("in", "poj", false);
    assert!(original
        .iter()
        .any(|row| KeyFamily::Poj.toneless_face(&row.canonical_tl) == "inn"));
}
