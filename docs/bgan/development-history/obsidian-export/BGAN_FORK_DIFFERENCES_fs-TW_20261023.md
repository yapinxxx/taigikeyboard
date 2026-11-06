> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# bgan 佮官方 TaigiKeyboard 3.8.0 ê 差別

這份比較想欲予上游作者了解我想欲啥物操作，閣有具體 ê 例會當討論。

**官方基準：**`desktop-3.8.0`，`69751e58ba0dbe64d3512330e4580eefaff71521`。  
**這馬咧用 ê 客製版：**`3.8.0-bgan.20261023`，源碼 `b8f00d81841cc69c1f9080ce6edceef7a912b392`，branch `bgan/3.8.0`。

[總覽](BGAN_FORK_OVERVIEW_en-US_20261023.md) · [en-US](BGAN_FORK_DIFFERENCES_en-US_20261023.md)

| 主題 | 官方 3.8.0 | 我想欲 ê bgan 操作 |
|---|---|---|
| 漢字邊仔 ê 讀音 | Combined 分開列 `好` 佮 `hó` | 仝一格顯示 `好 hó`，漢羅嘛保留獨立 ê `hó`；選漢字干焦送出漢字。 |
| 選好猶會當改 | 按 `q` 就隨時送出 `好` | 規句抑是較短 ê 詞先留佇編輯區。Left 閣開原來 ê 讀音，送出進前猶會當重選。每頁六鍵：`q w d f z x`。 |
| Space 確認 | 拍 `ho2`、按 Space 本來就會送出標示 ê `好` | 保留這款操作，閣延伸到整句編輯：前段選好、後壁猶有詞就繼續選；最後 Space／Enter 才送出。 |
| 有空白 ê POJ | Syllable Separator = Space 本來就有規个 `lí hó` | 保留會當直接選 ê `lí hó`，閣會當先選起來、後來才送出。明確選連字號嘛有 `lí-hó`。Auto-Space 另外管送出後壁 ê 空白。 |
| 較短 ê `bo` | `bokahi` 已經有規个 `無佮意`，毋過這擺清單無單獨 ê `無` | 閣予我照 `無 → 佮 → 意` 分開選，合法 ê `bok` 猶原有留。這是候選前段 ê 選擇，毋是改拼音規則。 |
| 精確揣 `in` | 換到後壁幾頁有 `iⁿ` 鼻化讀音佮其他延伸 | 完整 ê `in` 留佇仝款讀音，聲調變化有保留。`inn` 另外揣；猶未完整 ê `b` 佮縮寫 `ss` 嘛猶原有用。這擺觀察是官方 83 格、bgan 56 格，毋是固定數量。 |
| 漢羅／全 POJ 切換 | 這擺 Backtick 無改 Combined 候選順序 | 漢羅有 `q 你好 lí hó` 佮 `w lí hó`。Backtick 改做 `q lí hó 你好`，干焦送出 `lí hó`，免閣列重複 ê 純 POJ。切換會保留；標示用 `台`／`POJ`。 |
| Shift+Enter | `ho2` 本來就送出 `hó` | 保留這个功能，選好漢字嘛仝款。進前客製退步問題已修好，毋是官方 3.8.0 ê bug。 |
| 符號選擇器 | 原本就有 | 開外部符號選擇器進前，先完成編輯中 ê 選詞；取消嘛袂共狀態弄亂。 |

## 客製修改 ê 範圍

`editableSentenceEnabled` 是選用設定，預設關閉。Shared engine 管接受 ê 詞段、讀音游標佮精確查詢；desktop core 管候選顯示佮按鍵；Linux core 共六鍵標示、換頁佮選詞位置對齊。無仝 ê 漢字／canonical 讀音組合猶原分開，無合做伙。

四个辭典檔案佮官方 3.8.0 完全仝款。上游這馬共數字聲調顯示做聲調記號，親像 `tak8keho2` 顯示 `ta̍kkehó`；bgan 保留這个顯示，原來 ê 讀音猶會當修改。無改音節庫抑是基礎分音節規則。

Emoji 是既有 Fcitx 功能配合 Vial 按鍵，毋是這份源碼新加 ê emoji engine。Threads ê 字型／空白觀察嘛無當做已確認 ê 鍵盤 bug。影片留到文字確認了後才處理。

## 驗證佮限制

- **12 組功能檢查攏通過**，包含實際安裝路徑 ê addon、重選出 `逐雞好`、`bo → kah → i`、精確換頁、POJ 切換佮 Shift+Enter。
- **486 个 desktop／storage**、**70 个 Linux core／platform／FFI** 測試通過。
- Engine 剩落 ê **1,563 个測試通過**；**5 个原有測試略過**，**2 个 SQLite 學習測試明確排除**。
- 這兩个佇無修改 ê **官方 desktop-3.8.0** 嘛佇仝款 assertion 失敗：`literal_picks_reuse_existing_keys_and_keep_tone_boundaries_distinct` 佮 `the_engine_writes_what_the_platforms_wrote`。失敗有記錄，毋是當做通過。
- Engine、desktop core／storage、Linux core／platform／FFI ê Clippy，佮 Rust 格式檢查通過。
- 三支獨立 Fcitx5 整合檢查通過：選詞、修改、Space、模式、符號選擇器、Enter 佮 Shift+Enter。
- 完整 GTK／更新程式 gate 佇本機欠開發函式庫；設定程式用官方 3.8.0 binary。其他平台 shell 佮第二台實體電腦 ê installer 猶未驗證。

維護 branch 另外整理 Python 格式、明確對 `upstream` remote 揣官方更新，閣加 bgan CI 佮這份公開比較。這寡佮「這馬咧用 ê 源碼快照」有分開。GitHub Actions 有實際結果才會當講 CI 通過；干焦有 workflow 檔案猶毋是驗證。

## 後擺更新

`python3 tools/desktop/rebase-bgan.py desktop-<version>` 會另開 worktree、疊客製修改佮檢查。Conflict 抑是功能毋過就停落來，這馬會用 ê 安裝留咧。`verify-bgan.py` 做編譯佮驗證；3.8.0 若明確接受頂面 ê SQLite 限制，才用 `--allow-known-upstream-sqlite-tests`，下个 release 愛重新評估。

Rebase 會減少重做 ê 工，毋過客製功能猶原需要檢查；除非作者納入上游，否則袂保證完全免維護。這份差異會當提予作者建議，無自動向官方開 issue 抑是 PR。
