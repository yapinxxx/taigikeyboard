> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# 使用 TaigiKeyboard 時 ê emoji 輸入

**日期：**2026 年 10 月 22 日 · **平台：**Linux / Fcitx5  
**另外一个語言版本：**[en-US](EMOJI_ACCESS_en-US_20261022.md)

這馬使用 TaigiKeyboard ê 時，會當用兩个較順手 ê 按鍵組合揣 emoji。
這是透過鍵盤 ê Vial 配置，使用 Fcitx5 本來就有 ê Unicode 佮 Quick Phrase
addon。這擺無佇 TaigiKeyboard 新加 emoji engine，嘛無修改 TaigiKeyboard
源碼抑是 Fcitx ê shortcut 設定。

## 修改進前佮這馬 ê 差別

連接 ê 鍵盤是 **Pragmatic 60 34-key OS-aware**，用 Dvorak 配置，共五層。
Enter 拇指鍵按牢就會開 Layer 4。

| 按法 | 修改進前 | 這馬 ê 功能 |
|---|---|---|
| Enter 按牢，閣拍 E | Layer 4 是透明鍵，沿用基本層 ê E | 送出 Ctrl+Alt+Shift+U，開 Fcitx Unicode 搜尋 |
| Enter 按牢，閣拍 O | Layer 4 是透明鍵，沿用基本層 ê O | 送出 Super+semicolon，開 Fcitx Quick Phrase |

這兩个 Fcitx shortcut 本來就有。頭先建議用 Super+backtick 開 Quick Phrase，
毋過會佮 GNOME ê 視窗切換相衝突。後來讀實際咧用 ê Fcitx 設定，確認
Super+semicolon 嘛是既有 ê 開啟方式，所以免閣改 Fcitx 設定。

## 平常按怎用

先共猶咧編輯 ê 台語送出，才開 emoji 選擇。

- **Unicode 搜尋：**Enter 按牢，拍 E，閣共按鍵放開。拍描述抑是完整 ê
  codepoint，親像 `U+1F600`，閣選 😀。描述搜尋是揣規个 Unicode，
  所以結果嘛可能有漢字，毋是干焦 emoji。
- **Quick Phrase：**Enter 按牢，拍 O，閣共按鍵放開。拍 `:smile:`，
  用 **1** 選候選詞，就會輸入 😄。

Vial 會共頭一个組合顯示做 **Meh+U**；Meh 就是 Ctrl+Alt+Shift。
第二个是 **LGui+semicolon**；佇 Linux，LGui 就是左爿 ê Super／Windows 鍵。
這兩个攏是直接指定 ê 按鍵組合，無使用錄製 ê macro。

## 修改佮驗證

先用一般使用者權限，透過 Vial ê vendor interface 讀取配置，閣修改兩格。
無使用 sudo、無解鎖，嘛無重燒 firmware。

| 位置，對 0 開始算 | Keycode | 意思 |
|---|---|---|
| Layer 4，row 2，column 3 | `0x0718` | Ctrl+Alt+Shift+U |
| Layer 4，row 2，column 2 | `0x0833` | Super+semicolon |

寫入進前有重新讀實際配置做備份。寫入了後，完整讀轉來比較，閣另外讀
一擺確認，干焦這兩格有改變；其他按鍵、macro、tap dance、combo、
key override 佮設定攏佮備份仝款。

用 Fcitx 私用 input context 測試，Unicode 搜尋成功送出 😀，Quick Phrase
成功送出 😄。這款測試驗證 ê 是送出 ê shortcut，無模擬實際按拇指鍵；
後來使用者有確認，佇實體鍵盤頂懸嘛有成功。

## 備份佮記錄

- 加 emoji 按鍵進前 ê 配置 (original local artifact: `backups/vial-emoji-keys-20261022/before.vil`)
- 使用者確認正常了後，重新備份 ê 配置 (original local artifact: `backups/vial-working-20261022-032801/pragmatic-working-20261022-032801.vil`)
- 正常配置備份 ê 資料佮 checksum (original local artifact: `backups/vial-working-20261022-032801/backup-metadata.json`)
- 驗證記錄 (original local artifact: `backups/vial-emoji-keys-20261022/fcitx-verification.log`)
- 修改佮兩鍵還原 script (original local artifact: `scripts/set-vial-emoji-keys.py`)

這兩个 `.vil` 檔攏會當用 Vial 載入。頭一个會還原做修改進前 ê 配置；
第二个會保留已經確認正常 ê emoji 按鍵。兩款備份攏有原始配置記錄。

若欲回饋予作者，這是 TaigiKeyboard 配合 Fcitx 既有功能 ê 使用例。
會當考慮佇上游文件補充 emoji 用法佮 shortcut 衝突 ê 說明。
實體按鍵位置是照這把鍵盤安排，無建議共伊當做全部使用者通用 ê 綁定。

參考：[Fcitx 特殊字元輸入](https://fcitx-im.org/wiki/Tips_and_Tricks)、
[Vial 按鍵組合設定](https://get.vial.today/manual/any-key.html)。
猶未向作者送出報告抑是訊息。
