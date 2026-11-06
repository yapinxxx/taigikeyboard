> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# TaigiKeyboard: IThuan-style combination selection

This account uses a local TaigiKeyboard 3.6.13 addon with an optional sentence
mode. The system 3.6.10 package remains available for rollback.

The design reference is the installed IThuan `pojhan` schema, tested in isolated
librime sessions: six candidates selected with **q w d f z x**, whole or shorter
reading-prefix combinations, and reopening the reading to reselect a combination.
The rejected per-Hanji-cell picker has been removed.

## Typing

The example below uses Hanji-first Hanji–Romanization Pairing. Combined and
Romanization Only retain their normal candidate displays.

1. Type `tak8keho2`.
2. **q** selects `逐家好`. Alternatively, **w** selects `逐家`, then **q**
   selects `好`. Letter selections stay inside the composition.
3. **Enter** confirms the active preview and commits the sentence.
4. To revise a selection, **Left/Right** move through the original reading.
   The candidate list contains combinations for the prefix before that caret.
   Select again with the letter shown beside the desired combination.
5. **End** returns to the end of the remaining input. PageUp/PageDown page
   the six candidate choices; numbers type tones.

For example: select `逐家好`, press Left three times to reach the end of
`tak8ke`, choose `逐`, then choose `雞` for `ke`; End returns to `ho2`, where
q selects `好`. Enter sends `逐雞好`.

Sentence mode respects the stored display mode, script order, and Show Typed
Text First setting. Combined keeps Hanji and romanization separately selectable;
paired display shows them together and follows the chosen script order.
Shift plus a selection letter chooses the other script. For `ho2` in Combined,
q selects `好` and w selects POJ `hó`; Enter commits the chosen script.
Sentence mode uses the Standard tone scheme. With No Hyphens enabled,
the whole POJ candidate uses spaces: type `li2ho2`, select the `lí hó`
candidate once with `w` in Combined, then press Enter. Turning No Hyphens
off restores `lí-hó`. In Romanization Only the selection key is `q`.

Space now follows the installed IThuan input method in both POJ and TL:
it accepts the highlighted candidate in its displayed script. If the next
candidate menu remains, the selected prefix stays editable; if no menu
remains, it commits the sentence. Space also commits a sentence whose words
were already selected with letter keys. Enter remains available to finish
the sentence at any point. Space does not insert a word boundary while composing.

For example, type `ho2` and press Space to commit `好`. To commit `hó`,
highlight that POJ candidate with Down, then press Space. Selecting `逐家`
from `tak8keho2` with Space keeps the remaining `ho2` editable; the next
Space accepts `好` and commits `逐家好`.

Auto-space adds a trailing space after a romanized sentence when enabled,
whether Enter or Space finishes it. Hanji output gets no automatic trailing
space. With auto-space disabled, Space after the composition has ended types
a normal space in the application. Candidate vocabulary and
ranking still come from TaigiKeyboard, so they differ from IThuan's dictionary.

Combined display now includes the POJ/TL reading beside each Hanji choice
while sentence mode is enabled, for example `台語 tâi-gí`. The separate POJ
choice remains selectable. Different readings of the same Hanji remain
distinct annotated choices.

No Hyphens replaces candidate syllable hyphens with spaces.
The [direct-candidate correction](DIRECT_CANDIDATE_20261021.md) makes
No Hyphens format the complete candidate, without requiring separate picks.
The latest [IThuan Space report](ITHUAN_SPACE_20261021.md) supersedes the
earlier Space-to-hyphen and literal-word-space shortcuts.

## Installation and rollback

- Addon: `~/.local/lib/taigikeyboard-sentence/libtaigikeyboard.so`
- Override: `~/.local/share/fcitx5/addon/taigikeyboard.conf`
- Matching dictionary assets: the addon's `prefix/share/taigikeyboard/dictionaries/`
- Source: `taigikeyboard/`; patch: `issues/taigikeyboard-20261008/artifacts/taigikeyboard-editable-sentence.patch`
- Original backup: `issues/taigikeyboard-20261008/backups/taigi-sentence-backup-20261008-002402/`
- Flag: `values.editableSentenceEnabled` in TaigiKeyboard's settings JSON.

The private dictionary assets are required: the system package's association
format is older and cannot be opened by this engine.

Run the following commands from the workspace root (`bg-debug/`):

```sh
python3 issues/taigikeyboard-20261008/scripts/taigi-sentence-mode.py rollback issues/taigikeyboard-20261008/backups/taigi-sentence-backup-20261008-002402
fcitx5 -r -d
```

Rollback preserves other settings and learning records.

## Verification

The composition, desktop and Linux runtime tests cover letter-key combination
selection, six-choice paging, prefix reselection, numeric tones, cancellation and
Enter. Logs for this revision are `issues/taigikeyboard-20261008/logs/validation/ithuan-*`.

The installed-library checker uses temporary settings and learning databases:

```sh
python3 issues/taigikeyboard-20261008/scripts/check-taigi-sentence-install.py tl
python3 issues/taigikeyboard-20261008/scripts/check-taigi-sentence-install.py poj
```

`issues/taigikeyboard-20261008/scripts/check-taigi-fcitx-context.py` exercises the running Fcitx5 addon through a
private client input context, without typing into another application.

A broader earlier engine run found `dispatch/tests/user_data_writes.rs:298`
failing with sentence mode disabled. It fails at the same assertion on untouched
upstream code; see `issues/taigikeyboard-20261008/logs/validation/upstream-user-data-writes.log`.
Full Windows/macOS and the unchanged GTK settings app were not built.

Current validation: 655 composing tests, 450 desktop-core tests and 58 Linux
runtime/platform/FFI tests passed. Composing, desktop-core and Linux Clippy
passed. The rebuilt addon passed the actual installed-library checks in TL/POJ
and the running Fcitx5 D-Bus test. The latter selected `逐家` with w, `好` with
q, committed with Enter, and reopened/reselected combinations to produce
`逐雞好`. Evidence: `issues/taigikeyboard-20261008/logs/validation/ithuan-running-fcitx.txt`.
