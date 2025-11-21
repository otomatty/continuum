# 実装状況リスト

**作成日:** 2025-11-21  
**関連ドキュメント:** 20251121_screen-design-proposal.md, PRD.md

## 1. ページ実装状況

### ✅ 実装済みページ (4画面)

| # | ページ名 | パス | ファイル | 説明 | 備考 |
|---|---------|------|---------|------|------|
| 1 | **ホームページ** | `/` | `app/src/pages/home/mod.rs` | 仮のランディングページ | 簡易的な実装、PRDのランディングページとは異なる |
| 2 | **ダッシュボード** | `/dashboard` | `app/src/pages/dashboard/mod.rs` | 組織活動のサマリー表示 | ✅ PRDの要件に対応、モックデータ使用 |
| 3 | **ポートフォリオ** | `/portfolio` | `app/src/pages/portfolio/mod.rs` | 個人の実績表示 | ✅ PRDの要件に対応、モックデータ使用 |
| 4 | **コンポーネント一覧** | `/components` | `app/src/pages/components/mod.rs` | UIコンポーネントショーケース | 開発者向け、PRDには含まれない |

### ❌ 未実装ページ (5画面)

PRDで定義されているが未実装のページ：

| # | ページ名 | パス | 優先度 | 説明 | 依存機能 |
|---|---------|------|--------|------|---------|
| 1 | **ランディングページ** | `/` | **Phase 1** | PRD準拠の本格的なランディング | Hero, Footer, Stats |
| 2 | **知見共有一覧** | `/knowledge` | Phase 3 | GitHub Discussions一覧 | GitHub API連携、検索・フィルター機能 |
| 3 | **ナレッジ詳細** | `/knowledge/:id` | Phase 3 | 個別記事の詳細表示 | Markdown レンダリング、コメント表示 |
| 4 | **コントリビューター一覧** | `/contributors` | Phase 2 | 貢献者の一覧表示 | Pagination, 検索・フィルター機能 |
| 5 | **リポジトリ一覧** | `/repositories` | Phase 2 | リポジトリの一覧表示 | Table, Pagination, 検索・フィルター機能 |
| 6 | **リポジトリ詳細** | `/repository/:owner/:repo` | Phase 2 | プロジェクトの詳細情報 | チャート、Timeline、GitHub API連携 |
| 7 | **設定ページ** | `/settings` | Phase 3 | ユーザー個人設定 | フォームバリデーション、API連携 |
| 8 | **404 Not Found** | `/404` | Phase 1 | エラーページ | - |
| 9 | **認証エラー** | `/auth/error` | Phase 1 | 認証失敗時のエラー | GitHub OAuth |

---

## 2. UIコンポーネント実装状況

### ✅ 実装済みコンポーネント (36個)

#### フォーム系 (7個)
- [x] **Input** - テキスト入力フィールド
- [x] **Textarea** - 複数行テキスト入力
- [x] **Select** - セレクトボックス
- [x] **Checkbox** - チェックボックス
- [x] **Radio** - ラジオボタン
- [x] **Toggle** - トグルスイッチ
- [x] **Range** - レンジスライダー

#### ボタン系 (1個)
- [x] **Button** - ボタン (Primary, Secondary, Ghost variants)

#### 表示系 (11個)
- [x] **Card** - カード (CardTitle, CardBody)
- [x] **Badge** - バッジ (Primary, Success, Warning, Error variants)
- [x] **Avatar** - アバター
- [x] **Alert** - アラート (Info, Success, Warning, Error variants)
- [x] **Progress** - プログレスバー
- [x] **Loading** - ローディングスピナー
- [x] **Skeleton** - スケルトンローディング
- [x] **Divider** - 区切り線
- [x] **Stats** - 統計情報表示
- [x] **Countdown** - カウントダウンタイマー
- [x] **Rating** - レーティング表示

#### ナビゲーション系 (7個)
- [x] **Navbar** - ナビゲーションバー
- [x] **Breadcrumbs** - パンくずリスト
- [x] **Pagination** - ページネーション
- [x] **Tabs** - タブ
- [x] **Steps** - ステップインジケーター
- [x] **Footer** - フッター
- [x] **Accordion** - アコーディオン

