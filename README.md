# RTM (Rust Timer)

Rust と egui (eframe) で構築された、シンプルで高速なデスクトップタイマーアプリです。
Windows 向けに最適化されており、軽量なバイナリと直感的な操作感を提供します。

## 🚀 Specification

- **ターミナル連携**: インストール後、コマンドラインから `$ rtm` で即座に起動可能。
- **キーボード操作対応**: 主要操作にショートカットキーを割り当て。

#### download for windows

最新のインストーラー（MSI）は以下からダウンロードできます。

[**📥 rtm-rust-timer v0.1.1 をダウンロード**](https://github.com/nemuinari/rtm-rust-timer/releases/download/v0.1.1/rtm-rust-timer-0.1.1-x86_64.msi)

```bash
# インストール後の実行コマンド
$ rtm
```

## ⌨️Key Bindings

| キー                     | アクション                           |
| :----------------------- | :----------------------------------- |
| **[S]**                  | タイマーの開始 / 停止 (START / STOP) |
| **[R]**                  | タイマーのリセット (RESET)           |
| **[Space]**              | ウィンドウの最小化                   |
| **[Esc]**                | アプリケーションの終了               |
| **[Win + (Alt) + 矢印]** | 移動(Windows標準機能)                |

## 🛠️Developer Setup

#### Directory structure

```txt
rtm-rust-timer/
├── Cargo.toml          # プロジェクト設定・依存関係
├── build.rs            # Windowsリソース（アイコン）埋め込み用
├── assets/             # 静的リソース
│   ├── icon.png        # アプリ用アイコン (512x512)
│   ├── icon.ico        # Windows用アイコン (Tauri CLIで生成)
│   └── font.ttf        # デジタル数字フォント
├── src/
│   ├── main.rs         # エントリーポイント・ウィンドウ設定
│   ├── app.rs          # UI描画・入力ハンドリング
│   ├── timer.rs        # タイマー計算ロジック
│   └── components/     # 再利用可能なUIパーツ
└── wix/                # MSIインストーラー設定ファイル
```

### build and Relese

```bash
# インストール
$ cargo install tauri-cli

# png から ico / icns を自動生成
$ cargo tauri icon --output assets assets/icon.png

# run
$ cargo run

# build
$ cargo build --release

# cargo-wix
$ cargo install cargo-wix

# 初回のみ（Cargo.toml に authors と license が必要）
$ cargo wix init

# MSI ビルド
$ cargo wix

```

## Recommended environment

- Rust 1.70.0 以上
- Windows 10 / 11 (MSIインストーラー対応)
