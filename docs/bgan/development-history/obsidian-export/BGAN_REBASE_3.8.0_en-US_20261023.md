> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# bgan 3.8.0: rebase and feature verification

**Status: this computer is using 3.8.0-bgan.20261023.** The bgan features are verified and retained. This is an unofficial custom build genuinely rebased onto desktop-3.8.0, rather than only relabeling the old binary.

**Upstream:** desktop-3.8.0 · `69751e58ba0dbe64d3512330e4580eefaff71521`  
**bgan branch:** bgan/3.8.0 · `b8f00d81841cc69c1f9080ce6edceef7a912b392`

## Feature verification

| Feature | Verified behavior — passed |
|---|---|
| Candidate readings | Hanji and POJ in one row; separate POJ choice in Han-lo |
| Editable selections | Whole and shorter choices stay editable; Left reopens reading; reselection produces 逐雞好 |
| Space / Enter | Highlighted script accepted; partial sentence retained; final selection committed once |
| POJ formatting | Complete lí hó choice, explicit lí-hó option, independent trailing Auto-Space |
| Shorter bo | bokahi → 無 → 佮 → 意; valid bok retained |
| Exact reading | in remains exact through repeated paging; inn, b and ss remain searchable |
| Han-lo / POJ | Persistent backtick toggle; 台 / POJ; no duplicate bare POJ row |
| Shift+Enter | ho2 → hó before or after selecting Hanji; idle shortcut passes through |
| Other behavior | POJ/TL, Romanization Only, cancellation and symbol picker checks pass |

All four dictionaries match official 3.8.0 byte for byte. Upstream now displays tone digits as marks—for example, tak8keho2 renders as ta̍kkehó. bgan retains that upstream display; reading-caret navigation and reselection still pass.

## Tests and limits

- All 12 bgan feature groups passed, including the installed binary’s own dictionary prefix.
- 486 desktop/storage and 70 Linux core/platform/FFI tests passed.
- The remaining engine suite passed 1,563 tests; five existing tests remain ignored.
- Two SQLite learning failures were explicitly excluded after reproducing the same failures on unmodified official 3.8.0. These remain recorded failures.
- Engine, desktop core/storage and Linux core/platform/FFI Clippy passed; formatting passed for all three workspaces.
- Three integration scripts passed in a separate D-Bus/Fcitx5 daemon, avoiding interference from active desktop typing.
- Full GTK/update workspace gates require missing OpenSSL/GLib/GTK development libraries. The settings application uses the official 3.8.0 binary. Installation on a second physical device is not yet tested.

`literal_picks_reuse_existing_keys_and_keep_tone_boundaries_distinct`  
`the_engine_writes_what_the_platforms_wrote`

Validation receipts (original local artifact: `issues/taigikeyboard-20261008/logs/validation/bgan-380-final-validation.json`)

## Receiving future upstream updates

The bgan overlay is kept as three commits on top of the official release, in a separate branch. For a future official release tag, the helper creates another worktree, rebases the overlay, builds it, and runs the verification gates. It does not replace your working installation automatically.

```bash
cd legacy-workspace/taigikeyboard-bgan-3.8.0
python3 tools/desktop/rebase-bgan.py desktop-<next-version>
```

Replace `desktop-<next-version>` with an actual upstream tag. Conflicts or failed checks stop the process while leaving the current installation in place. This build environment needs its existing protoc, CMake and Fcitx5-header configuration.

To rerun verification on the current 3.8.0 while explicitly accepting the two reproduced upstream SQLite test limits, use the option below. It is restricted to desktop-3.8.0 and must be reevaluated for a later release.

```bash
python3 tools/desktop/verify-bgan.py --allow-known-upstream-sqlite-tests
```

Rebasing reduces repeated work, but custom changes still need verification until they are accepted upstream. Updating the official APT package does not automatically update the per-user bgan override. No branch or issue has been published.

## Package and backup

bgan 3.8.0 archive (original local artifact: `local-downloads/taigikeyboard-bgan-3.8.0-20261023-ubuntu24.04-amd64.tar.gz`)
Before-activation backup (original local artifact: `issues/taigikeyboard-20261008/backups/bgan-3.8.0-activation-20261023-142715`)

Your settings were preserved, and the learning databases were backed up consistently before activation. The old 3.6.13-based bgan binary and source directory remain available for recovery. Future bgan maintenance should use the new bgan/3.8.0 worktree.

[en-US](BGAN_REBASE_3.8.0_en-US_20261023.md) · [fs-TW](BGAN_REBASE_3.8.0_fs-TW_20261023.md) · [Author overview](../../author-feedback/2026-10-23/README_en-US.md)
