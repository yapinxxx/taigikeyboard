> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# TaigiKeyboard bgan

**Unofficial Linux/Fcitx5 customization of TaigiKeyboard, based on official desktop-3.8.0.** The bgan workflow is kept as an overlay so upstream improvements can be incorporated while its feature contract is checked.

| Reference | Purpose |
|---|---|
| `bgan/3.8.0` | Exact source snapshot matching the currently installed bgan build |
| `bgan-3.8.0-20261023` | Version tag for that source snapshot |
| `b8f00d81841cc69c1f9080ce6edceef7a912b392` | Running version's source commit |
| `bgan/main` | Maintenance scripts, CI and published comparison notes |
| `desktop-3.8.0` / `69751e58ba0dbe64d3512330e4580eefaff71521` | Official source baseline |

Version: **3.8.0-bgan.20261023**. The official project is [taigikeyboard/taigikeyboard](https://github.com/taigikeyboard/taigikeyboard). The upstream Apache-2.0 license, notices and separate dictionary/font terms remain in place.

Start with the [English differences](BGAN_FORK_DIFFERENCES_en-US_20261023.md) or [fs-TW differences](BGAN_FORK_DIFFERENCES_fs-TW_20261023.md). These distinguish requests from capabilities that official 3.8.0 already provides.

The runtime Rust/C++ code in the maintenance branch matches the running snapshot. Maintenance additions are identified in the comparison notes. Canonical notes are maintained separately; files published in the fork are self-contained export snapshots.

For a future upstream release, `tools/desktop/rebase-bgan.py desktop-<version>` creates a new worktree, reapplies the overlay and invokes `tools/desktop/verify-bgan.py`. The helper fetches from the official `upstream` remote. It leaves the current installation in place while verification runs. Conflicts or failed feature checks require review.

The custom CI workflow runs on pushes and pull requests targeting `bgan/main`, and supports manual dispatch. It checks formatting, builds the addon and runs the feature contract. The current 3.8.0 SQLite exclusions are explicit in the workflow summary and are restricted to that release. A newer release gets the strict checks again.

Upstream acceptance of these features would reduce the custom overlay. Until then, each update needs verification; this fork does not promise maintenance-free upgrades. Feedback can be prepared from the comparison without opening an upstream issue or pull request automatically.
