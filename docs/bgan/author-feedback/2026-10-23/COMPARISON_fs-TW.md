# TaigiKeyboard bgan：佮官方 3.8.0 ê 比較

這份回饋想欲予作者了解我想欲 ê 操作。**TaigiKeyboard bgan 是非官方客製版**，這馬用官方 desktop-3.8.0 加客製修改，版本是 3.8.0-bgan.20261023，猶原毋是官方 release。

官方比較套件是使用者提供 ê **taigikeyboard_3.8.0_amd64.deb**。進前用 3.6.10 做比較 ê 文件已經由這份取代；毋通共舊比較講做 3.8.0 ê 行為。

[en-US](COMPARISON_en-US.md) · [fs-TW](COMPARISON_fs-TW.md) · [8 topics / 八項說明](README_fs-TW.md)

## 欲予作者了解 ê 重點

我想欲漢字佮 POJ 讀音列佇仝一格，保留會當修改 ê 選詞流程，閣會當一鍵切換漢羅佮全 POJ。拍 bokahi 時嘛想欲家己分 bo、kah、i 來選；拍完整 in，換頁嘛愛留佇 in ê 讀音範圍。

基本 Space 確認佮空白分音節 ê POJ，官方 3.8.0 已經有。這兩項愛保留，毋通講做 bgan 才新加。Shift+Enter 送出 hó 兩爿攏有；進前退步是客製版 ê 問題，毋是官方 bug。

## 實際觀察

| 操作 | 官方 3.8.0 | 非官方 bgan 3.8.0 |
|---|---|---|
| ho2 候選詞 | 好 佮 hó 分開兩格 | 好 hó，閣有會當獨立選 ê hó |
| q 選 好 | 隨時送出 好 | 好 留佇編輯區；Left 閣開讀音 |
| ho2 + Space | 已經送出標示 ê 好 | 嘛送出 好；延伸 ê 部分是前段選好猶會當繼續編輯 |
| Syllable Separator = Space；li2ho2 | 已經有 lí hó；w 隨時送出 | 嘛有 lí hó；w 先選起來，Space 才送出 |
| bokahi | 已有規个 無佮意；無單獨 ê 無 | 會當 無 → 佮 → 意 分開選；合法 ê bok 猶原有 |
| in，閣換到後壁 ê 頁 | 這擺 83 格，包含 iⁿ 鼻化讀音 | 56 格，限制完整 in，保留聲調；inn 另外揣 |
| li2ho2 清單開咧，按 Backtick | 猶是漢字優先；q 送出 你好 | 改 lí hó 你好；q 選 POJ；免重複列單獨 ê lí hó |
| 狀態標示 | 這擺 C ABI 切換前後攏是 白 | 漢羅用 台；POJ 優先用 POJ |
| ho2 + Shift+Enter | 正確送出 hó | 正確送出 hó；進前客製退步問題已修好 |

## 比較方式佮限制

官方 .deb 干焦解開做 C ABI 重播，無安裝取代這馬 ê 輸入法。兩爿各用配套辭典、暫存設定佮學習目錄。設定 POJ、Combined、漢字優先、Syllable Separator = Space、Auto-Space 關閉。干焦 bgan 有開客製整句編輯。四个辭典檔案佮官方 3.8.0 完全仝款；選詞佮顯示差別是 bgan 欲討論 ê 操作。

客製源碼佇 [bgan/3.8.0 fork branch](https://github.com/yapinxxx/taigikeyboard/tree/bgan/3.8.0)，基礎是官方 desktop-3.8.0 commit `69751e58ba0dbe64d3512330e4580eefaff71521`。bgan 通過 12 組功能檢查、486 个 desktop／storage、70 个 Linux core／platform／FFI 測試。Engine 閣通過 1,563 个測試；5 个原有測試略過，2 个 SQLite 學習測試明確排除，這兩个佇無修改 ê 官方 3.8.0 嘛有仝款失敗。Clippy 佮格式檢查通過。完整 GTK／更新程式 gate 猶欠開發函式庫。

[Replay evidence / 比較證據](evidence/README.md)

影片留到文字確認了後。舊 3.6.10 影片已經收做歷史備份，毋通當做這份比較。猶未對作者送出。
