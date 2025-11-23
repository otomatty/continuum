# 認証状態管理の実装とGitHub OAuth設定

## 作成日
2025年11月23日

## 概要

認証済みユーザーに対して「GitHubでログイン」ではなく「ダッシュボードへ」ボタンを表示する機能を実装します。また、ネットワークアクセス頻度を最小限に抑えつつ、セキュアな認証状態の保持を実現します。

## 現状の問題点

1. **認証状態の管理不足**
   - `GitHubLoginButton`コンポーネントが認証状態を考慮していない
   - 認証済みユーザーでも「GitHubでログイン」が表示される
   - アプリケーション全体で認証状態を共有する仕組みがない

2. **ネットワークアクセスの非効率性**
   - 現在は`/api/auth/me`をクライアント側で呼び出しているが、毎回API呼び出しが発生する可能性がある
   - SSR（Server-Side Rendering）を活用していない

3. **GitHub OAuth設定の不明確さ**
   - GitHub側でのOAuth App設定手順が明確でない
   - 必要な環境変数の設定方法が不明確

## 実装計画（テスト駆動開発）

### テスト駆動開発の原則

各Phaseでは以下のサイクルに従って実装を進めます：

1. **Red**: テストケースを先に書く（失敗することを確認）
2. **Green**: テストが通る最小限の実装を行う
3. **Refactor**: コードの品質を向上させる
4. **Repeat**: 次のテストケースに進む

### Phase 1: SSRでの認証状態取得

**目的**: サーバー側でCookieから認証状態を取得し、初回ロード時のAPI呼び出しを不要にする

#### テストケース定義

**テストファイル**: `app/src/hooks/tests.rs`（新規作成）

**TC-001: 有効なセッションCookieがある場合、認証済み状態を返す**
- Given: 有効なセッションCookieが存在する
- When: `get_auth_status()`を呼び出す
- Then: `AuthStatus { authenticated: true, user_id: Some("user123") }`を返す

**TC-002: セッションCookieが存在しない場合、未認証状態を返す**
- Given: セッションCookieが存在しない
- When: `get_auth_status()`を呼び出す
- Then: `AuthStatus { authenticated: false, user_id: None }`を返す

**TC-003: 期限切れのセッションCookieがある場合、未認証状態を返す**
- Given: 期限切れのセッションCookieが存在する
- When: `get_auth_status()`を呼び出す
- Then: `AuthStatus { authenticated: false, user_id: None }`を返す

**TC-004: 不正な形式のセッションCookieがある場合、未認証状態を返す**
- Given: 不正な形式のセッションCookieが存在する
- When: `get_auth_status()`を呼び出す
- Then: `AuthStatus { authenticated: false, user_id: None }`を返す

#### 実装手順

1. **テストケースを書く（Red）**
   - `app/src/hooks/tests.rs`を作成
   - 上記のテストケースを実装
   - `cargo test`でテストが失敗することを確認

2. **最小限の実装を行う（Green）**
   - `app/src/hooks/use_auth.rs`を作成
   - `#[server(GetAuthStatus, "/api/auth/status")]`関数を実装
   - `leptos_axum::extract::RequestParts`からCookieを取得
   - `Session::from_cookie_value`で認証状態を確認
   - `AuthStatus`を返す
   - `cargo test`でテストが通ることを確認

3. **リファクタリング（Refactor）**
   - エラーハンドリングを改善
   - コードの可読性を向上
   - テストが引き続き通ることを確認

**技術的詳細**:
```rust
// app/src/hooks/use_auth.rs
#[server(GetAuthStatus, "/api/auth/status")]
pub async fn get_auth_status() -> Result<AuthStatus, ServerFnError> {
    use leptos_axum::extract::RequestParts;
    use axum_extra::extract::cookie::PrivateCookieJar;
    
    // RequestPartsからCookieを取得
    // Session::from_cookie_valueで認証状態を確認
    // AuthStatusを返す
}
```

### Phase 2: クライアント側の認証コンテキスト

**目的**: アプリケーション全体で認証状態を共有し、リアクティブに管理する

#### テストケース定義

**テストファイル**: `app/src/hooks/tests.rs`（Phase 1で作成したファイルに追加）

**TC-005: AuthContextが提供されている場合、use_auth()で取得できる**
- Given: `provide_auth_context()`が呼び出されている
- When: `use_auth()`を呼び出す
- Then: `AuthContext`が返される

**TC-006: AuthContextが提供されていない場合、use_auth()でエラーが発生する**
- Given: `provide_auth_context()`が呼び出されていない
- When: `use_auth()`を呼び出す
- Then: パニックが発生する（`expect`でエラーメッセージを表示）

