# TaigiKeyboard bgan compared with official 3.8.0

This feedback explains the interaction I wish to use. **TaigiKeyboard bgan is an unofficial custom build**, now rebased onto official desktop-3.8.0 with the bgan changes. Its version is 3.8.0-bgan.20261023 and it remains unofficial.

The official comparison package is the user-provided **taigikeyboard_3.8.0_amd64.deb**. This replaces the previous comparison against 3.6.10. Historical traces are not evidence of 3.8.0 behavior.

[en-US](COMPARISON_en-US.md) · [fs-TW](COMPARISON_fs-TW.md) · [8 topics / 八項說明](README_en-US.md)

## What I wish the author to understand

I want readings beside Hanji, an editable selection workflow, and one shortcut between Han-lo and full POJ. I also want manual bo | kah | i selection within bokahi, and candidate pages for a complete in query to remain within that reading.

Official 3.8.0 already has basic Space confirmation and spaced POJ candidates. Preserve those existing capabilities. Shift+Enter → hó works in both; the earlier regression belonged to the custom build.

## Observed behavior

| Action | Official 3.8.0 | Unofficial bgan 3.8.0 |
|---|---|---|
| ho2 candidates | 好 and hó in separate cells | 好 hó plus a separate selectable hó |
| q selects 好 | Immediately commits 好 | Keeps 好 editable; Left reopens its reading |
| ho2 + Space | Commits highlighted 好 | Also commits highlighted 好; editable partial-selection workflow is the extension |
| Syllable Separator = Space; li2ho2 | Already offers lí hó; w commits it | Also offers lí hó; w keeps it editable, then Space commits |
| bokahi | Already offers whole 無佮意; standalone 無 is absent | Allows manual 無 → 佮 → 意 selection; valid bok remains |
| in, then later candidate pages | 83 cells in this replay, including nasal iⁿ readings | 56 cells restricted to exact in with tone variants; inn remains separate |
| Backtick with li2ho2 menu open | Rows remain Hanji-first; q writes 你好 | Changes to lí hó 你好; q selects POJ; no duplicate bare lí hó row |
| Output indicator | C-ABI indicator stays 白 during that toggle | 台 for Han-lo; POJ for POJ-first |
| ho2 + Shift+Enter | Correctly commits hó | Correctly commits hó; a previous custom regression was repaired |

## Method and limits

The official .deb was extracted and replayed through its C ABI without installing it. Each build used matching dictionaries and isolated temporary settings and learning directories. Settings: POJ, Combined, Hanji first, Syllable Separator = Space, Auto-Space off; the custom editable-sentence option applies only to bgan. All four dictionary artifacts match the official 3.8.0 package byte for byte; bgan selection and presentation changes are the comparison focus.

The custom source is available in the [bgan/3.8.0 fork branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0), based on official desktop-3.8.0 commit `69751e58ba0dbe64d3512330e4580eefaff71521`. The rebased bgan build passed all 12 feature groups, 486 desktop/storage tests, and 70 Linux core/platform/FFI tests. The remaining engine run passed 1,563 tests, with five existing ignored tests and two explicitly excluded SQLite learning failures also reproduced on unmodified 3.8.0. Clippy and formatting passed. Full GTK/update workspace gates remain limited by missing development libraries.

[Replay evidence / 比較證據](evidence/README.md)

Videos remain deferred until the wording is settled. Old 3.6.10-based videos are archived and must not illustrate this comparison. Nothing has been submitted to the author.
