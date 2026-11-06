> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# TaigiKeyboard feedback: original installation versus local editable-sentence build

**Date:** October 21, 2026  
**Language:** en-US · [fs-TW / 台文 version](AUTHOR_COMPARISON_fs-TW_20261021.md)  
**Platform:** Linux, Fcitx5

## Feedback for the author

I would like an optional sentence-editing workflow inspired by 意傳 (ì-thoân):
select a whole phrase or a shorter combination, keep letter-selected text
editable, revise its underlying reading, and use Space to accept the highlighted
candidate. I also want the POJ reading beside Hanji candidates in Combined mode
and a complete POJ candidate such as `lí hó` that I can select once.

I have tried a local implementation and am keeping this version for daily use.
It is a prototype for discussing the interaction, with the remaining issues below;
it is not a request to replace the existing default workflow for everyone.

## 1. Versions and comparison method

| Item | Version or identity |
|---|---|
| Original installed package | Unmodified system package **3.6.10** |
| Source baseline for local changes | **3.6.13**, commit [`3bdd7ccb2e953f002e536e89a59b3a69eae74018`](https://github.com/taigikeyboard/taigikeyboard/commit/3bdd7ccb2e953f002e536e89a59b3a69eae74018), dated October 7, 2026 |
| Current local build | That source baseline plus the attached local patch; installed as a per-user Fcitx5 addon |
| Reference interaction | Installed IThuan `pojhan` schema, exercised through the installed librime library in an isolated temporary session |
| Current mode | POJ, Combined candidates, editable sentence enabled, syllable separator = Space, auto-space disabled |

The **runtime before/after results below compare installed 3.6.10 with the current
local build**. Separately, the source patch compares the local changes with the
pinned 3.6.13 source. These are different comparisons: differences between the
two installed binaries cannot all be attributed to this patch.

Both binaries were replayed through the same C ABI with temporary configuration
and learning directories. Each used its own matching installed dictionaries.
The controlled settings used POJ, Combined, Hanji-first, Show Typed Text First off,
Auto-Space off, and the legacy No Hyphens switch on. Only the local build recognizes
the new editable-sentence flag. No live input-method state or real learning
database was changed by that comparison. Dictionary/ranking parity was not tested.

## 2. Observed behavior comparison

| Action or feature | Original installed 3.6.10 | Current local version |
|---|---|---|
| Type `ho2` in Combined | First cells are `好` and `hó` | First cells are `好 hó` and `hó`: Hanji has a reading annotation |
| Press `q` for the complete `好` candidate | Immediately commits `好` to the application | Selects `好` inside the editable composition; no application commit yet |
| Press Left after that `q` selection | Passed to the application; the original selection has already committed | Reopens the reading and candidate list inside the IME |
| Press Space after typing `ho2`, with the first Hanji cell highlighted | Outputs the **other script**, `hó` | Accepts the **highlighted script**, `好`, and commits because that completes the selection |
| Highlight POJ `hó`, then Space | Space normally requests the other script | Accepts `hó`; finalizes when no candidate menu remains |
| Select a shorter combination with Space | Uses the original alternate-script policy | Accepts the highlighted combination and keeps composing while another candidate menu remains |
| Standard candidate page | Nine positions: `q w d f z x v y ;` | Six positions: `q w d f z x`, including consistent paging and click indices |
| No Hyphens: type `li2ho2` | Offers `你好` and `líhó` | Offers `你好 lí hó` and the complete `lí hó` candidate |
| Select that POJ candidate with `w` | Commits `líhó` | Keeps `lí hó` editable; Space or Enter then commits it |
| **Shift + Enter after `ho2`** | Commits typed pronunciation `hó` | **Currently commits `好`; known local regression** |

The exact recorded outputs are attached as original traces (original local artifact: `issues/taigikeyboard-20261008/logs/validation/author-original-3.6.10-20261021.json`)
and current traces (original local artifact: `issues/taigikeyboard-20261008/logs/validation/author-current-20261021.json`).

### Features already present upstream

POJ/TL input, Hanji/romanized candidate choices, alternate-script selection,
Hanji–Romanization Pairing, and auto-space were already TaigiKeyboard features.
The pinned 3.6.13 source also already provides **Syllable Separator = Space**.
The desired `lí hó` formatting should therefore be considered a preference using
that existing capability, rather than a new phonetic rule.

The local legacy No Hyphens migration was changed from `None` to `Space` to match
this preference. That changes the legacy switch's semantics and should not be
copied upstream indiscriminately. An explicit separator choice is preferable.

## 3. Reproducible current workflow

Use POJ, Combined, editable sentence enabled, and Syllable Separator = Space.
These examples assume Show Typed Text First is off.

1. **Direct POJ selection:** type `li2ho2`. Select the complete `lí hó` candidate
   with `w`, then press Space or Enter. The application receives `lí hó`.
   There is no need to select `lí` and `hó` separately.
2. **Highlighted candidate:** type `ho2`, then Space. The application receives
   `好`. To output `hó`, type `ho2`, press Down to highlight the POJ cell, then Space.
3. **Partial phrase:** type `tak8keho2`. In Combined, Down twice highlights
   the shorter Hanji combination `逐家`. Space accepts it and leaves `ho2`
   editable. Another Space accepts `好` and commits `逐家好`.
4. **Revise before committing:** type `tak8keho2`, then `q` to select `逐家好`.
   Left reopens its reading. Moving left three times reaches the prefix
   `tak8ke`; select `逐`, then `雞` for `ke`. End returns to the remaining
   `ho2`; choose `好` and finalize to produce `逐雞好`.

Letter selection stays editable. **Space accepts the highlighted candidate and
finalizes when its menu closes.** Enter accepts the remaining active choices
and finalizes. Space outside composition is an ordinary application space.
If Auto-Space is enabled, final romanized output gains a trailing space;
Hanji output does not. That setting is independent of internal syllable formatting.

IThuan's behavior was verified directly, including partial choices and final
Hanji/POJ commits: isolated reference trace (original local artifact: `issues/taigikeyboard-20261008/logs/validation/ithuan-space-reference-20261021.log`).
Its dictionary and ranking remain different; TaigiKeyboard does not embed librime.

## 4. Implementation included in the patch

- Optional `editableSentenceEnabled`, default off, carried through the desktop
  settings and the engine configuration protocol.
- Accepted combinations remain in composition; the reading caret bounds
  candidate fetching and ranking context, retains the suffix, and permits
  reopening and reselection. Longer reading spans precede shorter ones.
- Six-key candidate pages; sentence-mode Left/Right edit the reading rather
  than navigate candidates. Up/Down and paging remain candidate navigation.
- Combined Hanji cells show readings and distinguish different displayed
  Hanji/reading pairs. Separate POJ/TL choices remain selectable.
- Space uses an explicit sentence-confirmation intent. Space and Enter share
  finalization and the actual final script's auto-space verdict.
- Auxiliary repairs: the symbol picker finishes a sentence before opening;
  holding the script-swap/backtick key toggles once per press instead of
  repeatedly. These should be reviewed separately from the new interaction.

The patch also includes composition word-space support from the local work.
The legacy separator migration, word normalization, and held-key repair are
not all gated by the new sentence-mode flag. This is a local prototype, not
a promise of unchanged behavior everywhere when that flag is off.

## 5. Validation and limits

| Check | Evidence |
|---|---|
| Final desktop-core suite | **453 passed** |
| Final Linux core/platform/FFI suites | **64 passed**; total with desktop-core: **517** |
| Clippy | Desktop-core and those three Linux crates passed with warnings denied |
| Installed addon | Combined and Romanization Only; No Hyphens enabled/disabled; direct POJ selection; partial/final Space; auto-space on |
| Running Fcitx5 | Private input-context checks passed for highlighted Hanji/POJ, partial selection, already-selected final output, reading reselection, direct `lí hó`, and symbol picker |
| Original versus current | New isolated C-ABI traces captured from both installed binaries |

Earlier composing-engine coverage recorded 655 passing tests with two optional
tests ignored; two targeted sentence tests were rerun after the final engine
adjustment. That is supplemental history, not a fresh full-engine run for this
report. Full cross-platform release validation has not been completed.

The broad Linux gate previously stopped on an unavailable GTK settings dependency
download. The GTK settings application, other platform shells, and a full OS
reboot were not validated. An earlier broader run also reproduced a dispatch
user-data test failure on untouched upstream; it is not evidence of a new defect
caused by this feature. Performance and memory improvements are not claimed.

## 6. Items to resolve before upstreaming

1. **Restore a distinct literal-commit action.** Shift+Enter currently shares
   Enter's candidate-confirmation path in sentence mode. The attached runtime
   comparison reproduces the regression. It remains unfixed in this daily-use build.
2. **Align the settings UI and shortcut descriptions.** The installed settings
   application is from 3.6.10; the addon is a newer local build. Sentence-mode
   Space overrides the original “other script” binding, and enabling sentence
   mode currently requires its settings key rather than a finished UI control.
3. **Keep separator preferences explicit.** Prefer exposing the existing Space
   separator over redefining the legacy No Hyphens migration for every user.
4. **Review shared-engine and platform behavior.** Build and validate each shell,
   review learning/reading-boundary behavior and auxiliary changes, and resolve
   the literal shortcut before treating the patch as a release-ready contribution.

The requested upstream outcome is an **opt-in editable sentence interaction**
and clear candidate readings, while retaining the existing workflow as an option.

## Attachments

- Source patch against the pinned 3.6.13 baseline (original local artifact: `issues/taigikeyboard-20261008/artifacts/taigikeyboard-local-changes-20261021.patch`)
- Runtime comparison script (original local artifact: `issues/taigikeyboard-20261008/scripts/compare-taigi-original-current.py`)
- Installed IThuan reference replay (original local artifact: `issues/taigikeyboard-20261008/scripts/check-ithuan-space-reference.py`)
- Final running-service evidence (original local artifact: `issues/taigikeyboard-20261008/logs/validation/ithuan-space-running-fcitx-20261021.log`)
- Validation manifest (original local artifact: `issues/taigikeyboard-20261008/artifacts/author-feedback-validation-20261021.json`)

The source changes remain local and uncommitted. No report, issue, or message has
been sent to the author. The attachment bundle contains reports, source changes,
and test evidence; it excludes binaries, dictionaries, and personal learning data.
