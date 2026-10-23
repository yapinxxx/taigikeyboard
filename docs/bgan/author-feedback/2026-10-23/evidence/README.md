# Official 3.8.0 versus TaigiKeyboard bgan: evidence

The official baseline is **taigikeyboard_3.8.0_amd64.deb**, extracted without installing it. The custom build is **TaigiKeyboard bgan**, an unofficial custom build now rebased onto desktop-3.8.0, version 3.8.0-bgan.20261023.

官方基準是 **taigikeyboard_3.8.0_amd64.deb**，干焦解開做測試，無安裝取代這馬 ê 輸入法。**TaigiKeyboard bgan** 這馬已經移植到官方 desktop-3.8.0，版本是 3.8.0-bgan.20261023，猶原是非官方客製版。

Both binaries were replayed through their C ABI with isolated temporary settings and learning directories. Both now use **byte-identical official 3.8.0 dictionary files**. Settings: POJ, Combined, Hanji first, literal candidate off, Auto-Space off, Syllable Separator = Space; only bgan recognizes the custom editable-sentence option. Dictionary parity is verified by SHA-256. Source-patch comparisons now use the official desktop-3.8.0 source tag.

| File | Meaning |
|---|---|
| [original.json](original.json) | Actual official 3.8.0 package replay |
| [current.json](current.json) | Actual current bgan behavior replay |
| [original-boundary-demo.json](original-boundary-demo.json) | Official 3.8.0 shorter-prefix and in/inn queries |
| [current-boundary-demo.json](current-boundary-demo.json) | bgan manual 無 → 佮 → 意 selection and exact in pages |
| [provenance](provenance-3.8.0-bgan.json) | Versions, settings, method and official package checksum |

These JSON files record the isolated C-ABI comparison results. The corresponding custom implementation is available in the [bgan/3.8.0 source branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0).

Important corrections: official 3.8.0 already sends 好 for ho2 + Space and offers lí hó with the Space separator. It also already offers the whole 無佮意 phrase for bokahi; the remaining wish concerns the standalone shorter 無 choice. Official 3.8.0 and bgan both send hó for ho2 + Shift+Enter.

官方 3.8.0 拍 ho2、按 Space 已經送出 好，Space 分音節嘛已有 lí hó。拍 bokahi 已經有規个 無佮意；猶欠 ê 是會當獨立選較短 ê 無。兩版本拍 ho2、按 Shift+Enter 攏送出 hó。

Videos are deferred. The old 3.6.10-based videos have been archived; they must not be presented as a 3.8.0 comparison. No external report has been submitted. No learning database or live input configuration was changed by these replays.

## Rebase verification

bgan is now 3.8.0-bgan.20261023, based on desktop-3.8.0. All 12 feature groups passed using the installed addon and official dictionaries. Private Fcitx5 daemon checks passed for selections, editing, Space, mode switching, symbol picker, Enter and Shift+Enter. Earlier desktop-context attempts mixed with active typing; the isolated-daemon results are the final integration evidence.

The broad engine run has two known SQLite learning test failures also reproduced on unmodified official 3.8.0. The remaining engine suite passed 1,563 tests; desktop/storage passed 486, Linux core/platform/FFI passed 70. See the validation manifest for limitations.
