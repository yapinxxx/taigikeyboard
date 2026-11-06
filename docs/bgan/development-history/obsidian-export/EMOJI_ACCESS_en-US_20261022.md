> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# Emoji access while using TaigiKeyboard

**Date:** October 22, 2026 · **Platform:** Linux / Fcitx5  
**Other language:** [fs-TW / Taiwanese Taigi](EMOJI_ACCESS_fs-TW_20261022.md)

Emoji access is now available through two convenient keyboard gestures while
TaigiKeyboard remains active. This uses Fcitx5's existing Unicode and Quick
Phrase addons, exposed through the keyboard's Vial layout. No emoji engine
was added to TaigiKeyboard, and no TaigiKeyboard source or Fcitx shortcut
settings were changed for this work.

## Before and after

The connected keyboard is **Pragmatic 60 34-key OS-aware**, using a Dvorak
layout with five layers. Holding the Enter thumb key activates Layer 4.

| Gesture | Before | Current behavior |
|---|---|---|
| Hold Enter, tap E | Transparent Layer 4 position; inherits base E | Sends Ctrl+Alt+Shift+U to open Fcitx Unicode search |
| Hold Enter, tap O | Transparent Layer 4 position; inherits base O | Sends Super+semicolon to open Fcitx Quick Phrase |

Both Fcitx shortcuts already existed. Super+backtick was initially suggested
for Quick Phrase, but it conflicted with GNOME window switching. Reading
the active Fcitx configuration confirmed Super+semicolon as an existing
alternative, so no Fcitx configuration change was necessary.

## Daily use

Finish the current Taigi composition before opening either picker.

- **Unicode search:** hold Enter, tap E, then release the keys. Search a
  description or an exact codepoint such as `U+1F600`, then select 😀.
  Description searches cover Unicode broadly and can also return Hanji.
- **Quick Phrase:** hold Enter, tap O, then release the keys. Type `:smile:`
  and select candidate **1** to insert 😄.

Vial displays the first combination as **Meh+U**: Meh means Ctrl+Alt+Shift.
The second is **LGui+semicolon**: LGui is the left Super/Windows modifier
on Linux. These are direct modified-key assignments, not recorded macros.

## Implementation and verification

Only two keymap cells were written, using normal-user Vial vendor-interface
access without sudo, unlocking, or firmware flashing:

| Position, zero-based | Keycode | Meaning |
|---|---|---|
| Layer 4, row 2, column 3 | `0x0718` | Ctrl+Alt+Shift+U |
| Layer 4, row 2, column 2 | `0x0833` | Super+semicolon |

A fresh live backup was saved before writing. Full readback and a separate
verification read confirmed exactly those two changes; other keys, macros,
tap dances, combos, key overrides, and settings matched the backup.
Private Fcitx input-context checks successfully inserted 😀 through Unicode
search and 😄 through Quick Phrase. Those checks exercised the emitted chords,
not physical thumb-key presses; the user subsequently confirmed the shortcuts
work on the keyboard.

## Backups and evidence

- Original layout before the emoji mappings (original local artifact: `backups/vial-emoji-keys-20261022/before.vil`)
- Fresh backup of the user-confirmed working layout (original local artifact: `backups/vial-working-20261022-032801/pragmatic-working-20261022-032801.vil`)
- Working-backup metadata and checksum (original local artifact: `backups/vial-working-20261022-032801/backup-metadata.json`)
- Verification log (original local artifact: `backups/vial-emoji-keys-20261022/fcitx-verification.log`)
- Change and guarded two-key restore script (original local artifact: `scripts/set-vial-emoji-keys.py`)

The `.vil` backups can be loaded in Vial. The earlier backup restores the
pre-change layout; the working backup retains the emoji mappings. Raw
configuration snapshots accompany both backups.

For author feedback, this is an example of using existing Fcitx functionality
alongside TaigiKeyboard. A useful upstream addition would be documentation
of emoji access and shortcut conflicts. The physical key positions are specific
to this keyboard and are not proposed as universal TaigiKeyboard bindings.

References: [Fcitx special-character input](https://fcitx-im.org/wiki/Tips_and_Tricks),
[Vial modified-key assignments](https://get.vial.today/manual/any-key.html).
No report or message has been sent to the author.
