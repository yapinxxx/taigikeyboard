# Personal TaigiKeyboard roadmap

Updated November 11, 2026 against official source `b75f24409`.
This plan replaces a feature wish list based only on official 3.8.0 with a
maintenance plan grounded in [the current source comparison](UPSTREAM-COMPARISON.md).
It is separate from the IThuan/Rime portability and licensing work.

## P0 — integrate the surviving differences safely

**Status: pending.** Official main is fetched and the main checkout is synced.
The personal branch matches its published integration snapshot, but does not
yet contain the latest 167 upstream commits. The trial merge was rolled back;
there is no unfinished merge in the active checkout.

1. Start an isolated integration feature from `bgan/integrated-space-word-break`.
2. Resolve the AppConfig field-15 collision by preserving upstream's fields
   and allocating a checked free field for Editable Sentence. Keep the existing
   saved JSON preference usable; assess wire compatibility with existing builds.
3. Regenerate all platform protocol bindings with the project's supported tools.
   Do not hand-merge Java/Swift generated descriptors or silently omit mobile.
4. Adapt custom-word and dictionary-source handling to the new upstream APIs.
   Preserve enabled-source filtering and user-word identity in the automatic
   Roman-boundary decision.
5. Reconcile the other recorded key, query, settings, and display conflicts.
   Retain upstream TPS and learning improvements instead of replacing them with
   old fork implementations.

**Exit gate:** clean merge; generated bindings agree with the protocol; relevant
engine/desktop/Linux tests and strict Clippy pass or have explicitly reproduced
baseline failures; no installed module changes during source integration.

## P1 — decide which overlay parts can be retired

Run official and personal builds with each build's matching dictionaries and
isolated data/configuration. Record exact source revisions, settings, keys,
preedit, candidates, caret position, and commits. Source inspection alone must
not be labeled a successful application test.

| Replay | Decision it controls |
| --- | --- |
| `li2ho2`, Space separator; Romanization Only; pairing display | Keep upstream formatting/settings; remove redundant fork formatting only if the requested output remains available. |
| `tak8keho2`, whole and shorter selections, reopen and reselect as 逐雞好 | Retain explicit-confirmation sentence mode unless upstream now passes the same complete interaction. |
| All eight sentence keys across pages and both scripts | Preserve one key per displayed cell; reuse upstream paging machinery. |
| `bokahi`, `bok`, `bok8`; `in`, `inn`, incomplete `b`, abbreviation `ss` | Keep only the exact prefix/reading restrictions still necessary with current data. |
| `boe9khi` and `boe9khi3`, select/edit before Enter | Preserve unmatched Roman segmentation without an early document commit; check custom words and dictionary toggles. |
| `ho2` plus Shift+Enter, before and after selecting Hanji | Reuse upstream literal-commit resolution; preserve the retained-sentence extension and explicit hyphens. |
| Space/Enter on incomplete `k`, an explicit suggestion pick, final Space | Keep sentence confirmation distinct from accepting a completion. |
| Backtick held, released, and pressed again | Decide whether the once-per-press delta is still required from observed events. |
| Symbol opening/cancellation and TPS with the setting on/off | Preserve composition boundaries and upstream TPS independence. |

**Retirement rule:** remove an overlay change only after current official source
passes its user interaction, not merely because a similarly named feature exists.

## P2 — Linux application acceptance and packaging

- Build the complete package after integration. The previous full package and
  newer test addon are different revisions; do not treat the old IBus binary as
  containing the Roman-boundary fix.
- Test the loaded module in normal desktop applications, including remote
  development's real desktop session. A remote shell build is not an IME test.
- Back up settings/learning records consistently; check for a user addon
  registration shadowing the packaged addon; restart quietly and retain rollback.
- Re-run the two previously reproduced upstream SQLite learning failures on the
  new baseline. Old exclusions are evidence, not permanent permission to skip.
- Review the tone-glyph request independently using authoritative references.
  The historical `bǒe` request is not a claim that numeric tone 9 must be remapped.

## P3 — reduce long-term maintenance

- Prepare small upstream proposals only for differences that remain after P1.
  Keep official features, personal preferences, and confirmed defects distinct.
- Keep IThuan schema/dictionary distribution questions in the IThuan project.
  A successful local setup is not evidence of redistribution permission.
- Keep a versioned, checksum-verified copy of `docs/bgan` in Obsidian. Use the
  project hub and roadmap as management notes; label copied source docs as a
  dated snapshot. Never mirror-delete personal notes when refreshing a snapshot.
- Keep earlier feature branches and history backups until their changes and
  recovery purpose are accounted for. The canonical working branch now matches
  the published name so normal pull/push targets are unambiguous.
- Update this comparison at an upstream release or meaningful engine/API change,
  before another rebase or installation; skip repeated full builds for docs-only
  changes.

Current application evidence is still the dated evidence in [PROGRESS.md](PROGRESS.md).
No new runtime acceptance or cross-platform build is claimed by this roadmap update.
