# パネル移動（Photoshop式ドック）作業指示書

PCで Claude Code に作業させるための指示書です。`AGENTS.md` と `GORILLA.md` を先に読ませてください。

## 目的
Photoshopと同じように、パネルを自由に配置できるようにする。操作感はPhotoshopに合わせる。

1. **段階1**: タブを別のパネルグループへドラッグで付け替える（例: Channels を Color のグループへ）。
2. **段階2**: タブ／グループをドラッグして浮かせる（フローティング）。ドラッグで戻せる。
3. **段階3**: ドックを左右の両側に置けるようにし、ドラッグ中に青（赤）の挿入ガイドを出す。

## 現状（`crates/ui-egui/src/dock.rs`, `panels.rs`）
- `Group` enum（Color, Properties, Character, Navigator, History, Layers）が固定で、タブも `Group::tabs()` で決め打ち。
- 保存形式は `DockLayout { order, heights, collapsed }` と `DockTabs`（グループごとのタブ番号）。
- できるのは右カラム内でのグループの上下入れ替えだけ。
- `Group::` は約150か所で使われている（`panels.rs` の `dock_body`、`workspace_ui.rs`、テスト等）。

## 進め方の方針
- 段階ごとにPRを分け、各段階で `cargo test -p photocraft-ui-egui` と clippy を通す。
- 新しいパネルモデルは新規モジュール `crates/ui-egui/src/gorilla_dock/` に置き、本家ファイルは呼び出しだけにする（`GORILLA.md` の方針）。つなぎ目を増やしたら `GORILLA.md` の表に1行足す。
- 保存データは `#[serde(default)]` で旧形式を読めるようにする（古い `panelLayout` / ワークスペースが壊れないこと）。
- パニックしない（`AGENTS.md` の Never crash）。範囲外のタブ番号や空のグループは必ず許容する。
- UIは `cargo run -p photocraft-ui-egui --example snapshot` で画像を出して目で確認する。
- 終わったら `log/devlog.md` に追記する。

## 最初に渡す指示文
```
AGENTS.md と GORILLA.md を読んで。次に GORILLA_PANELS.md を読んで、段階1（タブを別グループへドラッグで付け替え）だけを実装して。
まず dock.rs / panels.rs / workspace_ui.rs / dock_tests.rs を読んで設計案を出し、私が了承してから書いて。
ブランチは gorilla/panels-tab-move。完了したら cargo test と clippy を通してPRを作って。
```
