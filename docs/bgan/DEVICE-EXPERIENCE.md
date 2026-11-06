# Linux desktop typing experience

Recorded November 6, 2026, from the local Linux desktop build and typing
session. These notes describe the user's experience, the resulting fix, and
what still needs confirmation. They supplement the older official 3.8.0
comparison; they do not establish a defect in that official release.

## Keep mixed Roman and Hanji input editable

The user reported seeing `bŏe去` and being unable to edit the sentence into
the requested `bǒe 去`. The reported input was `boe9khi3`; the user also
asked about `boe9khi`. The complete selection and caret sequence was not
provided, so that report alone does not specify an exact key-by-key replay.

Two questions are distinct:

- **Word boundary and editing:** the Roman reading should be a separate,
  editable part of the sentence before 去.
- **Tone glyph:** the requested example uses `ǒ`, while the current converter
  renders `boe9` with `ŏ`. This session changed segmentation, not tone rules.
  The tone-glyph expectation remains unresolved and needs separate review
  against the project's phonetic references before any converter change.

The user's proposed interaction was: when `boe9` has no matching word,
typing `k` should finish that Roman segment and begin looking up the next
syllable. Typing should continue naturally without requiring a manually
inserted Space or confirmation of the unmatched Roman reading.

In the installed pre-fix build, an isolated session reproduced the pending
query remaining `boe9khi` or `boe9khi3`. The closed-reading filter offered
`boe9` whole, but subsequent letters still belonged to that same pending
query. Moving left operated on raw keys, including a tone digit that is not
shown separately in the rendered text. Editing therefore required knowledge
of hidden raw-key positions rather than just the visible word boundary.

## Resulting behavior

The shared engine fix is local commit
`a555863a06e5283a375d8e8124d3d5e6cfaeefb6`.
In Editable Sentence mode it separates a pending, complete tone-closed
reading with no dictionary or user word when the next alphabetic input
arrives. The Roman segment stays in the composition and preserves its
original raw keys. It has no picked-word identity and creates no automatic
learning event. Known readings keep their phrase-matching behavior.

With the session's POJ settings, automated checks now establish:

| Input/action | Expected result | Evidence status |
| --- | --- | --- |
| `boe9`, then `k` | Roman segment `bŏe`; `k` starts the pending query; no application commit | Engine regression passed |
| `boe9khi`, then Enter | `bŏe 去` | Linux session and built-addon checks passed |
| `boe9khi3`, then Enter | `bŏe 去` | Linux session and built-addon checks passed |
| Select 去, reopen the Roman reading, and change `boe9` to `bo5` | Composition remains editable; Enter can produce `無去` | Linux session regression passed |
| A known or custom reading, or Editable Sentence disabled | Existing phrase/input behavior retained | Engine regressions passed |

These are automated results against the engine, Linux session, and optimized
Fcitx5 addon. The user has not yet confirmed a successful interactive retry
of the new module. The fix also does not add reopening of text already
committed to the host application.

## Keep the terminal usable during testing

The user reported that diagnostic output took over the terminal while they
only wanted to test the feature. The restart command used the foreground
Fcitx5 mode. The test installer now restarts in daemon mode and redirects
its output:

```bash
fcitx5 -r -d >/dev/null 2>&1
```

Build and diagnostic logs should be saved to files, with concise progress
updates. A test handoff should identify the built revision and provide one
activation command. The local installer saves the previous addon registration
and supports restoring it; settings and learned data are retained.

## Follow-up on this device

Use the optimized Fcitx5 module from `a555863a0`, with POJ, Editable Sentence,
and Space Breaks Words enabled. Retest both inputs in a normal application
text field. Confirm that the automatic boundary appears when `k` is typed,
that both parts remain editable before Enter, and that Enter sends the
sentence once. Record the application and exact keys if anything differs.

Confirm the intended tone glyph separately. No successful manual acceptance,
application-specific compatibility, or testing on other platforms is claimed
by this note. The earlier complete Linux package predates this fix; its IBus
binary does not establish the newer behavior.

See [integration progress and test evidence](PROGRESS.md) for build revisions,
validation counts, and the two reproduced upstream engine test failures.
