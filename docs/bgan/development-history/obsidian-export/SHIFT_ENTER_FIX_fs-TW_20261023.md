> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# Shift+Enter 讀音輸出——2026 年 10 月 23 日修好

10 月 22 日作者比較報告內底記錄 ê Shift+Enter 退步問題，這馬已經修好。
重新 build ê addon 已經安裝，Fcitx5 嘛已經重新啟動。

| 整句模式 ê 操作 | 修改進前 | 修好了後 |
|---|---|---|
| 拍 `ho2`，閣按 Shift+Enter | 送出 `好` | 送出 `hó` |
| 拍 `ho2`、選 `好`，閣按 Shift+Enter | 猶原用選好 ê 漢字輸出 | 送出原本拍 ê 讀音 `hó` |
| 拍 `li2-ho2`，閣按 Shift+Enter | 接受辭典候選詞 | 送出 `lí-hó`，保留原本明確拍 ê hyphen |

原因是共用 ê 按鍵分類，原本共讀音送出 ê 動作，交予整句確認仝一條
`Commit` 路徑處理。彼條路徑會先接受標示 ê 辭典候選詞，才完成送出。
這馬整句模式改用 engine 既有 ê `CommitAsTyped`，直接送出所拍 ê 讀音。
TPS 佮無開整句模式 ê 操作，猶原照既有 ê 路徑。
這擺無改語音規則、候選詞排序、辭典資料、鍵盤 firmware 抑是 Vial 配置。

普通 ê Enter 佮 Space 猶原照整句選詞 ê 操作。Auto-Space 開啟 ê 時，
最後送出 ê 羅馬字後壁才會加空白。無文字咧編輯 ê 時，Shift+Enter
猶原交予應用程式處理。這擺使用既有 ê 讀音轉換，無新加共無分隔 ê
多音節輸入自動斷音節 ê 功能。

驗證結果：

- 新 ê regression test 佇修改進前失敗，修好了後通過。涵蓋 POJ／台羅、
  Auto-Space 開／關、猶未選詞、整段選詞，佮選部分詞組 ê 情形。
- 完整 desktop-core：**454 个通過**；Linux core/platform/FFI：**68 个通過**。
- Desktop-core 佮 Linux-core ê Clippy 攏通過，警告視做錯誤處理。
- 重新 build ê addon 通過 Shift+Enter、普通 Space、`bokahi`、精確 `in`，
  佮漢羅／POJ 優先切換 ê 檢查。
- 實際咧行 ê Fcitx5 驗證 (original local artifact: `logs/validation/shift-enter-running-fcitx-20261023.log`)
  確認會送出 `hó`，選詞了後嘛仝款；明確 hyphen、這馬 ê Auto-Space
  設定，佮無編輯文字時交予應用程式 ê 行為，攏有通過。
- 完整 Linux gate 猶因為離線環境欠 GTK 相依套件而中斷；其他平台
  佮 GTK 設定程式猶未驗證。

進前 ê addon 有留佇還原備份 (original local artifact: `backups/shift-enter-20261023/`)。
10 月 22 日 ê patch 佮回饋 ZIP 猶原是彼工 ê 歷史記錄，無包含這擺 ê 修正。
猶未向作者送出訊息。
