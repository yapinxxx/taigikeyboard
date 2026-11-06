> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# Shift+Enter pronunciation output — fixed October 23, 2026

This corrects the known regression recorded in the October 22 author comparison.
The rebuilt addon is installed and Fcitx5 has restarted.

| Action in editable sentence mode | Before | Fixed behavior |
|---|---|---|
| Type `ho2`, then Shift+Enter | Commits `好` | Commits `hó` |
| Type `ho2`, select `好`, then Shift+Enter | Keeps the selected Hanji output | Commits the underlying typed pronunciation `hó` |
| Type `li2-ho2`, then Shift+Enter | Confirms dictionary choices | Commits typed pronunciation `lí-hó`, preserving the explicit hyphen |

The root cause was the shared key classifier routing the literal-commit action
to the same `Commit` path used for sentence confirmation. That path accepted
the highlighted dictionary candidate before finalizing. Editable sentence mode
now routes the configured literal action through the existing `CommitAsTyped`
engine operation, bypassing candidate confirmation. TPS and the original
non-sentence workflow retain their existing paths. No phonetic rules, candidate
ranking, dictionary data, keyboard firmware, or Vial configuration were changed.

Normal Enter and Space keep their sentence-selection behavior. Auto-Space
adds a trailing space to the final typed romanization only when enabled.
Shift+Enter outside a composition continues to pass to the application.
This uses the existing typed-reading normalization; it does not introduce
automatic segmentation of unseparated multisyllable input.

Validation:

- The new regression test failed before the fix and passed afterward. It covers
  POJ/TL, Auto-Space on/off, unselected input, and selected full/partial phrases.
- Full desktop-core suite: **454 passed**. Linux core/platform/FFI: **68 passed**.
- Desktop-core and Linux-core Clippy passed with warnings denied.
- Rebuilt-addon checks passed for Shift+Enter, ordinary Space, `bokahi`, exact
  `in`, and the Han-lo / POJ-first toggle.
- Running Fcitx5 verification (original local artifact: `logs/validation/shift-enter-running-fcitx-20261023.log`)
  confirmed `hó`, including after selection, explicit-hyphen pronunciation,
  current Auto-Space behavior, and idle pass-through.
- The broad Linux gate remains blocked by an unavailable offline GTK dependency;
  other platforms and the GTK settings application were not validated.

The previous addon is saved in the rollback backup (original local artifact: `backups/shift-enter-20261023/`).
The October 22 patch and feedback ZIP remain historical snapshots and do not
contain this fix. No message has been sent to the author.
