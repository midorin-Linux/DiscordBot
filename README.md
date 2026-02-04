# DiscordBot(仮)
OpenAI互換APIで動作するDiscord用のエージェントです

# NekoAI プロジェクト構造

エージェントAI搭載のDiscord Botとして、各モジュールの役割を明確にした詳細なフォルダ構成とその解説です。

この構成は、**「Discord（入口）」「Agent（頭脳）」「Service（外部連携）」**を明確に分離し、将来的な機能拡張やAIモデルの変更に強い設計になっています。

---

## フォルダ構成

```plain text
NekoAI/
├── config/                     # アプリケーションの設定ファイル
│   └── model.toml              # AIモデルの設定
│
├── prompts/                    # AIの性格や行動指針を決めるプロンプト群
│   ├── original.txt            # オリジナルのシステムプロンプト
│   └── system_prompt.txt       # 基本となるシステムプロンプト
│
├── src/                        # ソースコード
│   ├── main.rs                 # エントリーポイント。各モジュールの初期化と起動
│   │
│   ├── agent/                  # 【エージェント層】AIの思考と記憶を管理
│   │   ├── mod.rs              # Agentモジュールのエクスポート
│   │   ├── agent.rs            # エージェントの司令塔。対話と記憶を統合
│   │   ├── chat.rs             # 対話の組み立て（プロンプト＋履歴＋ツール実行）
│   │   ├── context.rs          # Discordコンテキスト（サーバー、チャンネル、ユーザー情報）の管理
│   │   ├── memory.rs           # 短期/長期記憶（会話履歴の保持）
│   │   ├── prompts.rs          # プロンプトの管理と読み込み
│   │   └── tools/              # ツール実行層
│   │       ├── mod.rs
│   │       ├── tools.rs        # ツール定義と実行の中核
│   │       ├── discord.rs      # Discord操作ツールの統合
│   │       └── discord_tools/  # Discord固有のツール実装
│   │           ├── channel.rs  # チャンネル操作
│   │           ├── emoji.rs    # 絵文字操作
│   │           ├── guild.rs    # サーバー操作
│   │           ├── invite.rs   # 招待リンク操作
│   │           ├── member.rs   # メンバー操作
│   │           ├── message.rs  # メッセージ操作
│   │           ├── role.rs     # ロール操作
│   │           ├── schedule.rs # スケジュール操作
│   │           ├── thread.rs   # スレッド操作
│   │           └── voice.rs    # ボイスチャンネル操作
│   │
│   ├── bot/                    # 【Discord層】Discord固有のUI/UXを管理
│   │   ├── mod.rs              # Botモジュールのエクスポート
│   │   ├── client.rs           # Discordクライアント（Serenity）の構築
│   │   ├── handler.rs          # メッセージ受信などのイベントの振り分け
│   │   └── commands/           # スラッシュコマンド等の具体的な処理
│   │       ├── mod.rs          # コマンドモジュールのエクスポート
│   │       ├── commands.rs     # コマンド定義
│   │       ├── admin/          # 管理者用コマンド
│   │       │   ├── mod.rs
│   │       │   ├── exec.rs     # コード実行コマンド
│   │       │   └── prompt.rs   # プロンプト管理コマンド
│   │       └── general/        # 一般ユーザー用コマンド
│   │           ├── mod.rs
│   │           └── ping.rs     # Pingコマンド
│   │
│   ├── core/                   # 【制御層】アプリの生命維持装置
│   │   ├── mod.rs              # Coreモジュールのエクスポート
│   │   ├── runner.rs           # 非同期ランタイム（Tokio）のループ制御
│   │   └── shutdown.rs         # 終了時のクリーンアップ処理（Ctrl+C対応など）
│   │
│   ├── models/                 # 【データ層】アプリ全体で使う共通の型定義
│   │   ├── mod.rs              # Modelsモジュールのエクスポート
│   │   ├── conversation.rs     # 会話セッションの管理
│   │   ├── message.rs          # 発言内容、ロール（User/AI）、時刻などの構造体
│   │   └── user.rs             # Discordユーザー情報の定義
│   │
│   ├── services/               # 【サービス層】外部APIやデータベースとの通信
│   │   ├── mod.rs              # Servicesモジュールのエクスポート
│   │   └── openai.rs           # OpenAI APIとの生のリクエスト/レスポンス処理
│   │
│   └── utils/                  # 【共通工具層】どこからでも使われる便利機能
│       ├── mod.rs              # Utilsモジュールのエクスポート
│       ├── config.rs           # .envや設定ファイルの読み込み
│       ├── error.rs            # アプリ固有のエラー型定義
│       └── logger.rs           # ログ出力（tracing）のセットアップ
│
├── .env                        # APIキーなどの機密情報（Git管理外）
├── .env.example                # .envのテンプレート
├── .github/workflows/rust.yml  # GitHub Actions CI/CD設定
├── Cargo.toml                  # 依存ライブラリの管理
├── Cargo.lock                  # 依存関係のロックファイル
├── FEATURE.md                  # 機能仕様書
├── README.md                   # プロジェクト概要
└── STRUCTURE.md                # このファイル
```