#### レイアウト系 (2個)
- [x] **Hero** - ヒーローセクション
- [x] **Table** - テーブル (TableHead, TableBody, TableRow, TableCell)

#### オーバーレイ系 (7個)
- [x] **Modal** - モーダルダイアログ
- [x] **Dropdown** - ドロップダウンメニュー
- [x] **Drawer** - サイドドロワー
- [x] **Tooltip** - ツールチップ
- [x] **Popover** - ポップオーバー
- [x] **Toast** - トースト通知

**備考:** 基本的なUIコンポーネントは全て実装済み。各コンポーネントには`.spec.md`仕様書が付属。

---

## 3. ページ専用コンポーネント実装状況

### ✅ 実装済み (ダッシュボードページ用)

| コンポーネント | ファイル | 説明 |
|-------------|---------|------|
| **StatsCard** | `app/src/pages/dashboard/components/stats_card.rs` | 統計情報カード |
| **RankingTable** | `app/src/pages/dashboard/components/ranking_table.rs` | ランキング表示テーブル |
| **ActivityTimeline** | `app/src/pages/dashboard/components/activity_timeline.rs` | アクティビティタイムライン |
| **RepositoryList** | `app/src/pages/dashboard/components/repository_list.rs` | リポジトリ一覧 |

**備考:** 各コンポーネントは個別ファイルに分割され、`components/mod.rs`で再エクスポートされています。

### ✅ 実装済み (ポートフォリオページ用)

| コンポーネント | ファイル | 説明 |
|-------------|---------|------|
| **UserProfile** | `app/src/pages/portfolio/components/user_profile.rs` | ユーザープロフィール表示 |
| **ContributionGraph** | `app/src/pages/portfolio/components/contribution_graph.rs` | コントリビューショングラフ |
| **ContributionHighlights** | `app/src/pages/portfolio/components/contribution_highlights.rs` | 貢献ハイライト |
| **RepositoryContributionList** | `app/src/pages/portfolio/components/repository_contribution_list.rs` | リポジトリ別コントリビューション一覧 |

**備考:** 各コンポーネントは個別ファイルに分割され、`components/mod.rs`で再エクスポートされています。

### ✅ 実装済み (ランディングページ用)

| コンポーネント | ファイル | 説明 |
|-------------|---------|------|
| **ValuePropositionCard** | `app/src/pages/home/components/value_proposition_card.rs` | 価値提案カード（3カラム） |
| **StatisticsPreview** | `app/src/pages/home/components/statistics_preview.rs` | 統計情報プレビュー（認証不要） |
| **CTASection** | `app/src/pages/home/components/cta_section.rs` | Call-to-Action セクション |

**備考:** 各コンポーネントは個別ファイルに分割され、`components/mod.rs`で再エクスポートされています。HomePageもPRD準拠のレイアウトに更新済み。

### ❌ 未実装 (今後必要になるページ専用コンポーネント)

#### 知見共有ページ用
- [ ] **KnowledgeCard** - ナレッジカード
- [ ] **SearchBar** - 検索バー
- [ ] **CategoryFilter** - カテゴリフィルター
- [ ] **AuthorFilter** - 投稿者フィルター

#### ナレッジ詳細ページ用
- [ ] **MarkdownRenderer** - Markdown レンダラー
- [ ] **CommentSection** - コメントセクション
- [ ] **ReactionButtons** - リアクションボタン
- [ ] **RelatedArticles** - 関連記事

#### コントリビューター一覧ページ用
- [ ] **ContributorCard** - コントリビューターカード
- [ ] **ContributorGrid** - グリッドレイアウト
- [ ] **StatusFilter** - ステータスフィルター (Current/Alumni/External)

#### リポジトリ一覧ページ用
- [ ] **RepositoryTable** - リポジトリテーブル（高機能版）
- [ ] **LanguageFilter** - 言語フィルター
- [ ] **SortControl** - ソートコントロール

