# Personal features versus current upstream

Reviewed November 11, 2026. Official source: **`b75f24409f42d77200915f0b7ee4c577805a47e2`**
(November 10). Personal integration reviewed: **`dab3ced0aa14c831a9d01c0282d7f6d096189549`**.
The personal branch already contained upstream through `6fe46e5d4`; the newer
source contains another 167 commits. Commit counts are not feature counts.

This is a source comparison. It does not replace a fresh built-addon replay or
interactive application testing. Older official 3.8.0 comparisons remain
historical snapshots; they must not be read as claims about today's upstream.

## Decisions

| Personal requirement | Current upstream evidence | Maintenance decision |
| --- | --- | --- |
| One complete POJ candidate with syllable spaces | `AppConfig::rendered_syllable_joiner` supports Space/Hyphen/None; this basic capability already existed in the official 3.8.0 replay. | **Already provided.** Use the setting and keep a regression; do not implement another hyphen-to-space conversion. |
| Hanji readings and selectable Roman output | Pairing and Romanization Only already exist. Upstream Combined presentation emits separate Hanji/Roman cells without the personal inline annotation. [Presentation source][presentation]. | **Partly provided.** Evaluate Pairing first; retain only the requested Combined-row annotation and script-order differences if still needed. |
| Final selected combination remains editable until explicit confirmation | Upstream `nail_segment` finalizes when the pending tail becomes empty. The personal setting retains the selection. [Selection transition][selection]. | **Still distinct.** Keep the opt-in retained-sentence workflow; do not describe all caret editing as absent upstream. |
| Eight sentence keys aligned with eight visible cells | Upstream already has slot-key selection and configurable presentation. The personal sentence page/key policy is an additional contract. | **Keep only the eight-cell sentence policy.** Reuse upstream paging/key machinery. |
| Space breaks words, Enter sends; completion suggestions require explicit selection | Upstream has candidate confirmation, alternate-script selection, and commit intents, but not the personal `BreakSentenceWord`/Editable Sentence setting. | **Still distinct.** Port this policy, preserving ordinary upstream input when disabled. |
| `bokahi` allows manual 無 → 佮 → 意 while retaining valid `bok` | The upstream span-local builder still suppresses shorter single-syllable prefixes under its normal guards. The personal `CompleteSentenceSplits` policy is absent. [Prefix filter][prefix]. | **Retain pending replay.** Verify the exact sequence against current dictionaries; do not infer phonetic rules or remove valid longer syllables. |
| Complete `in` queries stay exact while `inn`, incomplete `b`, and abbreviation `ss` remain searchable | Upstream has prefix, abbreviation, and walker search. The personal sentence-specific exact-reading filter is not present in its query path. [Candidate query][query]. | **Retain pending replay.** Source inspection establishes different filtering, not today's complete menu contents. |
| Persistent Han-lo/POJ-first Combined ordering, no redundant bare POJ row | Upstream already deduplicates displayed Combined cells and supports output/display settings. The personal reordering/annotation policy still differs. [Presentation source][presentation]. | **Partly provided.** Keep only required ordering/annotation behavior; avoid a second general deduplication layer. |
| Shift+Enter pronunciation, including after a retained Hanji selection | Upstream has literal commits and `CommitAsTyped`; its explicit executor branch is TPS-specific. The official 3.8.0 replay already supported basic `ho2` → `hó`. [Commit executor][commit]. | **Preserve/extend.** Do not claim basic pronunciation output is missing upstream. Retained-sentence behavior still needs its adapter and regression. |
| Symbol picker during an editable sentence | Upstream already supplies the symbol picker. The custom sentence lifetime requires a compatible finalize-before-picker handoff. | **Integration only.** No new symbol engine. |
| Unmatched `boe9` becomes editable Roman text when `k` begins the next syllable | The personal closed-reading/automatic-boundary helpers are absent upstream. Local `a555863a0` tests cover `boe9khi[3]` → `bŏe 去`. | **Still distinct.** Keep the focused fix; current-upstream runtime comparison remains pending. |
| Holding backtick toggles once per press | The personal once-per-press roster includes `ToggleTranslateSwapped`; current upstream's roster lists the guide, symbol picker, and TPS panel only. [Repeat policy][repeat]. | **Difference remains.** Preserve the regression and replay the complete upstream event path before attributing a live defect. |
| TPS behavior remains independent of the personal sentence mode | Upstream has newer TPS/custom-word learning and candidate-window behavior. | **Prefer upstream implementations.** Retain only the sentence-mode opt-out guard and its tests. |
| Emoji access, terminal punctuation appearance, keyboard tap/hold behavior | Fcitx addons, Vial mappings, fonts, and host events are separate from the TaigiKeyboard engine. | **Separate device work.** Do not add duplicate engine features or call unconfirmed rendering/hardware hypotheses fixed. |

