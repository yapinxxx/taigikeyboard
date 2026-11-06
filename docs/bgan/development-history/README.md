# Development history and lessons

This handoff brings together the original development sessions, the Obsidian
TaigiKeyboard issue archive, and the current integration records. Recorded
November 6, 2026. It covers development experience, including corrections and
unresolved findings, rather than treating every earlier explanation as proven.

## Reading order

1. [Current progress and test evidence](../PROGRESS.md).
2. [Recent device typing experience](../DEVICE-EXPERIENCE.md).
3. [Remote development setup](../REMOTE-DEVELOPMENT.md).
4. [The 23 historical Obsidian reports](obsidian-export/README.md).
5. [Eight bilingual author-feedback topics](../author-feedback/2026-10-23/README_en-US.md).

## Chronology

| Period | User experience and resulting work | Status and lesson |
| --- | --- | --- |
| Earlier Ubuntu input project | The Obsidian Taiwanese Input on Ubuntu project prioritized usable Taigi input on Ubuntu and explored integrating the IThuan/Rime frontend, with later cross-platform ambitions. | Background motivation, not evidence that the later TaigiKeyboard fork embeds Rime. |
| October 8 | Wanted IThuan-style whole or shorter combination selection, retaining the sentence for changes before Enter. A per-Hanji-cell editor did not match that request and was replaced with reading-based combination reselection. | `tak8keho2` could be selected as 逐家好, reopened, and reselected as 逐雞好. The dictionary and ranking remained TaigiKeyboard's. |
| October 8 | A newly built addon initially had no Hanji candidates. | The engine and dictionary asset formats did not match. The fix installed matching assets with the addon; preserve that compatibility during deployment. |
| October 8 | An early sentence patch prevented POJ output by forcing Hanji-first preferences and disabling alternate-script selection. | Restored saved display preferences and POJ/TL selection. Editable Hanji must not remove Roman output or mixed-script choices. |
| October 8 | Opening the symbol table during composition only selected the first candidate. | The opener assumed selection finished composing. It was changed to finalize the editable sentence before opening a picker that writes outside it. |
| October 8 | Ctrl+Space failed in Discord while IThuan appeared to work. A Snap denial led to a persistent XIM workaround. | The workaround was reverted at the user's request. The user reported recovery after restarting Fcitx5. A denial did not establish the exact TaigiKeyboard-only cause; test service recovery before adding permanent configuration. |
| October 9 | Holding backtick repeatedly toggled punctuation width. | Confirmed repeat-policy defect: the toggle was missing from the once-per-press roster. A held press toggled once after correction; release/repress toggled again. |
| October 9 | Apostrophe and Enter sometimes appeared twice. | Clean injected taps did not reproduce every reported duplicate. Repeat keydowns after a composition commit can reach the application; quick-retap-and-hold was reproduced in QMK/GTK simulation. These are plausible paths, not proof of a hardware fault or a complete explanation of the physical trials. |
| October 9 | Holding the apostrophe position switched away from Taigi. | The dual-role key emitted Left Shift, which matched the configured Fcitx mode-switch shortcut. Separate firmware mappings, host repeat, and IME behavior when diagnosing an event. No firmware was flashed during these diagnoses. |
| October 21 | Wanted POJ readings beside Hanji and spaces instead of syllable hyphens. | Several corrections distinguished whole-candidate formatting, word boundaries, and trailing Auto-Space. Selecting one complete `lí hó` candidate is different from separately selecting syllables or appending a trailing space. |
| October 21–22 | Wanted 無 → 佮 → 意 inside `bokahi` and exact `in` pages without expanding into `inn`. | Candidate-prefix exposure and exact-reading scope were adjusted, while preserving valid `bok`, incomplete `b`, and abbreviation `ss`. Do not infer spelling rules from absent dictionary candidates. |
| October 21–23 | Wanted a persistent Han-lo/POJ-first switch, readable indicators, and no duplicate bare POJ choice. | Candidate annotations, script selection, and indicators were refined. An early six-key layout was later superseded by eight sentence keys in the current integration. |
| October 22 | Wanted easy emoji access without leaving TaigiKeyboard. | Existing Fcitx Unicode search and Quick Phrase were assigned to two Vial layer keys. Super+semicolon avoided a GNOME Super+backtick conflict. These were keyboard mappings, not a new TaigiKeyboard emoji engine. The user confirmed the gestures worked. |
| October 21–23 | Reported narrow-looking space after the POJ raised dot in Threads. | The pasted text retained a space. Application/font rendering was a hypothesis; a confirmed engine defect was not established. |
| October 23 | Shift+Enter selected Hanji instead of outputting typed pronunciation. | The literal action had been routed through candidate confirmation. Routing through CommitAsTyped restored `ho2` → `hó`, including after selection. This repaired a custom regression; official 3.8.0 already supported the basic action. |
| October 23 | Needed a transferable installer and an accurate official/custom comparison. | Earlier binaries depended on machine paths. A deployment archive with matching assets was prepared, then the custom overlay was genuinely rebased onto desktop-3.8.0. Relabeling an older binary was not treated as a rebase. |
| October 31–November 1 | `、` looked different in the Fcitx panel and terminal; some font combinations showed unusual spacing. | The reported character was U+3001. The session distinguished terminal font selection and fallback from input correctness. Source Code Pro plus OpenHuninn fallback was tried; a temporary OpenHuninn-first experiment was rolled back. This is device-specific rendering experience, not a universal font prescription. |
| November 5–6 | Wanted isolated developer, tester, reviewer, debugger, release, and main worktrees; all five personal branches integrated against the original project's newer main. | The workspace was reorganized, branches integrated/rebased, and code-preserving attribution cleanup backed up. Worktrees share Git metadata, not uncommitted files. |
| November 5–6 | Testing exposed a removed typed-commit spacing API after the rebase. | The bridge now uses the engine's commit resolution and spacing verdict. The full engine run retained two failures independently reproduced on the upstream baseline. |
| November 5–6 | Build logs took over the terminal; later `boe9khi3` was difficult to edit into spaced Roman/Hanji output. | Quiet daemon restart and file-based logs improve the test handoff. The unmatched closed reading now separates when the next syllable starts. Automated checks pass; the latest interactive retry and the requested tone-glyph difference remain open. |

