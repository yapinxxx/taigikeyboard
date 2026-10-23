# TaigiKeyboard bgan: wishes compared with official 3.8.0

**Status: personal, unofficial review draft.** These requested behaviors are kept separate from upstream documentation and have not been accepted by the upstream project.

[bgan source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

Review draft: the purpose is to help the author understand my wishes. Each topic explains the desired behavior, why it matters, and a concrete example. Review the wording first; videos come after the requests are settled.

[en-US](README_en-US.md) · [fs-TW](README_fs-TW.md)

[Official 3.8.0 versus bgan comparison](COMPARISON_en-US.md) · [fs-TW 比較](COMPARISON_fs-TW.md)

Compare official TaigiKeyboard 3.8.0 (taigikeyboard_3.8.0_amd64.deb) with TaigiKeyboard bgan 3.8.0, now rebased onto the official desktop-3.8.0 source. bgan remains an unofficial custom build. Basic Space confirmation and spaced POJ already work in 3.8.0; those topics describe preservation or workflow extensions.

| Topic | My wish | Details |
|---|---|---|
| 01 | Show POJ readings beside Hanji candidates | [en-US](01-candidate-readings/en-US.md) |
| 02 | Keep selected combinations editable and reopen their reading | [en-US](02-editable-selection/en-US.md) |
| 03 | Make Space accept the highlighted script and finish selection | [en-US](03-space-confirmation/en-US.md) |
| 04 | Offer a complete POJ candidate with syllable spaces | [en-US](04-poj-spaces/en-US.md) |
| 05 | Allow bo selection inside bokahi when the suffix is valid | [en-US](05-bo-prefix/en-US.md) |
| 06 | Keep fully spelled in candidate pages within the exact reading | [en-US](06-exact-in/en-US.md) |
| 07 | Toggle Han-lo and POJ-first candidate rows without bare duplicates | [en-US](07-poj-toggle/en-US.md) |
| 08 | Preserve Shift+Enter typed-pronunciation output | [en-US](08-shift-enter/en-US.md) |

## Review order

Start with 01, 03, 04, and 07: readings, Space, POJ spacing, and mode switching. Then review editing (02), search (05–06), and Shift+Enter (08). Feedback can be as simple as “Issue 03 should …”.

Videos are deferred to the final step. Old 3.6.10-based media have been archived and are not evidence for this comparison. Nothing has been submitted externally. Emoji access is a Vial shortcut setup for existing Fcitx functionality, so it is separate from these eight engine topics.