#### リポジトリ詳細ページ用
- [ ] **ContributorPieChart** - コントリビューター構成円グラフ
- [ ] **TopContributorsList** - トップコントリビューター一覧
- [ ] **LanguageBarChart** - 言語構成バーチャート
- [ ] **RepositoryHeader** - リポジトリヘッダー

#### 設定ページ用
- [ ] **ProfileSettingsForm** - プロフィール設定フォーム
- [ ] **PrivacySettings** - プライバシー設定
- [ ] **NotificationSettings** - 通知設定
- [ ] **DataExportButton** - データエクスポートボタン

---

## 4. Concept実装状況

### ✅ 実装済み Concepts (6個)

| Concept | ファイル | 説明 | 仕様書 |
|---------|---------|------|--------|
| **User** | `app/src/concepts/user/` | ユーザー情報管理 | ✅ `user.spec.md` |
| **Activity** | `app/src/concepts/activity/` | アクティビティ管理 | ✅ `activity.spec.md` |
| **Repository** | `app/src/concepts/repository/` | リポジトリ情報管理 | ✅ `repository.spec.md` |
| **Contribution** | `app/src/concepts/contribution/` | コントリビューション管理 | ✅ `contribution.spec.md` |
| **Organization** | `app/src/concepts/organization/` | 組織統計情報管理 | ✅ `organization.spec.md` |
| **Ranking** | `app/src/concepts/ranking/` | ランキング情報管理 | ✅ `ranking.spec.md` |

### ❌ 未実装 Concepts (今後必要になる可能性)

| Concept | 説明 | 優先度 |
|---------|------|--------|
| **Discussion** | GitHub Discussions情報管理 | Phase 3 |
| **Comment** | コメント情報管理 | Phase 3 |
| **Reaction** | リアクション情報管理 | Phase 3 |
| **Notification** | 通知情報管理 | Phase 3 |
| **Search** | 検索状態管理 | Phase 2 |
| **Filter** | フィルター状態管理 | Phase 2 |
| **Auth** | 認証状態管理 | Phase 1 |

---

## 5. Synchronization実装状況

### ✅ 実装済み Synchronizations (1個)

| Synchronization | ファイル | 説明 | 仕様書 |
|----------------|---------|------|--------|
| **Ranking Sync** | `app/src/synchronizations/ranking_sync.rs` | User と Activity を連携してランキング計算 | ✅ `ranking.spec.md` |

### ❌ 未実装 Synchronizations (今後必要になる可能性)

| Synchronization | 説明 | 関連 Concepts | 優先度 |
|----------------|------|--------------|--------|
| **User Portfolio Sync** | User と Contribution を連携してポートフォリオ生成 | User, Contribution | Phase 1 |
| **Repository Activity Sync** | Repository と Activity を連携してリポジトリ活動集計 | Repository, Activity | Phase 2 |
| **Search Result Sync** | Search と各Concept を連携して検索結果生成 | Search, User, Repository, Discussion | Phase 2 |
| **Notification Sync** | Activity と Notification を連携して通知生成 | Activity, Notification | Phase 3 |

---

## 6. GitHub API連携状況

### ✅ 実装済み

| モジュール | ファイル | 説明 |
|-----------|---------|------|
| **GitHub Client** | `app/src/github/client.rs` | GitHub API クライアント |
| **GraphQL Queries** | `app/src/github/queries.rs` | GraphQL クエリ定義 |
| **Types** | `app/src/github/types.rs` | GitHub API 型定義 |

**備考:** 基本的な構造は実装済みだが、現在はモックデータを使用中。実際のAPI連携は未実装。

### ❌ 未実装機能

- [ ] **GitHub OAuth 認証フロー** (Phase 1)
- [ ] **リポジトリ一覧取得 API** (Phase 2)
- [ ] **ユーザー情報取得 API** (Phase 2)
- [ ] **コントリビューション取得 API** (Phase 2)
- [ ] **Discussions 取得 API** (Phase 3)
- [ ] **キャッシュ機構 (Cloudflare KV)** (Phase 2)

---

## 7. ルーティング実装状況

### ✅ 実装済みルート (4個)