**TC-007: set_status()で認証状態を更新できる**
- Given: `AuthContext`が提供されている
- When: `set_status(Some(AuthStatus { authenticated: true, user_id: Some("user123") }))`を呼び出す
- Then: `status()`で更新された認証状態が取得できる

**TC-008: HTMLから認証状態を読み取り、初期値として設定できる**
- Given: HTMLに`data-auth-status='{"authenticated":true,"user_id":"user123"}'`が埋め込まれている
- When: クライアント側で初期化処理を実行
- Then: `status()`で認証状態が正しく取得できる

#### 実装手順

1. **テストケースを書く（Red）**
   - `app/src/hooks/tests.rs`に上記のテストケースを追加
   - `cargo test`でテストが失敗することを確認

2. **最小限の実装を行う（Green）**
   - `app/src/hooks/use_auth.rs`に`AuthContext`を定義
   - `provide_auth_context()`を実装
   - `use_auth()`フックを実装
   - HTMLから認証状態を読み取る処理を実装
   - `cargo test`でテストが通ることを確認

3. **リファクタリング（Refactor）**
   - エラーハンドリングを改善
   - コードの可読性を向上
   - テストが引き続き通ることを確認

**技術的詳細**:
```rust
// app/src/hooks/use_auth.rs
#[derive(Clone)]
pub struct AuthContext {
    pub status: ReadSignal<Option<AuthStatus>>,
    pub set_status: WriteSignal<Option<AuthStatus>>,
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
        .expect("AuthContext should be provided")
}

pub fn provide_auth_context() -> AuthContext {
    let (status, set_status) = create_signal(None::<AuthStatus>);
    let context = AuthContext { status, set_status };
    provide_context(context.clone());
    context
}
```

### Phase 3: GitHubLoginButtonの更新

**目的**: 認証状態に応じてボタンの表示を切り替える

#### テストケース定義

**テストファイル**: `app/src/components/github_login_button/tests.rs`（新規作成）

**TC-009: 認証済みの場合、「ダッシュボードへ」ボタンが表示される**
- Given: 認証状態が`authenticated: true`である
- When: `GitHubLoginButton`コンポーネントをレンダリング
- Then: 「ダッシュボードへ」というテキストが表示される
- And: ボタンをクリックすると`/dashboard`にリダイレクトされる

**TC-010: 未認証の場合、「GitHubでログイン」ボタンが表示される**
- Given: 認証状態が`authenticated: false`または`None`である
- When: `GitHubLoginButton`コンポーネントをレンダリング
- Then: 「GitHubでログイン」というテキストが表示される
- And: GitHubアイコンが表示される
- And: ボタンをクリックすると`/auth/login`にリダイレクトされる

**TC-011: 認証状態が変更された場合、ボタンの表示が切り替わる**
- Given: 初期状態が未認証である
- When: 認証状態を`authenticated: true`に更新
- Then: 「ダッシュボードへ」ボタンに切り替わる

#### 実装手順

1. **テストケースを書く（Red）**
   - `app/src/components/github_login_button/tests.rs`を作成
   - 上記のテストケースを実装
   - `cargo test`でテストが失敗することを確認

2. **最小限の実装を行う（Green）**
   - `GitHubLoginButton`コンポーネントで`use_auth()`を使用
   - 認証状態に応じた条件分岐を実装
   - 「ダッシュボードへ」ボタンを実装
   - `cargo test`でテストが通ることを確認

3. **リファクタリング（Refactor）**
   - コンポーネントの構造を改善
   - コードの可読性を向上
   - テストが引き続き通ることを確認

**技術的詳細**:
```rust
// app/src/components/github_login_button/mod.rs
#[component]
pub fn GitHubLoginButton(...) -> impl IntoView {
    let auth = use_auth();
    let is_authenticated = move || {
        auth.status.get()
            .map(|s| s.authenticated)
            .unwrap_or(false)
    };
    
    view! {
        <Show
            when=is_authenticated
            fallback=|| view! {
                // GitHubでログインボタン
            }
        >
            // ダッシュボードへボタン
        </Show>
    }
}
```

### Phase 4: キャッシュと再検証

**目的**: ネットワークアクセスを最小限に抑えつつ、セキュリティを維持する

#### テストケース定義

**テストファイル**: `app/src/hooks/tests.rs`（Phase 2で作成したファイルに追加）

