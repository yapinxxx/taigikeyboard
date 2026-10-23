# TaigiKeyboard bgan：佮官方 3.8.0 比較我想欲 ê 操作

**狀態：個人整理 ê 非官方審查草稿。** 這寡操作建議佮 upstream 文件分開放；上游專案猶未接受。

[bgan 源碼 branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)

這份先予我確認作者看了會了解我想欲啥物。每項分開寫欲按怎用、為啥物、佮一个例。這馬先看文字，影片留到內容確認了後才處理。

[en-US](README_en-US.md) · [fs-TW](README_fs-TW.md)

[Official 3.8.0 versus bgan comparison](COMPARISON_en-US.md) · [fs-TW 比較](COMPARISON_fs-TW.md)

這份比較官方 TaigiKeyboard 3.8.0（taigikeyboard_3.8.0_amd64.deb）佮 TaigiKeyboard bgan。bgan 這馬已經移植到官方 desktop-3.8.0 源碼頂面，版本是 3.8.0-bgan.20261023，猶原是非官方客製版。3.8.0 已經有基本 Space 確認佮有空白 ê POJ；這兩項是保留操作抑是延伸整句編輯。

| 項目 | 我想欲啥物 | 分項說明 |
|---|---|---|
| 01 | 候選漢字邊仔顯示 POJ 讀音 | [fs-TW](01-candidate-readings/fs-TW.md) |
| 02 | 選好 ê 詞組猶會當修改，閣開原來 ê 讀音 | [fs-TW](02-editable-selection/fs-TW.md) |
| 03 | Space 接受標示 ê 候選詞，選好才送出 | [fs-TW](03-space-confirmation/fs-TW.md) |
| 04 | 規个 POJ 候選詞直接用空白分音節 | [fs-TW](04-poj-spaces/fs-TW.md) |
| 05 | 拍 bokahi 時，會當先選 bo ê 無 | [fs-TW](05-bo-prefix/fs-TW.md) |
| 06 | 完整拍 in，換頁猶原揣 in ê 讀音 | [fs-TW](06-exact-in/fs-TW.md) |
| 07 | 切換漢羅佮 POJ 優先，免重複列 POJ | [fs-TW](07-poj-toggle/fs-TW.md) |
| 08 | Shift+Enter 保留送出所拍讀音 ê 功能 | [fs-TW](08-shift-enter/fs-TW.md) |

## 這擺先確認啥物

先看 01、03、04、07，確認讀音顯示、Space、POJ 空白佮切換模式。閣看 02 ê 編輯操作、05–06 ê 搜尋、08 ê Shift+Enter。會當直接講「第 03 項應該……」，我就照彼項改。

影片留到最後一步。這馬 ê 文字稿無以影片做審查重點，嘛猶未對外送出。Emoji 是既有 Fcitx 功能配合 Vial 按鍵 ê 設定，毋是 TaigiKeyboard engine 新功能，所以無列做這八項之一。