```rust
// app/src/lib.rs
<Routes fallback=|| "Page not found.".into_view()>
    <Route path=StaticSegment("") view=HomePage/>
    <Route path=StaticSegment("dashboard") view=DashboardPage/>
    <Route path=StaticSegment("portfolio") view=PortfolioPage/>
    <Route path=StaticSegment("components") view=ComponentsPage/>
</Routes>
```

### ❌ 未実装ルート

動的パラメータを含むルート：

```rust
// 未実装
<Route path="/portfolio/:username" view=PortfolioPage/>
<Route path="/repository/:owner/:repo" view=RepositoryDetailPage/>
<Route path="/knowledge" view=KnowledgePage/>
<Route path="/knowledge/:id" view=KnowledgeDetailPage/>
<Route path="/contributors" view=ContributorsPage/>
<Route path="/repositories" view=RepositoriesPage/>
<Route path="/settings" view=SettingsPage/>
<Route path="/404" view=NotFoundPage/>
<Route path="/auth/error" view=AuthErrorPage/>
```

**備考:** 動的パラメータ (`:username`, `:owner/:repo`, `:id`) を含むルーティングは、Leptos Routerの設定が必要。現在のPortfolioPageは固定ユーザー名 `alice-dev` を使用中。

---

## 8. 実装優先順位と推奨ロードマップ

### Phase 1: MVP (Minimum Viable Product)

**目標:** PRD準拠の基本機能を提供

#### 1.1 認証機能 (最優先)
- [ ] GitHub OAuth 認証フロー実装
- [ ] Auth Concept 実装
- [ ] ログイン/ログアウト機能
- [ ] 認証エラーページ (`/auth/error`)

#### 1.2 ランディングページ刷新
- [x] PRD準拠のランディングページ実装 (`/`)
- [x] ValuePropositionCard コンポーネント
- [x] StatisticsPreview コンポーネント
- [x] CTASection コンポーネント
- [x] 既存のHomePageを置き換え

#### 1.3 ページ基盤整備
- [ ] 404 Not Found ページ (`/404`)
- [ ] グローバルナビゲーション改善
- [ ] 動的ルーティング対応 (`:username`, etc.)

**完了条件:** 認証済みユーザーがダッシュボードとポートフォリオを閲覧できる

---

### Phase 2: コミュニティ機能

**目標:** コントリビューター・リポジトリ一覧を提供

#### 2.1 GitHub API 実装
- [ ] リポジトリ一覧取得 API
- [ ] ユーザー情報取得 API
- [ ] コントリビューション取得 API
- [ ] キャッシュ機構 (Cloudflare KV)

#### 2.2 コントリビューター機能
- [ ] コントリビューター一覧ページ (`/contributors`)
- [ ] ContributorCard コンポーネント
- [ ] Search Concept 実装
- [ ] Filter Concept 実装
- [ ] 検索・フィルター機能

#### 2.3 リポジトリ機能
- [ ] リポジトリ一覧ページ (`/repositories`)
- [ ] リポジトリ詳細ページ (`/repository/:owner/:repo`)
- [ ] RepositoryTable コンポーネント
- [ ] ContributorPieChart コンポーネント
- [ ] LanguageBarChart コンポーネント

**完了条件:** 全てのリポジトリとコントリビューターを閲覧・検索できる

---

### Phase 3: 知見共有機能

**目標:** GitHub Discussions ベースのナレッジベースを提供

#### 3.1 Discussions API 連携
- [ ] Discussions 取得 API
- [ ] Discussion Concept 実装
- [ ] Comment Concept 実装
- [ ] Reaction Concept 実装

#### 3.2 知見共有ページ
- [ ] 知見共有一覧ページ (`/knowledge`)
- [ ] ナレッジ詳細ページ (`/knowledge/:id`)
- [ ] KnowledgeCard コンポーネント
- [ ] MarkdownRenderer コンポーネント
- [ ] CommentSection コンポーネント
- [ ] 検索・フィルター機能

#### 3.3 設定ページ
- [ ] 設定ページ (`/settings`)
- [ ] ProfileSettingsForm コンポーネント
- [ ] PrivacySettings コンポーネント
- [ ] NotificationSettings コンポーネント

