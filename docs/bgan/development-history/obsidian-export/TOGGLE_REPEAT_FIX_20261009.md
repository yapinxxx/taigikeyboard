> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# Script-swap held-key repeat fix — October 9, 2026

The installed TaigiKeyboard addon previously changed `isTranslateSwapped` on
every backtick keydown, including held-key repeat events. Starting with
half-width punctuation, three press events produced full-width, half-width,
then full-width. This explains repeated punctuation-width toggles when held.
It does not establish the source of the user's separate quick double-input
report for apostrophe and Enter in English.

## Root cause and change

`desktop/crates/taigi-desktop-core/src/keys/shortcut_actions.rs:130` excluded
`ToggleTranslateSwapped` from `fires_once_per_press`. Linux's existing latch in
`linux/crates/taigi-linux-core/src/session.rs:187` therefore never latched the
script swap. Added the action to the shared once-per-press policy and updated
its roster test. Added a Linux regression test covering Combined mode,
unchanged settings revisions during repeats, and a new toggle after release.

This shared policy is also consumed by Windows repeat handling. Windows runtime
behavior was not tested on this Linux device. macOS native shortcut handling
was not changed. No timing-based filter was added to normal typing; distinct
press/release pairs remain distinct presses.

## Validation and installation

- The original installed addon fails the repeat checker. Evidence:
  `../logs/validation/toggle-repeat-before-20261009.txt`.
- Shared desktop-core tests pass, including the updated action roster.
- Linux core, platform, and FFI tests pass, including editable-sentence tests.
- Clippy passes for the changed desktop core and Linux core/platform/FFI,
  all targets, with warnings denied. Formatting and i18n checks pass.
- Rebuilt the Rust release FFI with the existing per-user
  `TAIGIKEYBOARD_PREFIX`, then linked the Fcitx5 module using its existing
  CMake build directory. The first staged build lacked that prefix and failed
  the Hanji check; it was corrected and retested before installation.
- The corrected addon passes held-key repeats, release/repress, full-width
  comma/exclamation/question marks, Hanji selection, Enter commit, and the
  symbol-picker regression checker.
- Installed atomically at the existing per-user addon location and restarted
  Fcitx5. Live private-context check passes: 11 press events toggle once;
  release/repress toggles again. Original punctuation width restored.

Installed SHA-256:
`693f1b0cffd5532d7915bc3a1487652e4ffce9b485b77a77c771325befdf3f4e`.

This validation covers the changed Linux behavior and shared policy. Full
cross-platform release gates were not run.

## Repeatable checks

From the workspace root:

```sh
python3 issues/taigikeyboard-20261008/scripts/check-taigi-toggle-repeat.py
/usr/bin/python3 issues/taigikeyboard-20261008/scripts/check-taigi-toggle-repeat-live.py
```

The first uses temporary settings and user-data directories. The second needs
the desktop D-Bus session, briefly toggles the live punctuation preference,
and restores it. Avoid changing that preference simultaneously with the check.

## Rollback

The previous installed module is saved at
`../backups/libtaigikeyboard.before-toggle-repeat-fix-20261009.so`.
To roll back, copy that module to
`~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so` and restart Fcitx5.
Do not overwrite a module in place while it is loaded: copy to a temporary
file beside it and rename the temporary file over the installed module.