**TC-012: キャッシュ期間内の場合、追加のAPI呼び出しを行わない**
- Given: 認証状態がキャッシュされている（5分以内）
- When: 認証状態を取得しようとする
- Then: キャッシュされた値が返される
- And: `/api/auth/me`へのリクエストが発生しない

**TC-013: キャッシュ期間を過ぎた場合、再検証が行われる**
- Given: 認証状態がキャッシュされている（5分以上経過）
- When: 認証状態を取得しようとする
- Then: `/api/auth/me`へのリクエストが発生する
- And: 最新の認証状態が返される

**TC-014: ログアウト時、キャッシュが即座にクリアされる**
- Given: 認証状態がキャッシュされている
- When: ログアウト処理を実行
- Then: キャッシュがクリアされる
- And: 認証状態が`authenticated: false`になる

**TC-015: 初回ロード時、SSRで取得した値を使用する**
- Given: SSRで認証状態が取得されている
- When: クライアント側で初期化処理を実行
- Then: SSRで取得した値が初期値として使用される
- And: 追加のAPI呼び出しが発生しない

#### 実装手順

1. **テストケースを書く（Red）**
   - `app/src/hooks/tests.rs`に上記のテストケースを追加
   - モックを使用してAPI呼び出しを検証
   - `cargo test`でテストが失敗することを確認

2. **最小限の実装を行う（Green）**
   - キャッシュ期間の設定（5分）
   - キャッシュの実装
   - 再検証タイミングの実装
   - ログアウト時のキャッシュクリア
   - `cargo test`でテストが通ることを確認

3. **リファクタリング（Refactor）**
   - キャッシュロジックの最適化
   - エラーハンドリングの改善
   - コードの可読性を向上
   - テストが引き続き通ることを確認

**技術的詳細**:
```rust
// キャッシュの実装例
let cache_duration = Duration::minutes(5);
let last_check = create_rw_signal(None::<SystemTime>);

let should_refresh = move || {
    if let Some(last) = last_check.get() {
        SystemTime::now()
            .duration_since(last)
            .map(|d| d > cache_duration)
            .unwrap_or(true)
    } else {
        true
    }
};
```

## GitHub OAuth設定手順

### 1. GitHub OAuth Appの作成

**手順**:

1. **GitHubにログイン**
   - GitHubアカウントにログイン（組織アカウントまたは個人アカウント）

2. **Settingsにアクセス**
   - 個人アカウントの場合: 右上のプロフィール画像 → Settings
   - 組織アカウントの場合: 組織のSettingsページ

3. **Developer settingsに移動**
   - Settings → Developer settings → OAuth Apps

4. **New OAuth Appをクリック**
   - 「New OAuth App」ボタンをクリック

5. **OAuth App情報を入力**
   - **Application name**: `Continuum`（任意の名前）
   - **Homepage URL**: 
     - 開発環境: `http://localhost:3000`
     - 本番環境: `https://your-domain.com`
   - **Authorization callback URL**: 
     - 開発環境: `http://localhost:3000/auth/callback`
     - 本番環境: `https://your-domain.com/auth/callback`
   - **Application description**: （任意）

6. **Register applicationをクリック**
   - OAuth Appが作成され、`Client ID`と`Client Secret`が表示される

### 2. 環境変数の設定

**必要な環境変数**:

```bash
# GitHub OAuth設定
GITHUB_CLIENT_ID=your_client_id_here
GITHUB_CLIENT_SECRET=your_client_secret_here
GITHUB_OAUTH_CALLBACK_URL=http://localhost:3000/auth/callback  # 開発環境
# GITHUB_OAUTH_CALLBACK_URL=https://your-domain.com/auth/callback  # 本番環境
GITHUB_ORG_NAME=your_org_name  # 組織名（オプション）

# セッション設定
SESSION_SECRET=your_random_secret_key_here_min_64_chars
SESSION_DURATION_SECS=86400  # 24時間（秒）

# サーバー設定
LEPTOS_SITE_ADDR=127.0.0.1:3000
ENV=DEV  # DEV または PROD
```

**`.env`ファイルの作成**:

プロジェクトルートに`.env`ファイルを作成し、上記の環境変数を設定します。

**セキュリティ注意事項**:
- `SESSION_SECRET`は64文字以上のランダムな文字列を使用
- `.env`ファイルは`.gitignore`に追加されていることを確認
- `GITHUB_CLIENT_SECRET`は絶対に公開リポジトリにコミットしない

### 3. OAuth Appの権限設定

**必要なスコープ**:

現在の実装では以下のスコープを要求しています：
- `read:user`: ユーザー情報の読み取り
- `read:org`: 組織情報の読み取り
- `repo`: リポジトリ情報の読み取り

