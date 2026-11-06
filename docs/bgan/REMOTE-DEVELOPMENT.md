# Continue development on another machine

Start with [current progress](PROGRESS.md), the [development history](development-history/README.md),
and [recent device experience](DEVICE-EXPERIENCE.md). The historical Obsidian
reports preserve the reasoning and failures behind the implementation; their
old installation commands and test totals are not current setup instructions.

## Source and branch

The local integration branch is `bgan/space-word-break`. The planned GitHub
destination is `bgan/integrated-space-word-break`, preserving the existing
remote branches after local history cleanup. Until a push succeeds, the newer
source and documents are local only. Verify the published tip before moving
development to another machine.

Latest tested code: `a555863a06e5283a375d8e8124d3d5e6cfaeefb6`.
Later documentation commits do not change that tested implementation.
Cleaned baseline: `86aa54f44396a82c2b6d0e2bf6bef2c873ea23c6`, with the same
file tree as upstream `5159fad364ef787d0d82d78afd1e21a2c04527f4`.

## Fresh role workspace

Use these commands only in a new empty workspace, after the branch has been
published. Substitute your fork URL. The initial clone checks out the published
integration branch, so creating local `main` does not replace an existing local
main branch. It keeps the cleaned baseline available for meaningful comparisons.

```bash
mkdir taigikeyboard-worktrees
cd taigikeyboard-worktrees
git clone --branch bgan/integrated-space-word-break <your-fork-url> main
git -C main switch -c main 86aa54f44396a82c2b6d0e2bf6bef2c873ea23c6
mkdir -p developer
git -C main worktree add -b bgan/space-word-break ../developer/bgan-space-word-break origin/bgan/integrated-space-word-break
git -C main worktree add -b release/workspace ../release main
git -C main worktree add --detach ../tester bgan/space-word-break
git -C main worktree add --detach ../code-reviewer bgan/space-word-break
git -C main worktree add -b debug/investigation ../debugger bgan/space-word-break
cp developer/bgan-space-word-break/docs/bgan/remote-workspace/wt ./wt
cp developer/bgan-space-word-break/docs/bgan/remote-workspace/AGENTS.md ./AGENTS.md
chmod +x wt
./wt list
```

Use `./wt add <feature> bgan/space-word-break` for another isolated developer.
Use `./wt inspect tester <commit>` or `./wt inspect code-reviewer <commit>`
to hand off an exact clean revision. Removal retains branches and never forces
dirty worktree deletion. Roles describe responsibilities, not access controls.
Each checkout's AGENTS.md and contribution guides still apply.

## Environment and validation

Use the checked-out project's `docs/BUILDING.md` and Linux contribution guide
for the supported environment. Set up required submodules in the active
checkout; the optional corpus remains optional. On the tested Ubuntu desktop,
build dependencies included protobuf-compiler, CMake, Ninja, pkg-config,
GTK 4/libadwaita development files, Fcitx5 development modules,
extra-cmake-modules, dpkg-dev, and desktop-file-utils.

Engine, desktop, and Linux are separate Cargo workspaces. Keep build outputs
in the assigned checkout. This device used Rust stable 1.99.0 and cached locked
dependencies; `RUSTUP_TOOLCHAIN=stable` avoided unwanted cross-target setup.
Build for the target machine and verify its installed libraries rather than
assuming another machine has the same cache, paths, or system packages.

Run the focused commands in [PROGRESS.md](PROGRESS.md). Build the Rust FFI
before linking the Fcitx5 module; the feature contract can be checked with:

```bash
python3 tools/desktop/check-bgan-features.py <built-addon.so> --data-dir <share/taigikeyboard>
```

Use matching dictionary artifacts and isolated configuration/data directories.
The complete earlier Linux package predates the automatic Roman-boundary fix.
The latest tested artifact was an optimized Fcitx5 module, not a rebuilt IBus
package. Two known upstream engine test failures remain documented.

## Remote host versus desktop testing

A remote build or SSH session does not establish that the local application
received the new input module. Build on the intended target, preserve existing
settings/learning data, check for a user addon registration shadowing a system
package, verify the loaded artifact, and restart the desktop's Fcitx5 quietly.
Run interactive acceptance in a real desktop application. Keep terminal logs
in files and preserve a rollback path. No SSH credentials, private host names,
learning databases, or personal keyboard backups are required in Git.

## Immediate next work

1. Verify the published source and docs tip on the new machine.
2. Rebuild and manually test `boe9khi` and `boe9khi3`, editing before Enter.
3. Resolve the requested `ǒ` versus current `ŏ` glyph separately using phonetic
   references; the segmentation fix did not change tone conversion.
4. Rebuild the complete Linux package when ready for another installation.
5. Record platforms actually tested. Review the complete overlay against the
   cleaned baseline before upstream submission or release.
