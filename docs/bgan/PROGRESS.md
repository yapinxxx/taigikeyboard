# Personal integration progress

Updated November 6, 2026. This branch combines the five personal feature
branches and the subsequent Linux integration fixes.

The [Linux desktop experience](DEVICE-EXPERIENCE.md) connects the user's
reported typing and terminal-output problems to the changes below. The new
module has automated coverage; a successful interactive retry and the
requested `ǒ` versus current `ŏ` tone glyph still need confirmation.

The latest tested code is `a555863a06e5283a375d8e8124d3d5e6cfaeefb6`.
Its base is local main `86aa54f44396a82c2b6d0e2bf6bef2c873ea23c6`, whose
files match upstream `5159fad364ef787d0d82d78afd1e21a2c04527f4`.
The local history preserves code changes while removing the previously
identified assistant coauthor trailers. Older session records are retained.

See [development history](development-history/README.md) for earlier sessions and
Obsidian reports, and [remote development](REMOTE-DEVELOPMENT.md) for the handoff.

## Completed behavior

- Editable sentence composition retains selections until final confirmation.
- Eight sentence selection keys correspond to eight visible candidate cells.
- Space can break words; Enter confirms the sentence. A final Space sends
  the sentence when nothing remains to confirm. Completion suggestions require
  explicit selection.
- POJ display, exact reading boundaries, literal pronunciation commits, and
  TPS independence have regression coverage.
- A complete unmatched tone-closed reading becomes a separate Roman segment
  when the next syllable starts. In POJ sentence mode, `boe9khi` or
  `boe9khi3` followed by Enter produces `bŏe 去`. The Roman segment remains
  editable before confirmation. Dictionary and user readings keep their
  existing phrase matching. Tone rendering is unchanged.

The last behavior was missing because append kept `boe9` and subsequent
letters in one pending query (`engine/composing/src/transition.rs`,
`append_continuous`). The regression failed before the fix with pending
`boe9khi` instead of `khi`. The fix separates the unmatched reading inside
the shared engine without committing it to the application.

## Validation

The final code revision passed these checks on Linux x86_64 with Rust
1.99.0 and Fcitx5 5.1.7:

| Check | Result |
| --- | --- |
| Production-lexicon sentence boundary regressions | 10 passed |
| Linux session characterizations | 38 passed |
| Shared desktop core tests | 479 passed |
| Dispatch user-data adapter tests | 3 passed |
| Feature contract against the optimized Fcitx5 addon and installed dictionaries | 15 groups passed |
| Composing/dispatch Clippy, all targets, warnings denied | Passed |
| Linux core test Clippy, warnings denied | Passed |
| Rust formatting and diff whitespace | Passed |

Representative commands, with the installed stable toolchain and cached
dependencies, are:

```bash
cargo test --offline --locked --manifest-path engine/Cargo.toml -p composing --test prod sentence_reading_boundaries
cargo test --offline --locked --manifest-path linux/Cargo.toml -p taigi-linux-core --test session_characterisation
cargo test --offline --locked --manifest-path desktop/Cargo.toml -p taigi-desktop-core
cargo test --offline --locked --manifest-path engine/Cargo.toml -p dispatch --lib user_data::with_stores
cargo clippy --offline --locked --manifest-path engine/Cargo.toml -p composing -p dispatch --all-targets -- -D warnings
```

The earlier full integration run at `2624227315e5aa81bc434c4e3aa85c8d68fa35ca`
reported 1,611 engine tests passed, two failed, and five ignored diagnostics.
Both failures also reproduced on the upstream baseline:

- `dispatch/tests/literal_learning_identity.rs:97`:
  `literal_picks_reuse_existing_keys_and_keep_tone_boundaries_distinct`.
- `dispatch/tests/user_data_writes.rs:298`:
  `the_engine_writes_what_the_platforms_wrote`.

The full engine suite therefore has known failures. The focused checks above
do not imply that the full suite is green.

## Build and manual verification

A complete Linux package was built at `2624227315e5aa81bc434c4e3aa85c8d68fa35ca`.
The newer `a555863a0` code was built and checked as an optimized Fcitx5
module. The prior package's IBus binary does not contain that newer fix.
Local build artifacts, installation backups, and detailed logs remain outside
the tracked repository.

Remaining work is interactive application testing of the latest Fcitx5 module,
followed by a complete package rebuild when preparing the next installable
release. Other platforms have not been rebuilt or verified for the latest fix.
