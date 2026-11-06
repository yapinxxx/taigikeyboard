> Historical Obsidian export, captured November 6, 2026. The original report's
> versions, test totals, and installation claims apply to its recorded revision.
> Machine-specific paths were made portable; referenced local artifacts are
> identified but are not included here. See [export provenance](README.md) and
> [current development progress](../../PROGRESS.md) before following old commands.

# bgan 3.8.0：移植佮功能確認

**狀態：這台電腦已經使用 3.8.0-bgan.20261023。** bgan ê 功能已驗證保留。這是非官方客製版，源碼這馬確實用官方 desktop-3.8.0 做基礎，毋是干焦換套件名稱。

**Upstream:** desktop-3.8.0 · `69751e58ba0dbe64d3512330e4580eefaff71521`  
**bgan branch:** bgan/3.8.0 · `b8f00d81841cc69c1f9080ce6edceef7a912b392`

## 功能驗證

| 功能 | 結果：通過 |
|---|---|
| 候選讀音 | 漢字佮 POJ 仝一格；漢羅猶有獨立 POJ 選擇 |
| 選好猶會當改 | 規句抑是較短 ê 詞攏留佇編輯區；Left 閣開讀音；重選會當寫 逐雞好 |
| Space／Enter | 照標示 ê 文字選；後壁猶有詞就繼續編輯；規句選好才送出一擺 |
| POJ 排法 | 直接選 lí hó；明確選連字號嘛有 lí-hó；Auto-Space 另外管後壁空白 |
| 較短 ê bo | bokahi → 無 → 佮 → 意；合法 ê bok 有留 |
| 精確讀音 | in 換頁嘛猶原精確；inn、b 佮 ss 猶原揣會著 |
| 漢羅／POJ | Backtick 切換會保留；標示 台／POJ；無重複 ê 獨立 POJ 格 |
| Shift+Enter | ho2 → hó；選好漢字嘛仝款；無編輯文字就交予應用程式 |
| 其他操作 | POJ／TL、Romanization Only、取消佮符號選擇器檢查通過 |

官方 3.8.0 ê 四个辭典檔案攏比對仝款。上游這馬會共數字聲調顯示做聲調記號，親像 tak8keho2 顯示 ta̍kkehó；bgan 保留這个上游顯示，讀音游標佮重選功能嘛通過。

## 測試範圍佮限制

- 12 組 bgan 功能檢查通過，包含實際安裝路徑 ê binary。
- 486 个 desktop／storage、70 个 Linux core／platform／FFI 測試通過。
- Engine 剩落 ê 1,563 个測試通過；5 个原有測試略過。
- 2 个 SQLite 學習測試明確排除；無修改 ê 官方 3.8.0 嘛佇仝款 assertion 失敗，毋是共失敗藏起來。
- Engine、desktop core／storage、Linux core／platform／FFI ê Clippy，佮三組格式檢查通過。
- 獨立 D-Bus／Fcitx5 daemon ê 三支整合檢查攏通過，無佮使用者咧拍 ê 字相摻。
- 完整 GTK／更新程式 workspace gate 欠 OpenSSL、GLib／GTK 開發函式庫；設定程式使用官方 3.8.0 binary。其他實體電腦 ê installer 猶未驗證。

`literal_picks_reuse_existing_keys_and_keep_tone_boundaries_distinct`  
`the_engine_writes_what_the_platforms_wrote`

Validation receipts (original local artifact: `issues/taigikeyboard-20261008/logs/validation/bgan-380-final-validation.json`)

## 後擺按怎接上游更新

bgan ê 修改這馬佇獨立 branch，3 个 commit 疊佇官方 release 頂面。下擺有欲用 ê正式 tag，會當用這个工具另開 worktree、rebase、編譯佮檢查；袂隨便取代這馬會用 ê 版本。

```bash
cd legacy-workspace/taigikeyboard-bgan-3.8.0
python3 tools/desktop/rebase-bgan.py desktop-<next-version>
```

`desktop-<next-version>` 愛換做實際存在 ê 上游 tag。若有 conflict 抑是功能檢查毋過，就停落來，這馬 ê 安裝袂變。這台開發環境需要保留原來 ê protoc、CMake 佮 Fcitx5 headers 設定。

若欲重驗這馬 ê 3.8.0，確認接受這兩个已重現 ê 上游 SQLite 限制，才明確用下面 ê 參數。這个例外干焦適用 desktop-3.8.0；下个 release 愛重新評估。

```bash
python3 tools/desktop/verify-bgan.py --allow-known-upstream-sqlite-tests
```

Rebase 會減少重做 ê 工，毋過客製功能猶是需要檢查；除非作者納入上游，否則袂保證完全免維護。官方套件更新嘛袂自動更新使用者目錄內底 ê bgan override。這馬無推送 branch 抑是向作者送出 issue。

## 套件佮備份

bgan 3.8.0 archive (original local artifact: `local-downloads/taigikeyboard-bgan-3.8.0-20261023-ubuntu24.04-amd64.tar.gz`)
Before-activation backup (original local artifact: `issues/taigikeyboard-20261008/backups/bgan-3.8.0-activation-20261023-142715`)

原來 ê 設定有保留，學習資料庫已經先做一致性備份。舊 bgan 3.6.13 binary 佮舊源碼目錄留做還原用；下擺維護愛用 bgan/3.8.0 ê 新 worktree。

[en-US](BGAN_REBASE_3.8.0_en-US_20261023.md) · [fs-TW](BGAN_REBASE_3.8.0_fs-TW_20261023.md) · [Author overview](../../author-feedback/2026-10-23/README_fs-TW.md)
