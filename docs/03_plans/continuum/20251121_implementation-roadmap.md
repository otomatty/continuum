# 実装ロードマップ (改訂版)

**作成日:** 2025-11-21  
**関連ドキュメント:**
- `docs/02_research/2025_11/20251121_screen-design-proposal.md`
- `docs/02_research/2025_11/20251121_screen-flow-diagram.md`
- `docs/02_research/2025_11/20251121_implementation-status.md`
- `PRD.md`

## 1. コンポーネント構造の方針

### 1.1 基本方針

**ページ専用コンポーネントは、各ページディレクトリ直下に`components/`ディレクトリを作成し、コンポーネントごとにファイルを分けて実装する。**

### 1.2 ディレクトリ構造

```
app/src/pages/
├── mod.rs
├── home/
│   ├── mod.rs                          # ページコンポーネント本体
│   └── components/                     # ページ専用コンポーネント
│       ├── mod.rs                      # 再エクスポート
│       ├── hero_section.rs
│       ├── value_proposition_card.rs
│       └── statistics_preview.rs
├── dashboard/
│   ├── mod.rs
│   └── components/
│       ├── mod.rs
│       ├── stats_card.rs               # 統計カード
│       ├── ranking_table.rs            # ランキングテーブル
│       ├── activity_timeline.rs        # アクティビティタイムライン
│       └── repository_list.rs          # リポジトリ一覧
├── portfolio/
│   ├── mod.rs
│   └── components/
│       ├── mod.rs
│       ├── user_profile.rs             # ユーザープロフィール
│       ├── contribution_graph.rs       # コントリビューショングラフ
│       ├── contribution_highlights.rs  # コントリビューションハイライト
│       └── repository_contribution_list.rs  # リポジトリ別コントリビューション
├── knowledge/
│   ├── mod.rs
│   └── components/
│       ├── mod.rs
│       ├── knowledge_card.rs           # ナレッジカード
│       ├── search_bar.rs               # 検索バー
│       ├── category_filter.rs          # カテゴリフィルター
│       └── author_filter.rs            # 投稿者フィルター
├── knowledge_detail/
│   ├── mod.rs
│   └── components/
│       ├── mod.rs
│       ├── markdown_renderer.rs        # Markdownレンダラー
│       ├── comment_section.rs          # コメントセクション
│       ├── reaction_buttons.rs         # リアクションボタン
│       └── related_articles.rs         # 関連記事
├── contributors/
│   ├── mod.rs
│   └── components/
│       ├── mod.rs
│       ├── contributor_card.rs         # コントリビューターカード
│       ├── contributor_grid.rs         # グリッドレイアウト
│       └── status_filter.rs            # ステータスフィルター
├── repositories/
│   ├── mod.rs
│   └── components/
│       ├── mod.rs
│       ├── repository_table.rs         # リポジトリテーブル
│       ├── language_filter.rs          # 言語フィルター
│       └── sort_control.rs             # ソートコントロール
├── repository_detail/
│   ├── mod.rs
│   └── components/
│       ├── mod.rs
│       ├── repository_header.rs        # リポジトリヘッダー
│       ├── contributor_pie_chart.rs    # コントリビューター構成円グラフ
│       ├── top_contributors_list.rs    # トップコントリビューター一覧
│       └── language_bar_chart.rs       # 言語構成バーチャート
├── settings/
│   ├── mod.rs
│   └── components/
│       ├── mod.rs
│       ├── profile_settings_form.rs    # プロフィール設定フォーム
│       ├── privacy_settings.rs         # プライバシー設定
│       ├── notification_settings.rs    # 通知設定
│       └── data_export_button.rs       # データエクスポートボタン
├── not_found/
│   └── mod.rs                          # 404ページ（専用コンポーネント不要）
└── auth_error/
    └── mod.rs                          # 認証エラーページ（専用コンポーネント不要）
```

### 1.3 コンポーネントファイルの構造

各コンポーネントファイルは以下の構造を持つ：

