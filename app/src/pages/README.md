# Pages ディレクトリ構造

このディレクトリは、Next.jsのコロケーション（ファイルを機能ごとに近くに配置する）の考え方に基づいて、ページとその関連コンポーネントを整理しています。

## ディレクトリ構造

```
app/src/pages/
├── mod.rs                    # ページモジュールのエクスポート
├── home/
│   └── mod.rs                # HomePage コンポーネント
├── dashboard/                # 将来追加予定
│   ├── mod.rs                # DashboardPage コンポーネント
│   └── components.rs         # DashboardPage固有のコンポーネント（あれば）
└── portfolio/                # 将来追加予定
    ├── mod.rs                # PortfolioPage コンポーネント
    └── components.rs         # PortfolioPage固有のコンポーネント（あれば）
```

## 配置ルール

### 1. ページコンポーネント

各ページは `pages/{page_name}/mod.rs` に配置します。

**例: `pages/home/mod.rs`**
```rust
use leptos::prelude::*;
use crate::components::{button::Button, card::Card};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        // ページの内容
    }
}
```

### 2. ページ固有のコンポーネント

そのページでのみ使用されるコンポーネントは、同じディレクトリに `components.rs` として配置します。

**例: `pages/dashboard/components.rs`**
```rust
use leptos::prelude::*;

#[component]
pub fn RepositoryCard(repo: Repository) -> impl IntoView {
    // DashboardPageでのみ使用されるコンポーネント
}
```

**例: `pages/dashboard/mod.rs`**
```rust
mod components;
use components::RepositoryCard;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <RepositoryCard repo=... />
    }
}
```

### 3. 共通コンポーネント

複数のページで使用されるコンポーネントは `app/src/components/` に配置します。

**共通コンポーネントの例:**
- `Button` - 複数のページで使用
- `Card` - 複数のページで使用
- `Navbar` - 全体で使用

## 新しいページの追加方法

### ステップ1: ページディレクトリとファイルを作成

```bash
mkdir -p app/src/pages/dashboard
touch app/src/pages/dashboard/mod.rs
```

### ステップ2: ページコンポーネントを実装

**`app/src/pages/dashboard/mod.rs`**
```rust
use leptos::prelude::*;
use crate::components::card::Card;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Dashboard"</h1>
            <Card>
                // コンテンツ
            </Card>
        </div>
    }
}
```

### ステップ3: `pages/mod.rs` にエクスポートを追加

**`app/src/pages/mod.rs`**
```rust
pub mod home;
pub mod dashboard;  // 追加

pub use home::HomePage;
pub use dashboard::DashboardPage;  // 追加
```

### ステップ4: `lib.rs` にルートを追加

**`app/src/lib.rs`**
```rust
use pages::DashboardPage;  // 追加

// ...

<Routes>
    <Route path=StaticSegment("") view=HomePage/>
    <Route path=StaticSegment("dashboard") view=DashboardPage/>  // 追加
</Routes>
```

## ベストプラクティス

### ✅ 推奨される構造

```
pages/
├── dashboard/
│   ├── mod.rs              # DashboardPage
│   ├── components.rs       # DashboardPage固有のコンポーネント
│   └── hooks.rs            # DashboardPage固有のフック（あれば）
└── portfolio/
    ├── mod.rs              # PortfolioPage
    └── components.rs       # PortfolioPage固有のコンポーネント
```

### ❌ 避けるべき構造

```
pages/
├── dashboard.rs            # フラットな構造（関連ファイルが見つけにくい）
└── dashboard_components.rs # ページとコンポーネントが分離しすぎている
```

## 依存関係の管理

各ページファイルの先頭に、DEPENDENCY MAP を記載してください：

```rust
/**
 * Dashboard Page
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ app/src/lib.rs (ルーティング)
 *
 * Dependencies (External files that this file imports):
 *   ├─ app/src/components/card.rs
 *   ├─ app/src/components/button.rs
 *   └─ app/src/github/client.rs
 *
 * Related Documentation:
 *   ├─ Spec: app/src/pages/dashboard/dashboard.spec.md
 *   └─ Plan: docs/03_plans/continuum/phase4-dashboard-page/
 */
```

## 関連ドキュメント

- [プロジェクト全体の開発規則](../../../docs/rules/README.md)
- [依存関係マッピング規則](../../../docs/rules/dependency-mapping.md)

