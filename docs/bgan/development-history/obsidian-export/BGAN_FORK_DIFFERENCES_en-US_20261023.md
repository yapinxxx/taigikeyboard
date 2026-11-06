> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# bgan differences from official TaigiKeyboard 3.8.0

The purpose of this comparison is to explain the interaction I wish to use and provide concrete examples for discussion with the upstream author.

**Official baseline:** `desktop-3.8.0`, `69751e58ba0dbe64d3512330e4580eefaff71521`.  
**Running custom build:** `3.8.0-bgan.20261023`, source `b8f00d81841cc69c1f9080ce6edceef7a912b392`, branch `bgan/3.8.0`.

[Overview](BGAN_FORK_OVERVIEW_en-US_20261023.md) · [fs-TW](BGAN_FORK_DIFFERENCES_fs-TW_20261023.md)

| Topic | Official 3.8.0 | bgan request and behavior |
|---|---|---|
| Readings beside Hanji | Separate `好` and `hó` choices in Combined display | Show `好 hó` in one row, retaining a separate selectable `hó` in Han-lo. Selecting Hanji writes only Hanji. |
| Editable selections | Selecting `q` immediately commits `好` | Keep whole or shorter choices editable. Left reopens the underlying reading; later choices can be revised before finalization. Six keys: `q w d f z x`. |
| Space confirmation | Already confirms highlighted `好` for `ho2` + Space | Preserve that behavior and extend it to editable sentences: partial choices stay in preedit; final Space/Enter sends the selected sentence. |
| POJ with spaces | Already offers complete `lí hó` when Syllable Separator = Space | Preserve the complete spaced choice and keep it editable until finalization. Explicit hyphens remain available. Auto-Space independently controls trailing output spacing. |
| Shorter `bo` choice | `bokahi` already offers the whole `無佮意`, but standalone `無` is absent in the captured menu | Also allow manual `無 → 佮 → 意` selection, while retaining valid `bok` choices. This changes candidate-prefix exposure rather than spelling rules. |
| Exact `in` search | Later pages include nasal `iⁿ` and predictive expansions | Keep complete `in` queries within that reading, retaining tone variants. `inn` remains separate; incomplete `b` and abbreviation `ss` still work. Captured counts were 83 official cells and 56 bgan cells; counts are not fixed requirements. |
| Han-lo / full POJ switch | Backtick does not reorder the Combined candidates in the replay | Han-lo: `q 你好 lí hó`, `w lí hó`. Backtick: `q lí hó 你好` writes only `lí hó`; omit the duplicate bare POJ row. Toggle persists. Indicators: `台` / `POJ`. |
| Shift+Enter | Already sends `hó` for `ho2` | Preserve that behavior, including after selecting Hanji. An earlier custom regression is repaired; this is not an official 3.8.0 bug. |
| Symbol picker | Existing symbol picker | Finalize an editable selection before opening a picker that writes outside the composition; cancellation keeps selection behavior consistent. |

## Scope of the overlay

The optional `editableSentenceEnabled` setting defaults off. The shared engine owns editable segments, reading-caret boundaries and exact-reading queries. Desktop core owns candidate presentation and key intents. Linux core keeps six-key labels, page navigation and menu selection consistent. Distinct Hanji/canonical-reading pairs remain distinct.

Key source areas:

- `engine/composing/`: segment acceptance, caret editing, prefix policy, exact-reading query scope and commit behavior.
- `engine/protos/proto/envelope.proto`: the optional editable-sentence flag.
- `desktop/crates/taigi-desktop-core/`: settings, annotations, POJ-first presentation, confirmation intents and mode indicators.
- `linux/crates/taigi-linux-core/`: page size, selection and presentation wiring.
- `linux/data/icons/`: 台/POJ indicator assets.
- `tools/desktop/`: feature checker, build/verification helper and upstream rebase helper.

All four dictionaries are byte-identical to official 3.8.0. Upstream's normalized preedit display is retained: `tak8keho2` renders as `ta̍kkehó`; its underlying reading remains editable. Existing phonetic inventories and primitive syllabification rules are preserved.

Emoji access through existing Fcitx functionality and Vial keyboard mappings is separate from this source overlay. The Threads dot/spacing observation is not treated as a confirmed keyboard defect. Videos remain deferred until the wording is reviewed.

## Verification

- All **12 feature groups** passed against the built addon and installed dictionary prefix, including reselection to `逐雞好`, `bo → kah → i`, exact-reading paging, POJ mode and Shift+Enter.
- **486 desktop/storage** and **70 Linux core/platform/FFI** tests passed.
- The remaining engine suite passed **1,563 tests**, with **five existing ignored tests** and **two explicitly excluded SQLite learning failures**.
- Both excluded failures were reproduced at the same assertions on **unmodified official desktop-3.8.0** in the local build environment: `literal_picks_reuse_existing_keys_and_keep_tone_boundaries_distinct` and `the_engine_writes_what_the_platforms_wrote`.
- Engine, desktop core/storage and Linux core/platform/FFI Clippy passed; Rust formatting passed.
- Three isolated Fcitx5 integration scripts passed for candidate selection, editing, Space, mode switching, symbol picker, Enter and Shift+Enter.
- Full GTK/update workspace gates require development libraries unavailable in the local build environment. The settings application comes from the official 3.8.0 package. Other platform shells and installation on a second physical device are not validated.

The maintenance branch additionally formats the Python helpers, fetches official updates through an explicit `upstream` remote, adds the bgan CI gate and publishes these notes. Those additions are separate from the exact running-version source snapshot. CI results are reported by GitHub Actions; a workflow being present is not evidence of a completed passing run.

## Reproducing the comparison

Check out `desktop-3.8.0` for official behavior and `bgan/3.8.0` for the running custom behavior, using matching 3.8.0 assets and isolated configuration/data directories. Use POJ, Combined display, Hanji first, Syllable Separator = Space, literal candidate off and Auto-Space off; enable editable sentences only in bgan.

`python3 tools/desktop/check-bgan-features.py <built-addon.so> --data-dir assets` exercises the contract without editing live settings. `python3 tools/desktop/verify-bgan.py` builds and runs the verification gates. On 3.8.0 only, explicitly acknowledging the documented upstream SQLite limits is available through `--allow-known-upstream-sqlite-tests`.

Future updates use `python3 tools/desktop/rebase-bgan.py desktop-<version>`. The new worktree must pass verification before it is deployed. The full overlay can be reviewed with `git diff desktop-3.8.0..bgan/3.8.0`; this patch is against official 3.8.0 rather than the earlier 3.6.13 source.