**スコープの確認**:
- OAuth App作成後、必要に応じてスコープを調整可能
- 最小権限の原則に従い、必要なスコープのみを要求

### 4. 本番環境での設定

**本番環境での注意事項**:

1. **HTTPS必須**
   - 本番環境では必ずHTTPSを使用
   - `secure: true`のCookie設定が有効になる

2. **Callback URLの設定**
   - 本番環境のURLを`GITHUB_OAUTH_CALLBACK_URL`に設定
   - GitHub OAuth Appの設定でも同じURLを設定

3. **環境変数の管理**
   - 本番環境では環境変数を安全に管理（例: AWS Secrets Manager、HashiCorp Vault）
   - CI/CDパイプラインで環境変数を設定

## セキュリティ考慮事項

### 現在の実装の強み

1. **HTTP-only Cookie**
   - JavaScriptからアクセス不可（XSS対策）

2. **PrivateCookieJar**
   - Cookieの暗号化（改ざん防止）

3. **SameSite: Lax**
   - CSRF対策

4. **セッション有効期限チェック**
   - 期限切れセッションの自動無効化

5. **CSRF保護**
   - OAuthフローでstateパラメータを使用

### 追加推奨事項

1. **セッションリフレッシュ**
   - アクティブな操作でセッションを延長
   - ユーザー体験の向上

2. **セッション無効化リスト**
   - ログアウト時の即時無効化
   - Redis等の外部ストレージを使用（将来的な改善）

3. **レート制限**
   - `/api/auth/me`エンドポイントにレート制限を実装
   - ブルートフォース攻撃の防止

4. **監査ログ**
   - 認証イベントのログ記録
   - セキュリティインシデントの調査に使用

## 実装チェックリスト（テスト駆動開発）

### Phase 1: SSRでの認証状態取得

**テストフェーズ（Red）**
- [ ] `app/src/hooks/tests.rs`を作成
- [ ] TC-001のテストケースを実装（有効なセッションCookie）
- [ ] TC-002のテストケースを実装（セッションCookieなし）
- [ ] TC-003のテストケースを実装（期限切れセッションCookie）
- [ ] TC-004のテストケースを実装（不正な形式のセッションCookie）
- [ ] `cargo test`でテストが失敗することを確認

**実装フェーズ（Green）**
- [ ] `app/src/hooks/use_auth.rs`を作成
- [ ] `AuthStatus`構造体を定義
- [ ] `#[server(GetAuthStatus)]`関数を実装
- [ ] Cookieから認証状態を取得する処理を実装
- [ ] HTMLに認証状態を埋め込む処理を実装
- [ ] `cargo test`でテストが通ることを確認

**リファクタリングフェーズ（Refactor）**
- [ ] エラーハンドリングを改善
- [ ] コードの可読性を向上
- [ ] テストが引き続き通ることを確認

### Phase 2: クライアント側の認証コンテキスト

**テストフェーズ（Red）**
- [ ] TC-005のテストケースを実装（AuthContextの取得）
- [ ] TC-006のテストケースを実装（AuthContextなしの場合のエラー）
- [ ] TC-007のテストケースを実装（認証状態の更新）
- [ ] TC-008のテストケースを実装（HTMLからの認証状態読み取り）
- [ ] `cargo test`でテストが失敗することを確認

**実装フェーズ（Green）**
- [ ] `AuthContext`を定義
- [ ] `provide_auth_context()`を実装
- [ ] `use_auth()`フックを実装
- [ ] `App`コンポーネントでコンテキストを初期化
- [ ] HTMLから認証状態を読み取る処理を実装
- [ ] `cargo test`でテストが通ることを確認

**リファクタリングフェーズ（Refactor）**
- [ ] エラーハンドリングを改善
- [ ] コードの可読性を向上
- [ ] テストが引き続き通ることを確認

### Phase 3: GitHubLoginButtonの更新

**テストフェーズ（Red）**
- [ ] `app/src/components/github_login_button/tests.rs`を作成
- [ ] TC-009のテストケースを実装（認証済みの場合の表示）
- [ ] TC-010のテストケースを実装（未認証の場合の表示）
- [ ] TC-011のテストケースを実装（認証状態変更時の表示切り替え）
- [ ] `cargo test`でテストが失敗することを確認

**実装フェーズ（Green）**
- [ ] `GitHubLoginButton`で`use_auth()`を使用
- [ ] 認証状態に応じた条件分岐を実装
- [ ] 「ダッシュボードへ」ボタンを実装
- [ ] `cargo test`でテストが通ることを確認

