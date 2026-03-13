# RTM (Rust Timer)

```txt
Rustで開発したPC(windows向け)用デスクトップタイマー
ターミナルから"$ rtm" で起動可能、キー操作対応
※ .exe から直接開くことも可能
```

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

````txt
rtm-rust-timer/
├── Cargo.toml            # プロジェクト設定・依存関係
├── build.rs              # Windowsリソース（アイコン等）埋め込み用
├── assets/               # 静的リソース
│   ├── icon.png          # アプリ用アイコン (512x512)
│   ├── icon.ico          # Windows用アイコン
│   └── PixelMplus12.ttf  # デジタル数字用フォント
├── src/
│   ├── main.rs           # エントリーポイント（アプリ起動のみを担当）
│   └── components/       # アプリケーションの各機能モジュール
│       ├── mod.rs        # 各モジュールの公開定義
│       ├── app.rs        # メインループ、UI配置、イベントハンドリング
│       ├── logic.rs      # タイマー計算、時間フォーマットのロジック
│       ├── ui.rs         # 再利用可能なUIパーツ（表示、ボタン、共通スタイル）
│       ├── frame.rs      # ウィンドウ設定（サイズ、透過、常時最前面等）
│       ├── font.rs       # カスタムフォントの読み込み設定
│       └── icon.rs       # アプリ内でのアイコン読み込み
└── wix/                  # MSIインストーラー生成用設定```

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

````

## Recommended environment

- Rust 1.70.0 以上
- Windows 10 / 11 (MSIインストーラー対応)