```rust
/**
 * [Component Name]
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/[page]/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/[component].rs
 *   ├─ app/src/concepts/[concept]/mod.rs
 *   └─ ...
 *
 * Related Documentation:
 *   └─ docs/03_plans/continuum/20251121_implementation-roadmap.md
 */

use leptos::prelude::*;
use crate::components::...;

#[component]
pub fn ComponentName(/* props */) -> impl IntoView {
    view! {
        // implementation
    }
}
```

### 1.4 `components/mod.rs` の構造

```rust
// app/src/pages/[page]/components/mod.rs

mod stats_card;
mod ranking_table;
mod activity_timeline;
mod repository_list;

pub use stats_card::StatsCard;
pub use ranking_table::RankingTable;
pub use activity_timeline::ActivityTimeline;
pub use repository_list::RepositoryList;
```

---

## 2. Phase 1: MVP (Minimum Viable Product)

### 目標
PRD準拠の基本機能を提供し、認証済みユーザーがダッシュボードとポートフォリオを閲覧できるようにする。

### 2.1 既存コードのリファクタリング (Task 1)

**優先度:** 🔴 Critical  
**見積もり:** 2-3時間

#### タスク内容

1. **Dashboard コンポーネントの分割**
   - [ ] `app/src/pages/dashboard/components/` ディレクトリ作成
   - [ ] `stats_card.rs` - StatsCard コンポーネント分離
   - [ ] `ranking_table.rs` - RankingTable コンポーネント分離
   - [ ] `activity_timeline.rs` - ActivityTimeline コンポーネント分離
   - [ ] `repository_list.rs` - RepositoryList コンポーネント分離
   - [ ] `components/mod.rs` - 再エクスポート
   - [ ] `dashboard/mod.rs` - import文の更新
   - [ ] 既存の `components.rs` を削除

2. **Portfolio コンポーネントの分割**
   - [ ] `app/src/pages/portfolio/components/` ディレクトリ作成
   - [ ] `user_profile.rs` - UserProfile コンポーネント分離
   - [ ] `contribution_graph.rs` - ContributionGraph コンポーネント分離
   - [ ] `contribution_highlights.rs` - ContributionHighlights コンポーネント分離
   - [ ] `repository_contribution_list.rs` - RepositoryContributionList コンポーネント分離
   - [ ] `components/mod.rs` - 再エクスポート
   - [ ] `portfolio/mod.rs` - import文の更新
   - [ ] 既存の `components.rs` を削除

3. **テスト実行**
   - [ ] `bun run dev` でビルドエラーがないことを確認
   - [ ] `/dashboard` にアクセスして正常に表示されることを確認
   - [ ] `/portfolio` にアクセスして正常に表示されることを確認

---

### 2.2 GitHub OAuth 認証実装 (Task 2)

**優先度:** 🔴 Critical  
**見積もり:** 1-2日

#### タスク内容

1. **Auth Concept 実装**
   - [ ] `app/src/concepts/auth/` ディレクトリ作成
   - [ ] `state.rs` - 認証状態の型定義
     - `AuthState` - 認証状態
     - `User` - ログインユーザー情報
     - `OAuthToken` - トークン情報
   - [ ] `actions.rs` - 認証ロジック
     - `login()` - ログイン処理
     - `logout()` - ログアウト処理
     - `refresh_token()` - トークン更新
   - [ ] `auth.spec.md` - 仕様書作成
   - [ ] `tests.rs` - テストケース作成
   - [ ] `mod.rs` - 公開API

2. **GitHub OAuth フロー実装**
   - [ ] `app/src/github/oauth.rs` 作成
   - [ ] OAuth認証URLの生成
   - [ ] コールバック処理の実装
   - [ ] トークン取得・保存

3. **認証ページ実装**
   - [ ] `app/src/pages/auth_error/mod.rs` - 認証エラーページ
   - [ ] ルーティングに追加 (`/auth/error`)

4. **グローバルナビゲーション更新**
   - [ ] Navbar に認証状態表示
   - [ ] ログイン/ログアウトボタン追加
   - [ ] ユーザードロップダウン追加

