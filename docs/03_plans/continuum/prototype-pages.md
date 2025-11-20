# プロトタイプページ設計書

## 概要

モックデータを使用したプロトタイプ開発のために、Continuumアプリに必要なページを明確化します。

## ページ一覧

### 1. 認証関連ページ

#### 1.1 ログインページ (`/login`)
- **目的**: GitHub OAuth認証の開始
- **機能**:
  - GitHub OAuth認証ボタンの表示
  - 認証後のリダイレクト先指定
- **モックデータ**: 不要（認証フローは後で実装）

#### 1.2 OAuthコールバックページ (`/auth/callback`)
- **目的**: GitHub OAuth認証後のコールバック処理
- **機能**:
  - 認証コードの受け取り
  - トークン取得とセッション管理
  - ダッシュボードへのリダイレクト
- **モックデータ**: 不要（認証フローは後で実装）

---

### 2. ダッシュボード機能ページ

#### 2.1 メインダッシュボード (`/dashboard`)
- **目的**: 組織全体の活動を可視化
- **機能要件** (PRD 5.1):
  - ✅ 組織全体の活動サマリー
    - コントリビューター数
    - リポジトリ数
    - 外部からのPR数
  - ✅ 活動量ランキング
    - 週間ランキング
    - 月間ランキング
  - ✅ リアルタイムアクティビティタイムライン
    - コミット、PR、レビューなどの時系列表示
  - ✅ リポジトリ一覧
    - Star数
    - 最終更新日
    - コントリビューター構成比
- **モックデータが必要な項目**:
  - 組織統計データ（コントリビューター数、リポジトリ数、PR数）
  - ユーザーランキングデータ（週間/月間）
  - アクティビティタイムラインデータ（コミット、PR、レビュー）
  - リポジトリ一覧データ（名前、Star数、更新日、コントリビューター情報）

---

### 3. 知見共有機能ページ

#### 3.1 ナレッジフィード一覧 (`/knowledge`)
- **目的**: GitHub Discussionsをデータソースとしたナレッジの一覧表示
- **機能要件** (PRD 5.2):
  - ✅ ブログ形式のナレッジフィード表示
  - ✅ カテゴリによるフィルタリング
  - ✅ 投稿者によるフィルタリング
  - ✅ 検索機能
- **モックデータが必要な項目**:
  - Discussion一覧データ（タイトル、投稿者、カテゴリ、投稿日、概要）
  - カテゴリ一覧
  - 投稿者一覧

#### 3.2 ナレッジ詳細ページ (`/knowledge/:id`)
- **目的**: 個別のDiscussionの詳細表示
- **機能要件**:
  - ✅ Discussionの全文表示
  - ✅ コメント表示
  - ✅ 関連ナレッジの表示
- **モックデータが必要な項目**:
  - Discussion詳細データ（本文、コメント、メタデータ）
  - 関連Discussionデータ

---

### 4. ポートフォリオ機能ページ

#### 4.1 個人ページ (`/portfolio/:username`)
- **目的**: ユーザーごとの専用ページ（社外共有可能な公開URL）
- **機能要件** (PRD 5.3):
  - ✅ ユーザー情報表示（アバター、名前、所属期間など）
  - ✅ コントリビューショングラフ（時系列）
  - ✅ リポジトリ別コントリビューション
  - ✅ 組織貢献ハイライト
    - コミット数
    - PR数
    - レビュー数
    - 主要なコントリビューション
- **モックデータが必要な項目**:
  - ユーザープロフィールデータ
  - コントリビューショングラフデータ（日別/週別/月別）
  - リポジトリ別コントリビューションデータ
  - 主要なコントリビューション（コミット、PR、レビュー）

#### 4.2 組織貢献ハイライト (`/portfolio/:username/contributions`)
- **目的**: 会社Organizationへの貢献に特化した活動履歴
- **機能要件**:
  - ✅ 組織への貢献に特化した表示
  - ✅ 時系列での活動履歴
  - ✅ 貢献度の可視化
- **モックデータが必要な項目**:
  - 組織への貢献データ（コミット、PR、レビュー）
  - 時系列データ

---

### 5. その他のページ

#### 5.1 ホームページ (`/`)
- **目的**: ランディングページ（既存）
- **機能**:
  - プラットフォームの紹介
  - 主要機能への導線
- **モックデータ**: 不要（静的コンテンツ）

