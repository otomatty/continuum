# Phase 1-1: GitHub GraphQL API クライアント実装

## 概要

GitHub GraphQL API v4を使用して、組織のリポジトリ、コントリビューション、ユーザー情報を取得するクライアントを実装します。

## 目標

- GitHub GraphQL APIへの接続と認証
- 必要なデータを取得するクエリの実装
- エラーハンドリングとレート制限対応

## 実装内容

### 1. ディレクトリ構造

```
app/src/github/
├── mod.rs              # モジュール定義
├── client.rs           # GraphQLクライアント実装
├── queries.rs          # GraphQLクエリ定義
└── types.rs            # APIレスポンス型定義
```

### 2. 実装タスク

#### 2.1 GraphQLクライアント (`client.rs`)

**機能:**
- HTTPリクエスト送信（reqwest使用）
- 認証トークン管理
- レート制限監視
- エラーハンドリング

**実装項目:**
- `GitHubClient` 構造体
- `execute_query()` メソッド
- レート制限チェック機能
- リトライロジック

#### 2.2 GraphQLクエリ定義 (`queries.rs`)

**必要なクエリ:**
1. **組織情報取得**
   - 組織名、説明、アバター
   - メンバー数、リポジトリ数

2. **リポジトリ一覧取得**
   - リポジトリ名、説明、Star数
   - 最終更新日、言語、トピック
   - コントリビューター数

3. **ユーザー情報取得**
   - ユーザー名、アバター、プロフィール
   - 組織への参加日

4. **コントリビューション取得**
   - コミット数、PR数、レビュー数
   - 時系列データ
   - リポジトリ別集計

**実装項目:**
- 各クエリのGraphQL文字列定義
- クエリパラメータ型定義
- ページネーション対応

#### 2.3 型定義 (`types.rs`)

**必要な型:**
- `Organization`
- `Repository`
- `User`
- `Contribution`
- `Commit`
- `PullRequest`
- `Review`

**実装項目:**
- Serdeによるデシリアライゼーション
- GraphQLレスポンス型とのマッピング

### 3. 依存関係

```toml
# app/Cargo.toml に追加
[dependencies]
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
```

### 4. エラーハンドリング

**エラー種別:**
- ネットワークエラー
- GraphQLエラー（API制限、認証エラー等）
- パースエラー
- レート制限エラー

**実装:**
- `thiserror`を使用したカスタムエラー型
- エラーメッセージの適切な処理

### 5. テスト

**テスト項目:**
- モックサーバーを使用したクエリ実行テスト
- エラーハンドリングテスト
- レート制限テスト

## 実装手順

1. `app/src/github/` ディレクトリ作成
2. `mod.rs` でモジュール定義
3. `types.rs` で型定義実装
4. `queries.rs` でクエリ定義
5. `client.rs` でクライアント実装
6. エラーハンドリング実装
7. テスト作成

## 参考資料

- [GitHub GraphQL API Documentation](https://docs.github.com/en/graphql)
- [GraphQL Explorer](https://docs.github.com/en/graphql/overview/explorer)

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- データモデル: `../phase2-data-models/README.md`