5. **認証ガード実装**
   - [ ] 未認証時のリダイレクト処理
   - [ ] 認証状態のグローバルコンテキスト

**関連ドキュメント:**
- `docs/03_plans/continuum/phase1-github-oauth/README.md`

---

### 2.3 動的ルーティング対応 (Task 3)

**優先度:** 🔴 Critical  
**見積もり:** 4-6時間

#### タスク内容

1. **Leptos Router 動的パラメータ設定**
   - [ ] `app/src/lib.rs` のルート定義更新
   - [ ] `/portfolio/:username` ルート追加
   - [ ] パラメータ抽出ロジック実装

2. **Portfolio ページ更新**
   - [ ] `portfolio/mod.rs` のパラメータ取得ロジック修正
   - [ ] 固定ユーザー名 `alice-dev` を削除
   - [ ] 動的ユーザー名に対応

3. **404 Not Found ページ実装**
   - [ ] `app/src/pages/not_found/mod.rs` 作成
   - [ ] fallback ルートに設定

4. **テスト**
   - [ ] `/portfolio/alice-dev` にアクセスして表示確認
   - [ ] `/portfolio/bob-dev` にアクセスして表示確認
   - [ ] 存在しないユーザーで404表示確認

**関連ドキュメント:**
- Leptos Router ドキュメント

---

### 2.4 ランディングページ刷新 (Task 4)

**優先度:** 🟡 Important  
**見積もり:** 1日

#### タスク内容

1. **ページ専用コンポーネント実装**
   - [ ] `app/src/pages/home/components/` ディレクトリ作成
   - [ ] `hero_section.rs` - ヒーローセクション
     - キャッチコピー
     - サブタイトル
     - CTAボタン (GitHub OAuth でログイン)
   - [ ] `value_proposition_card.rs` - 価値提案カード
     - 3つの価値提案を表示
   - [ ] `statistics_preview.rs` - 統計情報プレビュー
     - 組織統計（認証不要）
   - [ ] `components/mod.rs` - 再エクスポート

2. **ランディングページ本体実装**
   - [ ] `home/mod.rs` を PRD に沿って刷新
   - [ ] ヒーローセクション追加
   - [ ] 価値提案セクション追加
   - [ ] 統計情報プレビュー追加
   - [ ] フッター追加

3. **スタイリング**
   - [ ] レスポンシブデザイン対応
   - [ ] ダークモード対応
   - [ ] アニメーション追加（オプション）

**デザイン要件（PRD より）:**
- ヒーローセクション: キャッチコピー + CTA
- 価値提案: 3カラムレイアウト
- 統計情報: 3つの主要指標
- フッター: 会社情報 + リンク集