#### 5.2 404ページ (`/*`)
- **目的**: 存在しないページへのアクセス時の表示
- **機能**: エラーメッセージとナビゲーション
- **モックデータ**: 不要

---

## ページ階層構造

```
/                           # ホームページ（既存）
├── /login                  # ログインページ
├── /auth/callback          # OAuthコールバック
├── /dashboard              # メインダッシュボード
├── /knowledge              # ナレッジフィード一覧
│   └── /knowledge/:id      # ナレッジ詳細
└── /portfolio/:username    # 個人ページ
    └── /portfolio/:username/contributions  # 組織貢献ハイライト
```

## 優先順位（プロトタイプ開発順）

### Phase 1: 基本ページ構造
1. ✅ ホームページ (`/`) - 既存
2. 🔄 メインダッシュボード (`/dashboard`)
3. 🔄 個人ページ (`/portfolio/:username`)

### Phase 2: 機能追加
4. ナレッジフィード一覧 (`/knowledge`)
5. ナレッジ詳細 (`/knowledge/:id`)

### Phase 3: 認証統合
6. ログインページ (`/login`)
7. OAuthコールバック (`/auth/callback`)

## モックデータ構造の要件

### 共通データ構造

#### User（ユーザー）
```rust
struct User {
    username: String,
    display_name: String,
    avatar_url: String,
    github_url: String,
    role: UserRole, // CurrentEmployee, Alumni, ExternalContributor
    joined_at: Option<DateTime>, // 組織への参加日（現役社員の場合）
    left_at: Option<DateTime>,   // 退職日（アルムナイの場合）
}
```

#### Repository（リポジトリ）
```rust
struct Repository {
    name: String,
    full_name: String,
    description: String,
    stars: u32,
    language: Option<String>,
    updated_at: DateTime,
    contributors: Vec<ContributorStats>,
}
```

#### Activity（アクティビティ）
```rust
enum ActivityType {
    Commit,
    PullRequest,
    Review,
    Issue,
    Discussion,
}

struct Activity {
    id: String,
    type: ActivityType,
    user: User,
    repository: Repository,
    title: String,
    created_at: DateTime,
    url: String,
}
```

### ダッシュボード用データ

#### OrganizationStats（組織統計）
```rust
struct OrganizationStats {
    total_contributors: u32,
    total_repositories: u32,
    external_prs_count: u32,
    total_commits: u32,
    period: Period, // Weekly, Monthly
}
```

#### RankingEntry（ランキングエントリ）
```rust
struct RankingEntry {
    user: User,
    commits: u32,
    prs: u32,
    reviews: u32,
    score: u32, // 総合スコア
    rank: u32,
}
```

### ポートフォリオ用データ

#### ContributionGraph（コントリビューショングラフ）
```rust
struct ContributionGraph {
    user: User,
    data: Vec<ContributionDay>, // 日別のコントリビューション数
    period: Period,
}

struct ContributionDay {
    date: NaiveDate, // Note: We use NaiveDate here because contribution graphs only need date-level granularity and do not require time or timezone information.
    commits: u32,
    prs: u32,
    reviews: u32,
}
```

#### RepositoryContribution（リポジトリ別コントリビューション）
```rust
struct RepositoryContribution {
    repository: Repository,
    commits: u32,
    prs: u32,
    reviews: u32,
    lines_added: u32,
    lines_deleted: u32,
    percentage: f64, // リポジトリ全体に対する貢献率
}
```

### ナレッジ用データ

#### Discussion（ディスカッション）
```rust
struct Discussion {
    id: String,
    title: String,
    body: String,
    author: User,
    category: DiscussionCategory,
    created_at: DateTime,
    updated_at: DateTime,
    comments: Vec<Comment>,
    repository: Option<Repository>,
}

enum DiscussionCategory {
    General,
    QAndA,
    Ideas,
    Announcements,
    ShowAndTell,
}
```

## 次のステップ

1. ✅ ページ構造の明確化（本ドキュメント）
2. 🔄 モックデータの生成（各ページ用のサンプルデータ作成）
3. 🔄 ページコンポーネントの実装（Leptos）
4. 🔄 ルーティング設定（`lib.rs`）
5. 🔄 UIコンポーネントの実装（共通コンポーネント）

## 関連ドキュメント

- [PRD.md](../../../PRD.md)
- [Pages ディレクトリ構造](../../../app/src/pages/README.md)
- [開発規則](../../rules/README.md)

