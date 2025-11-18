# Phase 4-1: レイアウトコンポーネント実装

## 概要

アプリケーション全体で使用する共通レイアウトコンポーネントを実装します。

## 目標

- ヘッダー/ナビゲーションコンポーネント
- 認証状態の表示
- レスポンシブデザイン対応
- アクセシビリティ対応

## 実装内容

### 1. ディレクトリ構造

```
app/src/components/
├── mod.rs              # モジュール定義
├── layout/
│   ├── mod.rs
│   ├── header.rs       # ヘッダーコンポーネント
│   ├── navigation.rs   # ナビゲーションコンポーネント
│   └── footer.rs       # フッターコンポーネント
└── auth/
    ├── mod.rs
    └── auth_status.rs  # 認証状態表示
```

### 2. ヘッダーコンポーネント (`layout/header.rs`)

**機能:**
- ロゴ表示
- ナビゲーションメニュー
- 認証状態表示
- モバイルメニュー（ハンバーガーメニュー）

**実装:**
```rust
#[component]
pub fn Header() -> impl IntoView {
    let auth_state = use_auth_state();
    
    view! {
        <header class="header">
            <div class="header-container">
                <Logo />
                <Navigation />
                <AuthStatus />
            </div>
        </header>
    }
}
```

**スタイル要件:**
- 固定ヘッダー（スクロール時も表示）
- レスポンシブ対応
- ダークモード対応（オプション）

### 3. ナビゲーションコンポーネント (`layout/navigation.rs`)

**機能:**
- メインナビゲーションリンク
- アクティブ状態の表示
- モバイルメニュー

**ナビゲーション項目:**
- ホーム (`/`)
- ダッシュボード (`/dashboard`)
- ポートフォリオ (`/portfolio`)
- 知見共有 (`/knowledge`)

**実装:**
```rust
#[component]
pub fn Navigation() -> impl IntoView {
    let location = use_location();
    
    view! {
        <nav class="navigation">
            <ul class="nav-list">
                <NavItem href="/" label="Home" />
                <NavItem href="/dashboard" label="Dashboard" />
                <NavItem href="/portfolio" label="Portfolio" />
                <NavItem href="/knowledge" label="Knowledge" />
            </ul>
        </nav>
    }
}
```

### 4. 認証状態表示 (`auth/auth_status.rs`)

**機能:**
- ログイン状態の表示
- ログイン/ログアウトボタン
- ユーザーアバター表示

**実装:**
```rust
#[component]
pub fn AuthStatus() -> impl IntoView {
    let auth_state = use_auth_state();
    
    view! {
        <div class="auth-status">
            {move || match auth_state.get() {
                Some(user) => view! {
                    <div class="user-menu">
                        <img src={user.avatar_url} alt="Avatar" />
                        <span>{user.login}</span>
                        <LogoutButton />
                    </div>
                }.into_view(),
                None => view! {
                    <LoginButton />
                }.into_view(),
            }}
        </div>
    }
}
```

### 5. フッターコンポーネント (`layout/footer.rs`)

**機能:**
- コピーライト表示
- リンク（利用規約、プライバシーポリシー等）
- ソーシャルリンク（オプション）

**実装:**
```rust
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="footer-container">
                <p>© 2024 Continuum. All rights reserved.</p>
                <nav class="footer-nav">
                    <a href="/terms">Terms</a>
                    <a href="/privacy">Privacy</a>
                </nav>
            </div>
        </footer>
    }
}
```

### 6. レイアウトコンポーネント (`layout/mod.rs`)

**機能:**
- 全体レイアウトの統合
- ヘッダー・フッターの配置
- メインコンテンツエリア

**実装:**
```rust
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="layout">
            <Header />
            <main class="main-content">
                {children()}
            </main>
            <Footer />
        </div>
    }
}
```

### 7. スタイリング

**実装方針:**
- Tailwind CSSを使用（メモリに基づく）
- コンポーネントごとのスタイルモジュール
- レスポンシブデザイン

**スタイル要件:**
- モバイルファースト
- アクセシビリティ（ARIA属性）
- ダークモード対応（オプション）

### 8. ルーティング統合

**実装:**
```rust
// app/src/lib.rs
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Layout>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/dashboard" view=DashboardPage />
                    // ...
                </Routes>
            </Layout>
        </Router>
    }
}
```

### 9. テスト

**テスト項目:**
- コンポーネントのレンダリングテスト
- ナビゲーション動作テスト
- 認証状態の表示テスト
- レスポンシブデザインテスト

## 実装手順

1. `app/src/components/` ディレクトリ作成
2. `layout/` ディレクトリ作成
3. `header.rs` でヘッダーコンポーネント実装
4. `navigation.rs` でナビゲーション実装
5. `auth_status.rs` で認証状態表示実装
6. `footer.rs` でフッター実装
7. `layout/mod.rs` でレイアウト統合
8. スタイリング
9. ルーティングに統合
10. テスト作成

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- ダッシュボードページ: `../phase4-dashboard-page/README.md`
- ポートフォリオページ: `../phase4-portfolio-page/README.md`

