> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# POJ candidate readings and Space separators — October 21, 2026

Installed and verified in the running Fcitx5 addon.

## Behavior

- With editable sentence mode enabled, Combined candidates show readings beside
  Hanji, for example `台語 tâi-gí` and `好 hó`. The adjacent romanization choice
  remains selectable. Identical Hanji with different readings remain distinct
  annotated choices.
- In POJ sentence composition, bare Space enters the same syllable hyphen as
  the `-` key. Type `tai5 gi2`, select `tâi-gí` with `w` in Combined, then press
  Enter. Romanization Only uses `q` for that example.
- Letter keys select candidates; Shift plus a selection letter selects the
  other script. Enter commits the sentence. Space outside composition belongs
  to the application. TL retains its existing Space selection behavior.

## Cause and implementation

The previous sentence-mode branch in
`desktop/crates/taigi-desktop-core/src/keys/intent.rs:206` mapped Space to
`CommitHighlightedCandidate`. The new Linux regression test reproduced this:
typing `tai5` followed by Space selected `台` and hid the candidates. The POJ
branch now emits `Input("-")` through the existing engine input path.

`desktop/crates/taigi-desktop-core/src/composing/presentation.rs:54` constructed
Combined Hanji cells with no annotation and deduplicated by Hanji alone. With
sentence mode enabled, it now attaches the candidate reading and deduplicates
Hanji choices by the Hanji/reading pair. Existing selection-to-candidate mapping
still determines committed text; annotations do not enter the document.

Reference: the installed `pojhan.schema.yaml` uses `always_show_comments: true`
and POJ comment formatting. The earlier isolated IThuan reference transcript
is `logs/validation/ithuan-reference-letter-flow.txt`. The requested Space
alias is a local behavior change; the IThuan schema itself uses Space to select.

## Verification

- 453 desktop-core tests passed, including distinct readings of the same Hanji.
- 62 Linux core/platform/FFI tests passed, including full POJ Space input,
  annotated candidates, romanization selection and Enter.
- Desktop-core and the three Linux crates passed Clippy with warnings denied.
- Formatting checks passed for the changed Rust files; `git diff --check` passed.
- The rebuilt installed library passed `scripts/check-taigi-poj-space.py` in
  Combined and Romanization Only: Space/hyphen equivalence, Backspace,
  annotations, editable selection and Enter. Existing installed sentence-mode
  checks passed in both TL and POJ.
- `scripts/check-taigi-fcitx-context.py` passed against the restarted service
  using a private input context, including POJ Space input, mixed scripts,
  sentence prefix reselection, and symbol picker behavior.

Logs use the prefix `logs/validation/poj-space-` and date `20261021`.
The complete `make linux-check` stopped while downloading the missing
`libadwaita-sys 0.9.2` dependency because `static.crates.io` could not resolve.
The GTK settings application and other desktop platforms were not validated.

The release Rust FFI library was rebuilt with the existing private dictionary
prefix. CMake was unavailable, so the unchanged C++ shell object was relinked
using the existing generated CMake `link.txt` command against the new Rust
archive. Installed and staged addon SHA-256:
`863eb70223e0f732841d12a48c7ab8c2e81f046290335826e2ce61651196c105`.

## Installation and rollback

The installed addon was atomically replaced at
`~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so`; Fcitx5 was restarted.
The user's settings were not changed. The prior addon and a settings snapshot
are saved in `backups/poj-space-20261021/`.

To restore the previous addon, run from the workspace root:

```sh
cp issues/taigikeyboard-20261008/backups/poj-space-20261021/libtaigikeyboard.so ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so.rollback
mv ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so.rollback ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so
fcitx5 -r -d
```

This repair builds on the existing uncommitted sentence-mode changes. No commit
or upstream publication was made.
