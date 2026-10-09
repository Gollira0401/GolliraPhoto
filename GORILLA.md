# GORILLA.md — 派生版の管理ルール

GorillaPhotoは [storytold/photocraft](https://github.com/storytold/photocraft) の派生版です。本家の更新を安全に取り込めるよう、**改造は1か所に集めて、本家のファイルには最小限の「つなぎ目」だけを入れる**方針で作っています。

## 1. 改造の置き場所

| 置き場所 | 中身 |
|---|---|
| `crates/ui-egui/src/gorilla.rs` | アプリ名、リンク、アイコン、ArtCraftリンクの表示切り替え、テーマ色の調整（`ACCENT` と `tune_tokens`：Pro系テーマの強調色をゴリラ赤に） |
| `assets/gorilla/` | アプリアイコン（PNG・ICO） |
| `README.md` / `GORILLA.md` | このリポジトリの説明 |

アプリ名は `gorilla::APP_NAME` の1か所で決まります。UIの文字列は翻訳関数を通るときに「PhotoCraft」が自動で「GorillaPhoto」に置き換わるので、本家が新しい文言を足しても自動でGorillaPhotoと表示されます。

## 2. 本家ファイルの「つなぎ目」一覧

本家の更新でここが衝突したら、本家側の新しいコードを受け入れたうえで、下の変更を入れ直してください。

| ファイル | 変更内容 |
|---|---|
| `crates/ui-egui/src/lib.rs` | `pub mod gorilla;` を追加 |
| `crates/ui-egui/src/i18n/mod.rs` | `tr` / `tr_ctx` / `tr_id` / `trn` の戻り値を `gorilla::rebrand` に通す |
| `crates/ui-egui/src/theme.rs` | `apply()` のトークンを `gorilla::tune_tokens` に通す |
| `crates/ui-egui/src/brand.rs` | タイトルバーのマークを `gorilla::ICON_PNG_128` に |
| `crates/ui-egui/src/links.rs` | URLを `gorilla::GITHUB` などに、ArtCraft/Discordのリンクを外す（テストも合わせて修正） |
| `crates/ui-egui/src/menus.rs` | Helpメニューから `help.discord` と `help.artcraftWebsite` を削除 |
| `crates/ui-egui/src/panels.rs` | ウィンドウ名の既定を `APP_NAME` に、タイトルバーのDiscordボタンを `SHOW_ARTCRAFT_COMMUNITY` で非表示 |
| `crates/ui-egui/src/canvas.rs` / `dialogs.rs` | スタート画面とAboutのDiscordボタンを `SHOW_ARTCRAFT_COMMUNITY` で非表示、Aboutのタイトル |
| `crates/ui-egui/src/native_menu.rs` | `APP_NAME` を `gorilla::APP_NAME` に（テストの期待値も） |
| `crates/ui-egui/src/gpu_status.rs` | システム情報のアプリ名 |
| `apps/photocraft/src/main.rs` | ウィンドウタイトル |
| `apps/photocraft/src/app_icon.rs` | ウィンドウ／タスクバーのアイコン |
| `apps/photocraft/build.rs` | Windows exeのアイコンと製品名 |
| `apps/photocraft/tests/mac_menu_appkit.rs` | macOSメニューのテストの期待値 |
| `docs/brand/` | ArtCraftのロゴを削除（ブランドライセンスがフォークに削除を求めているため） |
| `ATTRIBUTION.md` | GorillaPhotoアイコンの行を追加、ArtCraftロゴの行を変更 |

変えていないもの：クレート名・実行ファイル名（`photocraft`）、アプリID、`.pcraft` 形式、設定フォルダ。これらを変えると本家との差分が一気に増えるうえ、保存済みの設定やファイルとの互換が切れるためです。

**新しい改造を足すとき**は、まず `gorilla.rs`（大きくなったら `gorilla/` フォルダに分ける）に書き、本家ファイルには呼び出しだけを入れて、この表に1行追加してください。

## 3. 本家アップデートの取り込み手順

```sh
git remote add upstream https://github.com/storytold/photocraft   # 初回のみ
git fetch upstream --tags

# 1. 何が変わったか見る（例: v0.1.1 → v0.1.2）
git log --oneline v0.1.1..v0.1.2
git diff --stat v0.1.1 v0.1.2
# つなぎ目のファイルが変わっているか確認する
git diff --stat v0.1.1 v0.1.2 -- $(grep -o '`[a-z_/.-]*\.rs`' GORILLA.md | tr -d '`')

# 2. 試験用ブランチで取り込む（mainには直接入れない）
git switch -c update/v0.1.2 main
git merge v0.1.2

# 3. 全テスト
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# 4. 問題なければPull Requestを作って、動作確認後にmainへマージ
```

本家の「Sync fork」ボタンは使わないでください。確認なしで本家の更新がmainに入ってしまいます。

ブランド漏れのチェック（UIに「PhotoCraft」が直書きされていないか）：

```sh
grep -rn '"[^"]*PhotoCraft' --include=*.rs crates/ui-egui/src apps | grep -v 'tl!\|_tests.rs\|/tests/\|gorilla.rs'
```

翻訳関数を通る文字列は自動で置き換わるので、ここに出るのは翻訳を通らない文字列だけです。
