# GorillaPhoto

Photoshopの操作感をそのままに、見た目を自分用に作り替えた画像編集アプリ。

GorillaPhotoは、オープンソースの画像編集アプリ **[PhotoCraft](https://github.com/storytold/photocraft)**（Rust製、MIT OR Apache-2.0）をもとにした派生版です。編集エンジン・ツール・PSD対応・ショートカット・メニュー構成は本家PhotoCraftのものをそのまま使い、アプリ名・アイコン・見た目（UI）だけを変えています。

> GorillaPhotoは本家PhotoCraftおよびArtCraft Teamが作ったものではなく、本家の推奨・サポートを受けたものでもありません。Adobe、Photoshopは米国およびその他の国におけるAdobe Inc.の商標です。GorillaPhotoはAdobe Inc.とは無関係です。

## 本家との違い

| 項目 | GorillaPhoto |
|---|---|
| アプリ名 | GorillaPhoto |
| アイコン | 赤背景のゴリラ（`assets/gorilla/`） |
| ArtCraftのロゴ・コミュニティリンク | 外してある（ArtCraftブランドのライセンスに従う） |
| 操作感（メニュー・ショートカット・ツール） | 本家と同じ（Photoshop準拠） |

これから追加していく予定：Photoshop風のオリジナルツールアイコン、Photoshop式のパネル移動（タブの付け替え、フローティング、左右ドッキング）。

## ビルド

```sh
cargo run --release -p photocraft      # デスクトップアプリ
cargo test --workspace                 # テスト
```

Windowsでは事前に [rustup](https://rustup.rs/) と Visual Studio Build Tools（C++）が必要です。内部のクレート名・実行ファイル名は本家と同じ `photocraft` のままにしてあります（本家の更新を取り込みやすくするため）。

## 本家の更新の取り込み方

改造部分の分け方と、本家アップデートの手順は [`GORILLA.md`](GORILLA.md) にまとめてあります。

## ライセンス

本家PhotoCraftと同じく、[MIT](LICENSE-MIT) または [Apache-2.0](LICENSE-APACHE) のどちらかを選べるデュアルライセンスです。必要な表記は [NOTICE](NOTICE)、同梱アセットの出典は [ATTRIBUTION.md](ATTRIBUTION.md) にあります。

Copyright (c) 2026 ArtCraft Team and the PhotoCraft contributors; GorillaPhoto changes by the GorillaPhoto contributors.
