# NekoAI
これはRustで書かれている**Discord用AIエージェント**です。**OpenAI互換API**を使用して動作させることができ、メモリ管理（長期・中期・短期記憶）とツール実行機能を備えた高度なチャットボットです。

開発では[OpenRouter](https://openrouter.ai/)を使用することを推奨しています。

## 特徴
- **階層型メモリシステム:**
    - **短期記憶:** 最新の会話コンテキストを保持（インメモリストア）。
    - **中期記憶:** 過去の会話の要約をベクトル検索（Qdrant）で取得。7日間の有効期限付きで自動クリーンアップ。
    - **長期記憶:** ユーザーに関する永続的な事実をベクトル検索で取得。
- **マルチモーダル対話:** スラッシュコマンド（`/chat`）とメンション応答の両方に対応。
- **自動メッセージ分割:** Discordの2000文字制限を超える長い応答を適切に分割して送信。
- **拡張可能なツール機能:** Rig SDKを活用したエージェントツール（例: `send_message`）を搭載。
- **クリーンアーキテクチャ:** レイヤードアーキテクチャを採用し、DI（依存性の注入）により各コンポーネントが抽象化されています。

## 技術スタック
- **serenity**: [serenity-rs/serenity](https://github.com/serenity-rs/serenity) - A Rust library for the Discord API.
- **poise**: [serenity-rs/poise](https://github.com/serenity-rs/poise) - Discord bot command framework for serenity, with advanced features like edit tracking and flexible argument parsing
- **rig**: [0xPlaygrounds/rig](https://github.com/0xPlaygrounds/rig) - ⚙️🦀 Build modular and scalable LLM Applications in Rust

## 環境構築・使用方法

### 1. プロジェクトの初期設定
```bash
git clone https://github.com/midorin-Linux/NekoAI.git
cd NekoAI
cp .env.example .env
```
`.env` と `config/settings.toml` を開き、APIキーや接続情報を設定してください。

### 2. Qdrant の起動
```bash
docker pull qdrant/qdrant
docker run -p 6333:6333 -p 6334:6334 -e QDRANT__SERVICE__GRPC_PORT="6334" qdrant/qdrant
```

### 3. アプリケーションの起動
```bash
# 開発用
cargo run

# 本番用
cargo run --release
```

## ディレクトリ構造
```text
NekoAI/
├── config/
│   └── settings.toml           # 環境非依存の設定
├── src/
│   ├── application/            # ユースケース・ビジネスロジック
│   │   ├── chat/               # チャット処理（コンテキスト構築・AI呼び出し）
│   │   ├── command/            # Poiseコマンド定義と登録
│   │   └── traits/             # 抽象化トレイト（DI用）
│   ├── infrastructure/         # 外部サービス・DBの実装
│   │   ├── ai/                 # Rigクライアント・エージェントツール
│   │   ├── discord/            # Serenityクライアント
│   │   └── store/              # Qdrant/インメモリストア実装
│   ├── presentation/           # 外部インターフェース
│   │   └── events/             # Discordイベントハンドラー
│   ├── models/                 # ドメインモデル・エラー定義
│   └── shared/                 # 設定・ロガー・ユーティリティ
├── INSTRUCTION.md              # AI用システム命令
└── tests/                      # テストコード
```

## 開発
### テストの実行
```bash
cargo test
```

### リンターとフォーマット
フォーマットにはUnstableな項目が使われているため、Nightlyバージョンを使用してください。
```bash
cargo clippy -- -D warnings
cargo +nightly fmt --all --check
```
