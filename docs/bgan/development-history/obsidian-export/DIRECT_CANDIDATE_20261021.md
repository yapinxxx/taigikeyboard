> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# Direct POJ candidate formatting — October 21, 2026

Installed the clarified No Hyphens behavior: enabling the switch offers the
complete `lí hó` candidate directly from `li2ho2` or `li2-ho2`. In Combined,
press `w` once to select it, then Enter to commit. Romanization Only uses `q`.
Disabling the switch offers `lí-hó`. Separate word selections are unnecessary.

## Cause and fix

The earlier fixes handled typed Space and sentence auto-space, but the
legacy `hyphenlessRomanEnabled` switch still migrated to Syllable Separator =
None, which concatenated syllables. The installed C ABI reproduced the exact
failure: `li2ho2` offered `你好 líhó` and `líhó`.

`desktop/crates/taigi-desktop-core/src/settings/document.rs` now maps the enabled
legacy switch to Syllable Separator = Space for this local desktop repair.
The candidate renderer already handles that separator, so candidates and
selected output both show `lí hó`. The disabled legacy switch still uses
the default Hyphen separator. An explicitly stored successor separator choice
retains precedence. The user's stored settings were not changed.

This supersedes the earlier explanation that No Hyphens should concatenate
syllables, and the earlier recommendation to select `lí` and `hó` separately.
Explicit word-space typing and automatic trailing spacing remain available.

## Validation

- 453 desktop-core and 64 Linux core/platform/FFI tests passed.
- Desktop-core and the three Linux crates passed Clippy with warnings denied.
- The settings test checks the enabled and disabled legacy switch. The Linux
  integration test checks the complete displayed candidate, one selection and
  committed output in Combined and Romanization Only, with Space and Hyphen.
- The installed-library checker tests the actual legacy switch on and off,
  both `li2ho2` and `li2-ho2`, candidate labels and one-step selection.
- The running Fcitx5 private input-context check tests `li2ho2`, one `w`
  selection, and Enter producing `lí hó`. It also retains the earlier
  word-space, auto-space, sentence editing and symbol-picker checks.

Logs: `logs/validation/direct-candidate-*-20261021.log`.
Other platform shells and the GTK settings application were not rebuilt.

The previous library and settings snapshot are in
`backups/direct-candidate-20261021/`. The installed addon SHA-256 is
`8495d6666aaaa629ee588a4794506ebfbd87d9a289eb4acefaf71c7e03573d09`.
The Rust library was rebuilt and the existing unchanged C++ shell relinked;
the addon was atomically replaced and Fcitx5 restarted.

To restore the previous addon from the workspace root:

```sh
cp issues/taigikeyboard-20261008/backups/direct-candidate-20261021/libtaigikeyboard.so ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so.rollback
mv ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so.rollback ~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so
fcitx5 -r -d
```