**関連ドキュメント:**
- `PRD.md` - セクション 5.1, 8
- `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.A

---

### 2.5 Phase 1 完了条件

- [x] 既存コードのリファクタリング完了
- [x] GitHub OAuth 認証が動作する
- [x] 動的ルーティングが動作する (`/portfolio/:username`, `/auth/error`)
- [ ] PRD準拠のランディングページが表示される
- [x] 認証済みユーザーがダッシュボードとポートフォリオを閲覧できる
- [x] 404ページが表示される

---

## 3. Phase 2: コミュニティ機能

### 目標
コントリビューター・リポジトリ一覧を提供し、全てのリポジトリとコントリビューターを閲覧・検索できるようにする。

### 3.1 GitHub API 実装 (Task 5)

**優先度:** 🔴 Critical  
**見積もり:** 2-3日

#### タスク内容

1. **GitHub GraphQL クエリ実装**
   - [ ] `app/src/github/queries.rs` の拡充
   - [ ] リポジトリ一覧取得クエリ
   - [ ] ユーザー情報取得クエリ
   - [ ] コントリビューション取得クエリ
   - [ ] ページネーション対応

2. **GitHub API クライアント実装**
   - [ ] `app/src/github/client.rs` の拡充
   - [ ] 認証ヘッダー追加
   - [ ] レート制限対応
   - [ ] エラーハンドリング

3. **キャッシュ機構実装**
   - [ ] Cloudflare KV 連携
   - [ ] キャッシュ戦略の実装
   - [ ] TTL設定

**関連ドキュメント:**
- `docs/03_plans/continuum/phase1-github-api-client/README.md`
- `docs/03_plans/continuum/phase2-caching-strategy/README.md`

---

### 3.2 Search & Filter Concepts 実装 (Task 6)

**優先度:** 🟡 Important  
**見積もり:** 1日

#### タスク内容

1. **Search Concept 実装**
   - [ ] `app/src/concepts/search/` ディレクトリ作成
   - [ ] `state.rs` - 検索状態の型定義
   - [ ] `actions.rs` - 検索ロジック
   - [ ] `search.spec.md` - 仕様書
   - [ ] `tests.rs` - テスト

2. **Filter Concept 実装**
   - [ ] `app/src/concepts/filter/` ディレクトリ作成
   - [ ] `state.rs` - フィルター状態の型定義
   - [ ] `actions.rs` - フィルタリングロジック
   - [ ] `filter.spec.md` - 仕様書
   - [ ] `tests.rs` - テスト

---

### 3.3 コントリビューター一覧ページ (Task 7)

**優先度:** 🟡 Important  
**見積もり:** 1-2日

#### タスク内容

1. **ページ専用コンポーネント実装**
   - [ ] `app/src/pages/contributors/` ディレクトリ作成
   - [ ] `components/contributor_card.rs` - コントリビューターカード
   - [ ] `components/contributor_grid.rs` - グリッドレイアウト
   - [ ] `components/status_filter.rs` - ステータスフィルター
   - [ ] `components/mod.rs` - 再エクスポート

2. **ページ本体実装**
   - [ ] `contributors/mod.rs` - ページコンポーネント
   - [ ] 検索バー追加
   - [ ] フィルター機能追加
   - [ ] ページネーション追加

3. **ルーティング追加**
   - [ ] `app/src/lib.rs` に `/contributors` ルート追加

**関連ドキュメント:**
- `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.F

---

### 3.4 リポジトリ一覧ページ (Task 8)

**優先度:** 🟡 Important  
**見積もり:** 1-2日

#### タスク内容

1. **ページ専用コンポーネント実装**
   - [ ] `app/src/pages/repositories/` ディレクトリ作成
   - [ ] `components/repository_table.rs` - リポジトリテーブル
   - [ ] `components/language_filter.rs` - 言語フィルター
   - [ ] `components/sort_control.rs` - ソートコントロール
   - [ ] `components/mod.rs` - 再エクスポート

2. **ページ本体実装**
   - [ ] `repositories/mod.rs` - ページコンポーネント
   - [ ] 検索バー追加
   - [ ] フィルター機能追加
   - [ ] ソート機能追加
   - [ ] ページネーション追加

3. **ルーティング追加**
   - [ ] `app/src/lib.rs` に `/repositories` ルート追加

**関連ドキュメント:**
- `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.G

---

### 3.5 リポジトリ詳細ページ (Task 9)

**優先度:** 🟡 Important  
**見積もり:** 2-3日

#### タスク内容

1. **ページ専用コンポーネント実装**
   - [ ] `app/src/pages/repository_detail/` ディレクトリ作成
   - [ ] `components/repository_header.rs` - ヘッダー
   - [ ] `components/contributor_pie_chart.rs` - 円グラフ
   - [ ] `components/top_contributors_list.rs` - トップコントリビューター
   - [ ] `components/language_bar_chart.rs` - 言語構成バーチャート
   - [ ] `components/mod.rs` - 再エクスポート

2. **ページ本体実装**
   - [ ] `repository_detail/mod.rs` - ページコンポーネント
   - [ ] パンくずリスト追加
   - [ ] 統計情報表示
   - [ ] アクティビティタイムライン表示

3. **ルーティング追加**
   - [ ] `app/src/lib.rs` に `/repository/:owner/:repo` ルート追加

**関連ドキュメント:**
- `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.H

---

### 3.6 Phase 2 完了条件