**完了条件:** GitHub Discussions をプラットフォーム内で閲覧・検索できる

---

### Phase 4: 機能拡張

**目標:** エクスポート機能、高度な検索、通知機能を提供

- [ ] PDFエクスポート機能
- [ ] JSON/CSV エクスポート機能
- [ ] 通知機能 (Notification Concept)
- [ ] Notification Sync 実装
- [ ] 高度な検索・フィルター
- [ ] レスポンシブデザイン最適化
- [ ] アクセシビリティ改善

---

## 9. 現在の技術的課題

### 🔴 Critical (即座に対応が必要)

1. **GitHub OAuth 認証未実装**
   - 現状: 認証機能が存在しない
   - 影響: 実ユーザーのデータを扱えない
   - 対応: Phase 1 で最優先実装

2. **動的ルーティング未対応**
   - 現状: `:username`, `:owner/:repo` などの動的パラメータが動作しない
   - 影響: ポートフォリオページが固定ユーザーのみ
   - 対応: Leptos Router の動的ルート設定を実装

### 🟡 Important (Phase 1-2 で対応)

3. **GitHub API 未連携**
   - 現状: 全てモックデータ
   - 影響: 実データを表示できない
   - 対応: Phase 2 で API クライアント実装

4. **キャッシュ機構未実装**
   - 現状: キャッシュなし
   - 影響: GitHub API レート制限に抵触する可能性
   - 対応: Phase 2 で Cloudflare KV 連携

### 🟢 Nice to Have (Phase 3-4 で対応)

5. **Discussions 連携未実装**
   - 現状: 知見共有機能が存在しない
   - 影響: PRD の機能要件を満たせない
   - 対応: Phase 3 で実装

6. **通知機能未実装**
   - 現状: 通知機能が存在しない
   - 影響: ユーザー体験の低下
   - 対応: Phase 4 で実装

---

## 10. まとめ

### 実装状況サマリー

| カテゴリ | 実装済み | 未実装 | 進捗率 |
|---------|---------|--------|--------|
| **ページ** | 4 | 9 | 31% |
| **基本UIコンポーネント** | 36 | 0 | 100% |
| **ページ専用コンポーネント** | 11 | 22+ | 33% |
| **Concepts** | 6 | 7 | 46% |
| **Synchronizations** | 1 | 4 | 20% |
| **GitHub API連携** | 0 (構造のみ) | 6 | 0% |
| **ルーティング** | 4 (静的のみ) | 9 (動的含む) | 31% |

### 全体進捗率

**約 37%** (基本UIコンポーネントを除くと約 27%)

### 次のアクションアイテム

**Phase 1 (MVP) の優先タスク:**

1. ✅ **画面設計完了** (本ドキュメント)
2. ✅ **ランディングページ刷新** (ValuePropositionCard, StatisticsPreview, CTASection実装完了)
3. ⬜ **GitHub OAuth 認証実装** (最優先)
4. ⬜ **動的ルーティング対応**
5. ⬜ **404/Auth Error ページ実装**

**Phase 2 の準備:**

- GitHub API クライアント実装計画
- キャッシュ戦略の設計
- データモデルの見直し

---

## 11. 参考資料

- **PRD**: `PRD.md`
- **画面設計提案**: `docs/02_research/2025_11/20251121_screen-design-proposal.md`
- **画面遷移図**: `docs/02_research/2025_11/20251121_screen-flow-diagram.md`
- **実装計画**: `docs/03_plans/continuum/20250101_01_setup-plan.md`
- **コンポーネント一覧**: `app/src/components/UNIMPLEMENTED_COMPONENTS.md`

---

**Document Version:** 1.2  
**Last Updated:** 2025-11-21

**更新履歴:**
- v1.2 (2025-11-21): Phase 1ランディングページ用コンポーネント実装完了（ValuePropositionCard, StatisticsPreview, CTASection）を反映。ページ専用コンポーネント進捗率を24%→33%に更新。
- v1.1 (2025-11-21): ページ専用コンポーネントのファイルパスを更新（個別ファイル分割を反映）

