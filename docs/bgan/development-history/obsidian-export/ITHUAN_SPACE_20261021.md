> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# IThuan-style Space — October 21, 2026

Installed and verified in the running Fcitx5 service. This revision replaces the local literal-space shortcut with the Space behavior
of the installed IThuan `pojhan` input method. No Hyphens continues to format
the complete POJ candidate with spaces.

## Behavior

- While composing POJ or TL, Space accepts the highlighted candidate in its
  displayed script. Highlighting `好` writes Hanji; highlighting `hó` writes POJ.
- If a following candidate menu remains, the accepted prefix stays editable.
  If no candidate menu remains, Space commits the composition.
- After selecting a whole sentence with letter keys, Space commits it too.
  Letter keys themselves keep their existing editable selection behavior.
- Enter retains its existing sentence-finalization behavior. Space outside a
  composition remains the application's ordinary Space key.
- The enabled auto-space setting can add a trailing space after final POJ
  output. It does not add one after Hanji. The user's stored settings are
  unchanged.

Examples in Combined mode:

| Input | Result |
|---|---|
| `ho2`, Space | Commits `好` |
| `ho2`, Down, Space | Commits `hó` |
| `tak8keho2`, Down twice, Space | Accepts `逐家`; `ho2` remains editable |
| Space after the preceding partial choice | Commits `逐家好` |
| `li2ho2`, `w`, Space, with No Hyphens enabled | Commits `lí hó` |

## Reference and root cause

The installed `~/.local/share/fcitx5/rime/pojhan.schema.yaml` uses
`fluency_editor` and overrides `editor/bindings/space` to `commit_composition`.
Librime's [editor implementation](https://github.com/rime/librime/blob/master/src/rime/gear/editor.cc)
confirms the current selection and commits when no menu remains.

`scripts/check-ithuan-space-reference.py` copies the installed compiled schema
and dictionary into a temporary directory and replays the keys through the
installed librime C API. It never opens the real user dictionary. The reference
log confirms whole-candidate commits, partial selection, final selection after
a partial pick, letter-selected final output, and highlighted POJ output.

The TaigiKeyboard mismatch was in
`desktop/crates/taigi-desktop-core/src/keys/intent.rs`: sentence-mode Space
emitted `Input(" ")` for POJ. The regression test reproduced `hó ` in the
preedit instead of the reference's `好` commit. Space now emits the explicit
`ConfirmSentenceCandidate` intent. The shared intent executor accepts the
highlight, keeps a following menu editable, and calls the same sentence
finalization helper as Enter when the menu closes.

## Validation

- 453 desktop-core tests and 64 Linux core/platform/FFI tests passed.
- The new Linux scenario checks POJ and TL, auto-space on and off, final Hanji,
  highlighted POJ, shorter-prefix selection, finalization after a letter pick,
  and Space passed to the application while idle.
- Desktop-core and the three Linux crates passed Clippy with warnings denied.
- Existing No Hyphens tests retain direct one-step candidate selection.
- Installed-library and running-service checks are recorded under
  `logs/validation/ithuan-space-*`.
- The installed checker passed Combined and Romanization Only, No Hyphens
  enabled and disabled, Space on final highlighted/letter-selected output,
  and partial choices. The running Fcitx5 check passed the same Space cases
  alongside the existing sentence editing and symbol-picker cases.
- Existing installed sentence checks passed in TL and POJ. Their temporary
  settings now explicitly pin Hanji-first pairing and auto-space off, so their
  fixed Hanji expectations do not depend on the user's current script order.

Other platform shells and the GTK settings application were not rebuilt.
The Linux release library uses the existing private dictionary prefix; the
unchanged C++ shell is relinked with the generated CMake linker command.

The previous library and settings snapshot are saved in
`backups/ithuan-space-20261021/`. The installed addon SHA-256 is
`ef058acb690cbcf206a5326af238560842b34116ea1d009e9290035adefc9a15`.
The source addon was atomically installed, Fcitx5 restarted, and the stored
settings confirmed byte-identical to the backup. To restore that addon from the workspace root:

```sh
cp issues/taigikeyboard-20261008/backups/ithuan-space-20261021/libtaigikeyboard.so ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so.rollback
mv ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so.rollback ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so
fcitx5 -r -d
```