**リファクタリングフェーズ（Refactor）**
- [ ] コンポーネントの構造を改善
- [ ] コードの可読性を向上
- [ ] テストが引き続き通ることを確認

### Phase 4: キャッシュと再検証

**テストフェーズ（Red）**
- [ ] TC-012のテストケースを実装（キャッシュ期間内の動作）
- [ ] TC-013のテストケースを実装（キャッシュ期間経過後の再検証）
- [ ] TC-014のテストケースを実装（ログアウト時のキャッシュクリア）
- [ ] TC-015のテストケースを実装（SSR値の使用）
- [ ] `cargo test`でテストが失敗することを確認

**実装フェーズ（Green）**
- [ ] キャッシュ期間の設定（5分）
- [ ] キャッシュの実装
- [ ] 再検証タイミングの実装
- [ ] ログアウト時のキャッシュクリア
- [ ] `cargo test`でテストが通ることを確認

**リファクタリングフェーズ（Refactor）**
- [ ] キャッシュロジックの最適化
- [ ] エラーハンドリングの改善
- [ ] コードの可読性を向上
- [ ] テストが引き続き通ることを確認

### GitHub OAuth設定
- [ ] GitHub OAuth Appを作成
- [ ] 環境変数を設定
- [ ] 開発環境で動作確認
- [ ] 本番環境で設定（本番デプロイ時）

## 関連ファイル

### 新規作成
- `app/src/hooks/use_auth.rs`: 認証状態管理のフック
- `app/src/hooks/tests.rs`: 認証状態管理のテスト（Phase 1, 2, 4）
- `app/src/components/github_login_button/tests.rs`: GitHubLoginButtonのテスト（Phase 3）

### 更新
- `app/src/lib.rs`: 認証コンテキストの初期化
- `app/src/components/github_login_button/mod.rs`: 認証状態に応じた表示切り替え
- `server/src/auth/oauth.rs`: SSRでの認証状態取得（必要に応じて）

### 設定ファイル
- `.env`: 環境変数の設定
- `.env.example`: 環境変数のテンプレート（コミット可能）

## テスト実行方法

### 単体テストの実行

```bash
# すべてのテストを実行
cargo test

# 特定のテストファイルを実行
cargo test --test hooks
cargo test --test github_login_button

# 特定のテストケースを実行
cargo test TC-001
```

### テストカバレッジの確認

```bash
# cargo-tarpaulinを使用（インストールが必要）
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## テスト駆動開発のベストプラクティス

1. **テストを先に書く**
   - 実装前に必ずテストケースを定義する
   - テストが失敗することを確認してから実装を開始する

2. **小さなステップで進める**
   - 一度に1つのテストケースに集中する
   - テストが通る最小限の実装を行う

3. **リファクタリングを忘れない**
   - テストが通ったら、コードの品質を向上させる
   - リファクタリング後もテストが通ることを確認する

4. **テストの独立性**
   - 各テストケースは独立して実行できるようにする
   - テスト間で状態を共有しない

5. **明確なテスト名**
   - テストケース名は何をテストしているか明確にする
   - Given-When-Then形式でテストケースを記述する

## 参考資料

- [GitHub OAuth Apps Documentation](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps)
- [Leptos Server Functions](https://book.leptos.dev/server/25_server_functions.html)
- [Axum Cookie Extraction](https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/index.html)
- [OWASP Session Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html)

## 注意事項

### テスト駆動開発に関する注意事項

- **テストを先に書く**: 実装前に必ずテストケースを定義し、テストが失敗することを確認してから実装を開始してください
- **小さなステップで進める**: 一度に1つのテストケースに集中し、テストが通る最小限の実装を行ってください
- **リファクタリングを忘れない**: テストが通ったら、コードの品質を向上させ、テストが引き続き通ることを確認してください
- **テストの独立性**: 各テストケースは独立して実行できるようにし、テスト間で状態を共有しないでください

### 実装に関する注意事項

- **段階的な実装**: 本実装は段階的に進めることを推奨します
- **各Phaseの完了確認**: 各Phaseの実装後に動作確認を行い、問題がないことを確認してから次のPhaseに進んでください
- **テストの実行**: 各Phaseの実装後、必ず`cargo test`でテストが通ることを確認してください

### GitHub OAuth設定に関する注意事項

- **環境別の設定**: GitHub OAuth設定は開発環境と本番環境で別々に行う必要があります
- **環境変数の管理**: 環境変数は絶対に公開リポジトリにコミットしないでください
- **セキュリティ**: `SESSION_SECRET`は64文字以上のランダムな文字列を使用してください

