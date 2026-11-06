> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# TaigiKeyboard 使用回饋：原版佮在地修改版 ê 比較

**日期：** 2026 年 10 月 21 日  
**語言：** fs-TW／台文 · [en-US／英文版](AUTHOR_COMPARISON_en-US_20261021.md)  
**平台：** Linux、Fcitx5

## 欲共作者講 ê 重點

我希望會當選一个參考意傳（ì-thoân）ê 整句編輯模式：選規个詞組抑是較短
ê 組合，用字母鍵選好 ê 文字猶會當修改，閣會當轉去原來 ê 讀音重新選詞。
Space 用來接受這馬標示 ê 候選詞。我嘛希望佇「漢羅濫」ê 漢字候選詞邊仔
看著 POJ 讀音，閣會當一擺就選規个 `lí hó`，免分開選 `lí` 佮 `hó`。

我已經試用這个在地修改版，這馬決定繼續用這版做平常拍字。
這是一份欲討論操作方式 ê 試作，猶有下面列出 ê 問題；希望新模式會當予人
家己選，嘛保留原來 ê 操作方式。

## 1. 版本佮比較方式

| 項目 | 版本抑是識別資料 |
|---|---|
| 系統原來安裝 ê 套件 | 無修改 ê **3.6.10** |
| 在地修改所根據 ê 源碼 | **3.6.13**，commit [`3bdd7ccb2e953f002e536e89a59b3a69eae74018`](https://github.com/taigikeyboard/taigikeyboard/commit/3bdd7ccb2e953f002e536e89a59b3a69eae74018)，日期 2026-10-07 |
| 這馬 ê 在地版 | 頂懸 ê 源碼加附件內 ê patch，安裝做使用者專用 ê Fcitx5 addon |
| 參考 ê 操作 | 已經安裝 ê 意傳 `pojhan` 方案，用系統 ê librime 佇獨立暫存環境實際試鍵 |
| 這馬使用 ê 模式 | POJ、漢羅濫、開啟整句編輯、音節分隔選 Space、自動空白關閉 |

下面 ê **實際執行比較，是系統原版 3.6.10 佮這馬 ê 在地版比較**。
另外，源碼 patch 是對指定 ê 3.6.13 commit 做比較。這兩種比較無仝：
兩个安裝版本 ê 差別，袂使攏算做這份 patch 所造成 ê 改變。

比較 ê 時，兩版攏透過仝款 ê C ABI 試鍵，使用各自獨立 ê 暫存設定佮學習
資料目錄，嘛使用佮各自版本相配 ê 已安裝辭典。控制設定是 POJ、漢羅濫、
漢字排頭前、關閉「優先顯示輸入文字」、關閉 Auto-Space，閣開啟舊 ê
No Hyphens 開關。新的整句編輯旗標干焦在地版會認得。
這个比較無改著實際使用中 ê 輸入法狀態抑是學習資料庫。
無共兩版 ê 辭典內容佮排序當做仝款來驗證。

## 2. 實際行為比較

| 操作抑是功能 | 系統原版 3.6.10 | 這馬 ê 在地版 |
|---|---|---|
| 佇漢羅濫拍 `ho2` | 頭前兩格是 `好`、`hó` | 頭前兩格是 `好 hó`、`hó`；漢字邊仔有讀音 |
| 用 `q` 選規个 `好` | 隨送出 `好` 予應用程式 | `好` 猶留佇輸入法 ê 編輯區，猶未送出 |
| 頂一个 `q` 選好矣，閣按 Left | 原來 ê 選詞已經送出；Left 交予應用程式處理 | 佇輸入法內底重新開原來 ê 讀音佮候選詞 |
| 拍 `ho2`，標示頭一个漢字候選詞，按 Space | 輸出**另外一个文字形式** `hó` | 接受**這馬標示 ê 文字形式** `好`；選詞完成就送出 |
| 共 POJ `hó` 標示起來，閣按 Space | Space 照原來規則選另外一个文字形式 | 接受 `hó`；若無閣一个候選清單，就送出 |
| 用 Space 選較短 ê 詞組 | 用原來 ê 另一種文字輸出規則 | 接受標示 ê 組合；若猶有下一段候選詞，就繼續編輯 |
| Standard 模式每頁 ê 選詞鍵 | 九格：`q w d f z x v y ;` | 六格：`q w d f z x`；換頁、滑鼠點選佮鍵盤位置攏對齊 |
| 開 No Hyphens，拍 `li2ho2` | 提供 `你好`、`líhó` | 提供 `你好 lí hó`，閣有規个 `lí hó` 候選詞 |
| 用 `w` 選頂一个 POJ 候選詞 | 送出 `líhó` | `lí hó` 猶會當編輯；Space 抑是 Enter 才送出 |
| **拍 `ho2`，按 Shift + Enter** | 送出照所拍讀音轉出來 ê `hó` | **這馬會送出 `好`；這是在地版猶未修 ê 退步問題** |

詳細 ê 實際輸出記錄：
原版記錄 (original local artifact: `issues/taigikeyboard-20261008/logs/validation/author-original-3.6.10-20261021.json`)、
在地版記錄 (original local artifact: `issues/taigikeyboard-20261008/logs/validation/author-current-20261021.json`)。

### 上游原本就有 ê 功能

POJ／台羅輸入、漢字佮羅馬字候選詞、選另外一个文字形式、漢羅對應，佮
自動空白，本來就是 TaigiKeyboard ê 功能。指定 ê 3.6.13 源碼嘛已經有
**Syllable Separator = Space**。所以 `lí hó` 這款格式，較適合看做使用者
選擇既有 ê 分隔方式，毋是新加 ê 語音規則。

為著符合這个使用偏好，在地版共舊 No Hyphens 設定 ê 轉換，對 `None`
改做 `Space`。這會改著舊開關 ê 意思，毋適合無分別就予全部使用者套用。
若納入上游，較好是予使用者明確選擇分隔方式。

## 3. 按怎重現這馬 ê 操作

設定 POJ、漢羅濫、開啟整句編輯，音節分隔選 Space。
下面 ê 例攏假設「優先顯示輸入文字」是關閉 ê。

1. **直接選規个 POJ 候選詞：**拍 `li2ho2`，用 `w` 一擺選規个 `lí hó`，
   閣按 Space 抑是 Enter。應用程式收到 `lí hó`。免分開選兩个字。
2. **接受標示 ê 候選詞：**拍 `ho2`，按 Space，會送出 `好`。
   若欲送出 `hó`，拍 `ho2`，按 Down 共 POJ 候選詞標示起來，閣按 Space。
3. **選較短 ê 詞組：**拍 `tak8keho2`。佇漢羅濫按 Down 兩擺，會標示
   較短 ê 漢字組合 `逐家`。Space 接受伊，閣留 `ho2` 予你編輯。
   閣一擺 Space 會接受 `好`，送出 `逐家好`。
4. **送出進前改選：**拍 `tak8keho2`，按 `q` 選 `逐家好`。
   Left 會重新開原來 ê 讀音；按 Left 三擺，會到 `tak8ke` ê 後壁。
   選較短 ê `逐`，閣共 `ke` 選做 `雞`。按 End 到猶未選 ê `ho2`，
   選 `好`，最後送出 `逐雞好`。

用字母鍵選詞了後，文字猶會當修改。
**Space 接受這馬標示 ê 候選詞；候選清單結束，就共文字送出。**
Enter 會接受猶咧作用 ê 候選詞，閣共規句送出。
若無咧組字，Space 就是應用程式平常 ê 空白鍵。
若開啟 Auto-Space，羅馬字最後送出 ê 時會加一个尾後 ê 空白；
漢字輸出袂自動加。這佮詞內底 ê 音節分隔，是兩項無仝 ê 設定。

意傳 ê 操作有實際驗證，包含選部分詞組、送出漢字佮 POJ：
獨立參考環境 ê 記錄 (original local artifact: `issues/taigikeyboard-20261008/logs/validation/ithuan-space-reference-20261021.log`)。
兩套輸入法 ê 辭典佮排序猶是無仝；TaigiKeyboard 無內嵌 librime。

## 4. Patch 內底 ê 修改範圍

- 加一个會當選擇 ê `editableSentenceEnabled`，預設關閉，對桌面設定一路
  傳予引擎 ê 設定協定。
- 選好 ê 組合留佇輸入法內底；讀音游標決定候選詞佮排序所看 ê 範圍，
  後壁 ê 輸入會保留，嘛會當重新開讀音改選。較長 ê 讀音範圍排較頭前。
- 每頁六个選詞鍵。佇整句模式，Left／Right 改做移動讀音游標；
  Up／Down 佮換頁鍵繼續用來揀候選詞。
- 漢羅濫 ê 漢字格加讀音，予仝漢字、無仝讀音 ê 顯示組合會當分清楚。
  獨立 ê POJ／台羅候選詞猶原留咧。
- Space 用專用 ê 整句確認動作。Space 佮 Enter 共用最後送出 ê 處理，
  自動空白照實際最後輸出 ê 文字形式來決定。
- 附帶修理：開符號表進前先共整句送出；按牢 script-swap／反引號鍵 ê 時，
  一擺按落去干焦切換一擺。這兩項適合佮新操作方式分開審查。

Patch 嘛包含這擺在地修改所加 ê 組字內空白支援。
舊分隔設定轉換、字詞讀音轉換，佮按牢鍵 ê 修理，無全部受新 ê 整句開關
控制。這是一个在地試作；毋表示關掉新開關，逐項行為就攏佮原版仝款。

## 5. 驗證佮限制

| 檢查 | 結果佮證據 |
|---|---|
| 最後一版 desktop-core 測試 | **453 項通過** |
| 最後一版 Linux core／platform／FFI 測試 | **64 項通過**；佮 desktop-core 合計 **517 項** |
| Clippy | desktop-core 佮頂懸三項 Linux crate，攏通過將 warning 當 error ê 檢查 |
| 已安裝 ê addon | 驗證漢羅濫佮純羅馬字、No Hyphens 開／關、直接 POJ 選詞、部分／最後 ê Space 操作，佮開啟自動空白 |
| 實際咧行 ê Fcitx5 | 用私用輸入環境驗證標示漢字／POJ、選部分詞組、送出已選好 ê 文字、重新開讀音、直接選 `lí hó`，佮符號表 |
| 原版佮在地版比較 | 新錄兩个已安裝版本 ê 獨立 C ABI 試鍵記錄 |

較早 ê composing 引擎測試記錄有 655 項通過，另兩項可選測試略過。
最後一擺調整引擎了後，有閣跑兩項整句模式 ê 指定測試。
這是補充 ê 驗證歷史，毋是為著這份報告重新跑規套引擎測試。
猶未完成跨平台發行所需要 ê 全部驗證。

較大範圍 ê Linux 檢查，進前因為 GTK 設定程式需要 ê 套件下載袂落來而
中斷。GTK 設定程式、其他平台外殼，佮規台電腦重新開機，攏無完成驗證。
較早 ê 大範圍測試，嘛有佇無修改 ê 上游重現 dispatch 使用者資料測試失敗；
這袂使當做這个功能新造成 ê 缺失。這份報告無宣稱效能抑是記憶體有改善。

## 6. 若欲納入上游，猶愛處理 ê 項目

1. **恢復獨立 ê 原樣輸出動作。**整句模式內，Shift+Enter 這馬佮 Enter
   行仝一條候選詞確認路徑。附件 ê 實際比較有重現這个退步問題。
   這个平常咧用 ê 在地版猶未修伊。
2. **共設定介面佮快捷鍵說明對齊。**已安裝 ê 設定程式是 3.6.10，addon
   卻是較新 ê 在地版。整句模式 ê Space 取代原來「另一種文字」ê 動作；
   開啟整句模式嘛猶愛透過設定鍵，猶未有完成 ê 介面開關。
3. **予分隔偏好明確。**較好是提供既有 ê Space 分隔選項，毋是共舊
   No Hyphens 設定 ê 轉換意思對全部使用者攏改掉。
4. **審查共用引擎佮各平台行為。**各平台外殼攏愛建置佮驗證，閣愛看學習
   資料、讀音範圍佮附帶修改 ê 行為；原樣輸出快捷鍵嘛愛修好，才適合當做
   會當發行 ê contribution。

希望上游會當提供一个**由使用者家己選擇 ê 整句編輯操作**，佮看會清楚 ê
候選詞讀音，同時保留原來 ê 操作方式。

## 附件

- 對指定 3.6.13 源碼做 ê patch (original local artifact: `issues/taigikeyboard-20261008/artifacts/taigikeyboard-local-changes-20261021.patch`)
- 原版佮在地版 ê 實際比較程式 (original local artifact: `issues/taigikeyboard-20261008/scripts/compare-taigi-original-current.py`)
- 已安裝意傳 ê 獨立試鍵程式 (original local artifact: `issues/taigikeyboard-20261008/scripts/check-ithuan-space-reference.py`)
- 最後一版實際服務 ê 驗證記錄 (original local artifact: `issues/taigikeyboard-20261008/logs/validation/ithuan-space-running-fcitx-20261021.log`)
- 驗證清單 (original local artifact: `issues/taigikeyboard-20261008/artifacts/author-feedback-validation-20261021.json`)

源碼修改猶留佇本機，猶未 commit。無共報告、issue 抑是訊息寄予作者。
附件包干焦收報告、源碼修改佮測試證據；無收執行檔、辭典抑是私人的學習資料。