## Upstream changes to reuse

Recent source changes include dictionary-source settings carried in AppConfig,
custom/learned words in TPS conversion, editable learning-record capture,
dictionary-management search, revised expandable candidates, and opt-in `c`
affricate / `f` hyphen settings. Web/ChromeOS and Windows Arm64 work also exists;
it is not a reason to expand this Linux-focused personal overlay automatically.

The mobile universal layout now offers both tone-9 marks. That does **not** prove
that the requested desktop numeric `boe9` glyph is resolved. Keep `ǒ` versus `ŏ`
as a separate evidence question; no converter rule was changed in this review.

## Source-integration blocker

A trial merge into the clean personal branch reported **18 conflicting files**
and was aborted cleanly. The working personal code was preserved. In particular:

| AppConfig field | Personal branch | Current official source |
| --- | --- | --- |
| 15 | `editable_sentence_enabled` (bool) | `dictionary_sources` (message) |
| 16 | — | `custom_dictionary_disabled` |
| 17 | — | `c_types_affricate` |
| 18 | — | `f_types_hyphen` |

[Current protocol][protocol] must retain upstream wire identities. Allocate a
free field for the personal setting, regenerate Java and both Swift bindings,
and adapt source-filter/custom-word plumbing together. Do not resolve the
collision by choosing one side or by editing generated bindings manually.
The conflicts also touch candidate keys, query assembly, and shared settings.

The remote historical bgan branches retain earlier versions and independent
upstream merges. They were fetched and inventoried, not blindly merged or
deleted. The canonical active branch is `bgan/integrated-space-word-break`;
the checkout folder remains `developer/bgan-space-word-break`.

Next steps and acceptance gates: [personal roadmap](ROADMAP.md).

[presentation]: https://github.com/taigikeyboard/taigikeyboard/blob/b75f24409f42d77200915f0b7ee4c577805a47e2/desktop/crates/taigi-desktop-core/src/composing/presentation.rs#L36
[selection]: https://github.com/taigikeyboard/taigikeyboard/blob/b75f24409f42d77200915f0b7ee4c577805a47e2/engine/composing/src/transition.rs#L937
[prefix]: https://github.com/taigikeyboard/taigikeyboard/blob/b75f24409f42d77200915f0b7ee4c577805a47e2/engine/composing/src/shadow.rs#L613
[query]: https://github.com/taigikeyboard/taigikeyboard/blob/b75f24409f42d77200915f0b7ee4c577805a47e2/engine/composing/src/requests.rs#L210
[commit]: https://github.com/taigikeyboard/taigikeyboard/blob/b75f24409f42d77200915f0b7ee4c577805a47e2/desktop/crates/taigi-desktop-core/src/composing/intent_executor.rs#L137
[repeat]: https://github.com/taigikeyboard/taigikeyboard/blob/b75f24409f42d77200915f0b7ee4c577805a47e2/desktop/crates/taigi-desktop-core/src/keys/shortcut_actions.rs#L158
[protocol]: https://github.com/taigikeyboard/taigikeyboard/blob/b75f24409f42d77200915f0b7ee4c577805a47e2/engine/protos/proto/envelope.proto#L152
