# Phase 1-2: GitHub OAuth認証実装

## 概要

GitHub OAuth 2.0を使用した認証フローを実装し、ユーザーのGitHubアカウントでログインできるようにします。

## 目標

- OAuth 2.0認証フローの実装
- セッション管理（Cookie/Token）
- 認証ミドルウェアの作成
- 保護されたルートの実装

## 実装内容

### 1. ディレクトリ構造

```
server/src/auth/
├── mod.rs              # モジュール定義
├── oauth.rs            # OAuthフロー実装
├── session.rs          # セッション管理
└── middleware.rs       # 認証ミドルウェア
```

### 2. 実装タスク

#### 2.1 OAuthフロー実装 (`oauth.rs`)

**機能:**
- 認証URL生成
- コールバック処理
- アクセストークン取得
- ユーザー情報取得

**実装項目:**
- `OAuthConfig` 構造体（環境変数から読み込み）
- `/auth/login` エンドポイント
- `/auth/callback` エンドポイント
- `/auth/logout` エンドポイント
- GitHub APIを使用したユーザー情報取得

**OAuthフロー:**
```
1. ユーザーが /auth/login にアクセス
2. GitHub認証ページにリダイレクト
3. ユーザーが認証を承認
4. GitHubが /auth/callback にリダイレクト（code付き）
5. codeをアクセストークンに交換
6. アクセストークンでユーザー情報取得
7. セッション作成
8. ダッシュボードにリダイレクト
```

#### 2.2 セッション管理 (`session.rs`)

**機能:**
- セッション生成・検証
- Cookie管理
- セッション有効期限管理

**実装項目:**
- `Session` 構造体（ユーザーID、トークン、有効期限）
- セッション生成関数
- セッション検証関数
- Cookie設定・取得関数

**セキュリティ考慮:**
- HttpOnly Cookie使用
- Secure Cookie（HTTPS時）
- SameSite属性設定
- セッションIDのランダム生成

#### 2.3 認証ミドルウェア (`middleware.rs`)

**機能:**
- リクエストの認証状態チェック
- 保護されたルートへのアクセス制御

**実装項目:**
- `AuthMiddleware` 実装
- 認証不要ルートの設定
- 認証エラー時のリダイレクト処理

**保護対象ルート:**
- `/api/*` （一部を除く）
- `/dashboard`
- `/portfolio/*`

**公開ルート:**
- `/`
- `/auth/*`
- `/api/public/*`

### 3. 依存関係

```toml
# server/Cargo.toml に追加
[dependencies]
oauth2 = "4.5"
cookie = "0.21"
axum-extra = { version = "0.9", features = ["cookie"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 4. 環境変数

```bash
# .env
GITHUB_CLIENT_ID=your_client_id
GITHUB_CLIENT_SECRET=your_client_secret
GITHUB_OAUTH_CALLBACK_URL=http://localhost:3000/auth/callback
SESSION_SECRET=your_random_secret_key
```

### 5. GitHub OAuth App設定

**設定手順:**
1. GitHub Settings > Developer settings > OAuth Apps
2. "New OAuth App" をクリック
3. 以下を設定:
   - Application name: Continuum
   - Homepage URL: `http://localhost:3000`
   - Authorization callback URL: `http://localhost:3000/auth/callback`
4. Client ID と Client Secret を取得

**必要なスコープ:**
- `read:org` - 組織情報の読み取り
- `read:user` - ユーザー情報の読み取り
- `repo` - リポジトリ情報の読み取り（公開リポジトリのみ）

### 6. エラーハンドリング

**エラー種別:**
- OAuth認証エラー
- トークン交換エラー
- セッション無効エラー
- 権限不足エラー

### 7. テスト

**テスト項目:**
- OAuthフローの統合テスト
- セッション管理テスト
- ミドルウェアテスト
- エラーハンドリングテスト

## 実装手順

1. GitHub OAuth App作成
2. `server/src/auth/` ディレクトリ作成
3. 環境変数設定
4. `oauth.rs` でOAuthフロー実装
5. `session.rs` でセッション管理実装
6. `middleware.rs` で認証ミドルウェア実装
7. ルーティングに認証ミドルウェア適用
8. テスト作成

## セキュリティ考慮事項

- CSRF対策（stateパラメータ使用）
- セッション固定攻撃対策（セッションID再生成）
- XSS対策（HttpOnly Cookie）
- トークンの安全な保存

## 関連ドキュメント

- 親計画: `../20250101_01_setup-plan.md`
- GitHub API クライアント: `../phase1-github-api-client/README.md`

