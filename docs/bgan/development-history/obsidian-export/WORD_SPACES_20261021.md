> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# POJ word spaces and auto-space — October 21, 2026

Installed and verified in the running Fcitx5 service. This supersedes the
initial Space-to-hyphen behavior described in `POJ_SPACE_AND_READINGS_20261021.md`.

## Why the settings did not produce `lí hó`

The previous POJ sentence-mode key classifier changed Space to `Input("-")`.
The raw preedit therefore showed `lí-hó`. The enabled legacy No Hyphens switch
(`hyphenlessRomanEnabled`) migrates to Syllable Separator = None: it removes
internal syllable hyphens from selected candidates, producing `líhó`, rather
than converting those hyphens to word spaces. Separately, the earlier local
sentence-mode repair explicitly disabled auto-space at its gate even when the
user had enabled `autoSpaceEnabled`.

The regression was reproduced using the installed C ABI and those exact
settings. It was also pinned by the Linux session regression before the fix.

## Corrected behavior

- Space enters an actual word space in POJ sentence mode. The physical `-`
  key remains available for a syllable hyphen. No Hyphens still controls
  syllable formatting; typed word spaces are preserved independently.
- In Combined, type `li2 ho2`, select `lí` with `w`, select `hó` with `w`,
  then press Enter. Output is `lí hó`. Selecting `lí` before typing Space
  and `ho2` works too. In Romanization Only, the selection key is `q`.
- Auto-space appends a trailing space after Enter when the sentence ends in
  romanized output and the setting is enabled. Hanji output gets no trailing
  automatic space. Selecting individual words keeps the sentence editable.

The stored user settings were not changed.

## Implementation

`desktop/.../keys/intent.rs` preserves the typed Space. The shared composing
engine normalizes tones separately within space-delimited words, retains
word spaces when rendering selected Roman segments, and keeps consumed spaces
with the accepted reading instead of leaving them in front of the next lookup.
It also supports selecting a word before entering Space and the next word.

The engine's sentence finalization now returns its actual final-script spacing
verdict. The desktop bridge and manager carry that verdict to the Enter handler,
which applies the enabled auto-space setting. Punctuation and intermediate
candidate selection retain sentence-mode behavior.

## Verification

- 655 composing-engine tests passed (two additional tests remained ignored).
- 453 desktop-core tests passed.
- 63 Linux core/platform/FFI tests passed, including No Hyphens combined with
  auto-space enabled/disabled, selection before/after Space, and Hanji output.
- Desktop-core and the three Linux crates passed Clippy with warnings denied.
- Final sentence-mode engine and Linux session checks passed after the final
  metadata guard change. Changed Rust files were formatted; diff checks passed.
- The staged and installed libraries passed `check-taigi-poj-space.py` in both
  Combined and Romanization Only with the legacy No Hyphens switch and
  Auto-Space enabled. The existing installed sentence and script-choice checks
  also passed.
- The restarted Fcitx5 service passed the private input-context test, including
  `li2` + Space + `ho2` → `w`, `w` → Enter, exact output `lí hó`, and a trailing
  auto-space. Existing Hanji, mixed-script, sentence editing and symbol picker
  checks passed too.

Logs: `logs/validation/word-space-*-20261021.log`.
The GTK settings application and other platform shells were not built. The full
Linux settings dependency/download limitation remains as documented in the
earlier report. This repair uses the existing Linux build environment and its
private dictionary prefix; the unchanged C++ object was relinked against the
rebuilt Rust archive using the existing generated CMake linker command.

## Backup and rollback

The previous addon and the settings snapshot are in
`backups/word-space-20261021/`. The installed library SHA-256 is
`8b6d86cb126471d93043d19449070c6d9772052dcb9e4b3173bce64ab88672e0`.

To restore the previous addon, run from the workspace root:

```sh
cp issues/taigikeyboard-20261008/backups/word-space-20261021/libtaigikeyboard.so ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so.rollback
mv ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so.rollback ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so
fcitx5 -r -d
```

No commit or upstream publication was made.
