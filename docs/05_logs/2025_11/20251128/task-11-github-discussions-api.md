# Task 11: GitHub Discussions API 連携 - 実装ログ

## 日時

2025 年 11 月 28 日

## 概要

GitHub GraphQL API を使用して Discussions データを取得する機能を実装しました。

## 実装内容

### 1. GraphQL クエリの追加 (`app/src/github/queries.rs`)

以下の 3 つのクエリを追加：

- `DISCUSSION_CATEGORIES_QUERY`: Discussion カテゴリ一覧を取得
- `DISCUSSIONS_QUERY`: Discussion 一覧を取得（ページネーション対応、カテゴリフィルタ対応）
- `DISCUSSION_DETAIL_QUERY`: 単一の Discussion 詳細を取得（コメント含む）

### 2. 型定義の追加 (`app/src/github/types.rs`)

GitHub API レスポンスに対応する型を追加：

- `DiscussionPageInfo`: ページネーション情報
- `DiscussionCategoriesData`, `DiscussionCategoriesRepository`, `DiscussionCategoryConnection`: カテゴリ一覧レスポンス
- `GitHubDiscussionCategory`: カテゴリ情報
- `DiscussionsData`, `DiscussionsRepository`, `DiscussionConnection`: Discussion 一覧レスポンス
- `GitHubDiscussion`: 個別の Discussion
- `GitHubAuthor`: 投稿者情報
- `CommentCount`, `ReactionCount`: カウント情報
- `LabelConnection`, `GitHubLabel`: ラベル情報
- `DiscussionDetailData`, `GitHubDiscussionDetail`: 詳細レスポンス
- `DiscussionCommentConnection`, `GitHubDiscussionComment`: コメント情報

### 3. API クライアントメソッドの追加 (`app/src/github/client.rs`)

`GitHubClient` に以下のメソッドを追加：

- `get_discussion_categories`: カテゴリ一覧取得
- `get_discussions`: Discussion 一覧取得
- `get_discussion_detail`: Discussion 詳細取得

### 4. Server Function の実装 (`app/src/concepts/discussion/server.rs`)

新規作成。以下の Server Function を実装：

- `get_discussion_categories`: `/api/discussions/categories`
- `get_discussions`: `/api/discussions`
- `get_discussion_detail`: `/api/discussions/detail`

GitHub API レスポンスを Discussion Concept の型に変換する処理を含む。

### 5. mod.rs の更新 (`app/src/concepts/discussion/mod.rs`)

`server` モジュールを追加し、公開。

### 6. 仕様書の更新 (`app/src/concepts/discussion/discussion.spec.md`)

- Server Functions のドキュメントを追加
- 環境変数要件を記載
- 関連ファイルに `server.rs` を追加

## 確認結果

- `cargo check`: ✅ パス
- `cargo check --features ssr`: ✅ パス
- `cargo test --features ssr`: ✅ 138 テスト全パス

## 注意事項

- 使用には Discussions が有効なリポジトリが必要
- 環境変数 `GITHUB_ORG` と `GITHUB_TOKEN` が必須
- GitHub API のレート制限に注意

## 次のステップ

- 知見共有ページ（`/knowledge`）で Server Function を使用
- 詳細ページ（`/knowledge/:id`）で詳細取得を実装
- カテゴリフィルタリング UI の実装