## Source and evidence rules

The original October 8, October 9, October 21–23, October 31–November 1, and
November 5–6 session requests and final outcomes were read alongside the
Obsidian notes. Internal approval-review transcripts and unrelated sessions
were excluded. Raw chat logs contain unrelated context and are not published.

The [export manifest](obsidian-export/source-manifest.json) identifies the
23 source reports by relative name and checksum. Both existing language
versions are retained where present. Historical test totals belong to their
recorded revisions, not the current code. The eight author-feedback topics
and replay captures were already tracked and remain in their existing folder.

The Obsidian project overview and artifact index also established why scripts,
patches, captures, and deployment archives lived in the technical workspace
rather than the vault. This repository carries source, documents, and existing
curated captures; private settings, learning databases, machine backups, and
unrelated personal notes remain outside it.

## Practices to carry forward

- Reproduce the user's interaction before choosing an implementation. A
  per-character editor, reading-based reselection, and already-committed
  document editing are different capabilities.
- Preserve Roman output and settings while extending Hanji workflows.
- Compare with the exact upstream version and matching dictionaries. Features
  already in upstream should be described as preserved behavior or extensions.
- Distinguish user-confirmed operation, isolated addon tests, simulated events,
  and unverified explanations. Keep failed and inconclusive trials visible.
- Back up first, verify the actual loaded module, and provide a reversible
  install path. Package updates can be shadowed by per-user addon overrides.
- Keep the terminal available for typing; save long build output to files.
- Never silently exclude baseline failures or call an unbuilt platform tested.
- en-US is the default documentation language. fs-TW means Taiwanese Taigi;
  retain the original Taigi versions and use Traditional Han characters.

No upstream message, issue, or pull request is implied by documenting this work.