- [ ] GitHub API 連携が動作する
- [ ] キャッシュ機構が動作する
- [ ] コントリビューター一覧が表示・検索できる
- [ ] リポジトリ一覧が表示・検索できる
- [ ] リポジトリ詳細が表示される

---

## 4. Phase 3: 知見共有機能

### 目標
GitHub Discussions ベースのナレッジベースを提供し、Discussions をプラットフォーム内で閲覧・検索できるようにする。

### 4.1 Discussion Concepts 実装 (Task 10)

**優先度:** 🟢 Nice to Have  
**見積もり:** 1-2日

#### タスク内容

1. **Discussion Concept 実装**
   - [ ] `app/src/concepts/discussion/` ディレクトリ作成
   - [ ] `state.rs` - Discussion状態の型定義
   - [ ] `actions.rs` - Discussionロジック
   - [ ] `discussion.spec.md` - 仕様書
   - [ ] `tests.rs` - テスト

2. **Comment Concept 実装**
   - [ ] `app/src/concepts/comment/` ディレクトリ作成
   - [ ] `state.rs` - Comment状態の型定義
   - [ ] `actions.rs` - Commentロジック
   - [ ] `comment.spec.md` - 仕様書
   - [ ] `tests.rs` - テスト

3. **Reaction Concept 実装**
   - [ ] `app/src/concepts/reaction/` ディレクトリ作成
   - [ ] `state.rs` - Reaction状態の型定義
   - [ ] `actions.rs` - Reactionロジック
   - [ ] `reaction.spec.md` - 仕様書
   - [ ] `tests.rs` - テスト

---

### 4.2 GitHub Discussions API 連携 (Task 11)

**優先度:** 🟢 Nice to Have  
**見積もり:** 1-2日

#### タスク内容

1. **GraphQL クエリ追加**
   - [ ] Discussions 取得クエリ
   - [ ] Comments 取得クエリ
   - [ ] Reactions 取得クエリ

2. **API クライアント拡張**
   - [ ] Discussions API 呼び出し実装
   - [ ] キャッシュ対応

---

### 4.3 知見共有一覧ページ (Task 12)

**優先度:** 🟢 Nice to Have  
**見積もり:** 1-2日

#### タスク内容

1. **ページ専用コンポーネント実装**
   - [ ] `app/src/pages/knowledge/` ディレクトリ作成
   - [ ] `components/knowledge_card.rs` - ナレッジカード
   - [ ] `components/search_bar.rs` - 検索バー
   - [ ] `components/category_filter.rs` - カテゴリフィルター
   - [ ] `components/author_filter.rs` - 投稿者フィルター
   - [ ] `components/mod.rs` - 再エクスポート

2. **ページ本体実装**
   - [ ] `knowledge/mod.rs` - ページコンポーネント
   - [ ] 検索・フィルター機能
   - [ ] ページネーション

3. **ルーティング追加**
   - [ ] `app/src/lib.rs` に `/knowledge` ルート追加

**関連ドキュメント:**
- `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.D

---

### 4.4 ナレッジ詳細ページ (Task 13)

**優先度:** 🟢 Nice to Have  
**見積もり:** 2-3日

#### タスク内容

1. **ページ専用コンポーネント実装**
   - [ ] `app/src/pages/knowledge_detail/` ディレクトリ作成
   - [ ] `components/markdown_renderer.rs` - Markdownレンダラー
   - [ ] `components/comment_section.rs` - コメントセクション
   - [ ] `components/reaction_buttons.rs` - リアクションボタン
   - [ ] `components/related_articles.rs` - 関連記事
   - [ ] `components/mod.rs` - 再エクスポート

2. **ページ本体実装**
   - [ ] `knowledge_detail/mod.rs` - ページコンポーネント
   - [ ] パンくずリスト
   - [ ] 記事ヘッダー
   - [ ] Markdown レンダリング
   - [ ] コメント表示

3. **ルーティング追加**
   - [ ] `app/src/lib.rs` に `/knowledge/:id` ルート追加

**関連ドキュメント:**
- `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.E

---

### 4.5 設定ページ (Task 14)

**優先度:** 🟢 Nice to Have  
**見積もり:** 1-2日

#### タスク内容

1. **ページ専用コンポーネント実装**
   - [ ] `app/src/pages/settings/` ディレクトリ作成
   - [ ] `components/profile_settings_form.rs` - プロフィール設定
   - [ ] `components/privacy_settings.rs` - プライバシー設定
   - [ ] `components/notification_settings.rs` - 通知設定
   - [ ] `components/data_export_button.rs` - データエクスポート
   - [ ] `components/mod.rs` - 再エクスポート

2. **ページ本体実装**
   - [ ] `settings/mod.rs` - ページコンポーネント
   - [ ] フォームバリデーション
   - [ ] API連携

3. **ルーティング追加**
   - [ ] `app/src/lib.rs` に `/settings` ルート追加

**関連ドキュメント:**
- `docs/02_research/2025_11/20251121_screen-design-proposal.md` - セクション 2.1.I

---

### 4.6 Phase 3 完了条件

- [ ] GitHub Discussions を取得・表示できる
- [ ] 知見共有一覧が表示・検索できる
- [ ] ナレッジ詳細が表示される
- [ ] コメント・リアクションが表示される
- [ ] 設定ページが動作する

---

## 5. Phase 4: 機能拡張

### 目標
エクスポート機能、高度な検索、通知機能を提供する。

### タスク一覧

- [ ] **Task 15:** PDFエクスポート機能
- [ ] **Task 16:** JSON/CSV エクスポート機能
- [ ] **Task 17:** Notification Concept 実装
- [ ] **Task 18:** Notification Sync 実装
- [ ] **Task 19:** 高度な検索・フィルター実装
- [ ] **Task 20:** レスポンシブデザイン最適化
- [ ] **Task 21:** アクセシビリティ改善

---

## 6. 見積もりサマリー

| Phase | タスク数 | 見積もり工数 | 優先度 |
|-------|---------|-------------|--------|
| **Phase 1 (MVP)** | 4 | 5-7日 | 🔴 Critical |
| **Phase 2 (コミュニティ)** | 5 | 8-12日 | 🟡 Important |
| **Phase 3 (知見共有)** | 5 | 8-12日 | 🟢 Nice to Have |
| **Phase 4 (拡張)** | 7 | TBD | 🟢 Nice to Have |
| **合計** | 21 | 21-31日+ | - |

---

## 7. 次のアクションアイテム

### 即座に着手すべきタスク (Phase 1)

1. **Task 1: 既存コードのリファクタリング** (2-3時間)
   - Dashboard コンポーネント分割
   - Portfolio コンポーネント分割
   - テスト実行

2. **Task 2: GitHub OAuth 認証実装** (1-2日)
   - Auth Concept 実装
   - OAuth フロー実装
   - 認証ページ実装

3. **Task 3: 動的ルーティング対応** (4-6時間)
   - Leptos Router 設定
   - Portfolio ページ更新
   - 404ページ実装

4. **Task 4: ランディングページ刷新** (1日)
   - ページ専用コンポーネント実装
   - PRD準拠の内容に更新

### 推奨作業順序

```
Week 1:
  Day 1: Task 1 (リファクタリング) ✅
  Day 2-3: Task 2 (GitHub OAuth)
  Day 4: Task 3 (動的ルーティング)
  Day 5: Task 4 (ランディングページ)

Week 2-3:
  Phase 2 開始 (GitHub API, コントリビューター, リポジトリ)

Week 4-5:
  Phase 3 開始 (知見共有, 設定)

Week 6+:
  Phase 4 (機能拡張)
```

---

## 8. 参考資料

- **PRD**: `PRD.md`
- **画面設計**: `docs/02_research/2025_11/20251121_screen-design-proposal.md`
- **画面遷移図**: `docs/02_research/2025_11/20251121_screen-flow-diagram.md`
- **実装状況**: `docs/02_research/2025_11/20251121_implementation-status.md`
- **既存計画**: `docs/03_plans/continuum/20250101_01_setup-plan.md`

---

**Document Version:** 1.0  
**Last Updated:** 2025-11-21

