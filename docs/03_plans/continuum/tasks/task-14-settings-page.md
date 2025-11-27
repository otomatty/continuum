# Task 14: 設定ページ

## 1. 目的と背景

### なぜこのタスクが必要か
ユーザーが自身のプロフィール情報やアプリケーションの表示設定を管理できるページが必要です。

### 完成時のユーザー体験
- プロフィール情報（表示名、自己紹介など）を編集できる
- テーマ（ライト/ダーク）を切り替えられる
- 通知設定を管理できる
- アカウント連携状況（GitHub）を確認できる

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 2: GitHub OAuth 認証実装
- ✅ Task 3: 動的ルーティング対応

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/pages/settings/mod.rs` | ページコンポーネント（既存を拡充） |
| `app/src/pages/settings/components/mod.rs` | コンポーネント再エクスポート |
| `app/src/pages/settings/components/profile_section.rs` | プロフィール設定 |
| `app/src/pages/settings/components/appearance_section.rs` | 外観設定 |
| `app/src/pages/settings/components/account_section.rs` | アカウント設定 |
| `app/src/pages/settings/components/notification_section.rs` | 通知設定 |

---

## 4. 実装手順

### Step 1: ディレクトリ構造の作成

```bash
mkdir -p app/src/pages/settings/components
```

### Step 2: ProfileSection コンポーネント

`app/src/pages/settings/components/profile_section.rs`:

```rust
/**
 * ProfileSection Component
 */

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct ProfileData {
    pub display_name: String,
    pub bio: String,
    pub location: String,
    pub website: String,
}

impl Default for ProfileData {
    fn default() -> Self {
        Self {
            display_name: String::new(),
            bio: String::new(),
            location: String::new(),
            website: String::new(),
        }
    }
}

#[component]
pub fn ProfileSection(
    profile: ProfileData,
    on_save: Callback<ProfileData>,
) -> impl IntoView {
    let (display_name, set_display_name) = signal(profile.display_name.clone());
    let (bio, set_bio) = signal(profile.bio.clone());
    let (location, set_location) = signal(profile.location.clone());
    let (website, set_website) = signal(profile.website.clone());
    let (saving, set_saving) = signal(false);

    let handle_save = move |_| {
        set_saving.set(true);
        on_save.call(ProfileData {
            display_name: display_name.get(),
            bio: bio.get(),
            location: location.get(),
            website: website.get(),
        });
        // 保存完了後の処理は親コンポーネントで
    };

    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h2 class="card-title">"プロフィール"</h2>
                <p class="text-base-content/60 mb-4">
                    "公開プロフィールの情報を設定します"
                </p>

                <div class="space-y-4">
                    // 表示名
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"表示名"</span>
                        </label>
                        <input
                            type="text"
                            class="input input-bordered"
                            placeholder="山田 太郎"
                            prop:value=move || display_name.get()
                            on:input=move |ev| set_display_name.set(event_target_value(&ev))
                        />
                    </div>

                    // 自己紹介
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"自己紹介"</span>
                        </label>
                        <textarea
                            class="textarea textarea-bordered h-24"
                            placeholder="あなたについて教えてください..."
                            prop:value=move || bio.get()
                            on:input=move |ev| set_bio.set(event_target_value(&ev))
                        />
                    </div>

                    // 所在地
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"所在地"</span>
                        </label>
                        <input
                            type="text"
                            class="input input-bordered"
                            placeholder="東京都"
                            prop:value=move || location.get()
                            on:input=move |ev| set_location.set(event_target_value(&ev))
                        />
                    </div>

                    // ウェブサイト
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"ウェブサイト"</span>
                        </label>
                        <input
                            type="url"
                            class="input input-bordered"
                            placeholder="https://example.com"
                            prop:value=move || website.get()
                            on:input=move |ev| set_website.set(event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="card-actions justify-end mt-6">
                    <button
                        class="btn btn-primary"
                        disabled=saving
                        on:click=handle_save
                    >
                        {move || if saving.get() {
                            view! { <span class="loading loading-spinner loading-sm" /> }.into_any()
                        } else {
                            view! { "保存" }.into_any()
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}
```

### Step 3: AppearanceSection コンポーネント

`app/src/pages/settings/components/appearance_section.rs`:

```rust
/**
 * AppearanceSection Component
 */

use leptos::prelude::*;

#[component]
pub fn AppearanceSection(
    current_theme: String,
    on_theme_change: Callback<String>,
) -> impl IntoView {
    let themes = vec![
        ("light", "ライト", "☀️"),
        ("dark", "ダーク", "🌙"),
        ("system", "システム", "💻"),
    ];

    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h2 class="card-title">"外観"</h2>
                <p class="text-base-content/60 mb-4">
                    "アプリケーションの見た目をカスタマイズします"
                </p>

                <div class="space-y-4">
                    // テーマ選択
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"テーマ"</span>
                        </label>
                        <div class="flex gap-4">
                            {themes
                                .into_iter()
                                .map(|(value, label, emoji)| {
                                    let is_selected = current_theme == value;
                                    let value_for_click = value.to_string();
                                    let on_theme_change = on_theme_change.clone();

                                    view! {
                                        <label class="cursor-pointer">
                                            <input
                                                type="radio"
                                                name="theme"
                                                class="hidden"
                                                prop:checked=is_selected
                                                on:change=move |_| on_theme_change.call(value_for_click.clone())
                                            />
                                            <div class=move || format!(
                                                "card bg-base-100 border-2 p-4 text-center transition-colors {}",
                                                if is_selected { "border-primary" } else { "border-transparent hover:border-base-300" }
                                            )>
                                                <div class="text-2xl mb-2">{emoji}</div>
                                                <div class="font-medium">{label}</div>
                                            </div>
                                        </label>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
```

### Step 4: AccountSection コンポーネント

`app/src/pages/settings/components/account_section.rs`:

```rust
/**
 * AccountSection Component
 */

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub github_username: String,
    pub github_avatar_url: String,
    pub connected_at: String,
}

#[component]
pub fn AccountSection(
    account: AccountInfo,
    on_logout: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h2 class="card-title">"アカウント連携"</h2>
                <p class="text-base-content/60 mb-4">
                    "連携しているアカウントの管理"
                </p>

                // GitHub 連携状態
                <div class="flex items-center justify-between p-4 bg-base-100 rounded-lg">
                    <div class="flex items-center gap-4">
                        // GitHub アイコン
                        <div class="w-10 h-10 rounded-full bg-base-300 flex items-center justify-center">
                            <svg class="w-6 h-6" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                        </div>

                        <div>
                            <div class="font-medium">"GitHub"</div>
                            <div class="text-sm text-base-content/60">
                                "@" {account.github_username.clone()}
                            </div>
                        </div>
                    </div>

                    <div class="badge badge-success">"連携済み"</div>
                </div>

                // ログアウト
                <div class="divider" />

                <div>
                    <h3 class="font-medium mb-2">"ログアウト"</h3>
                    <p class="text-sm text-base-content/60 mb-4">
                        "すべてのデバイスからログアウトします"
                    </p>
                    <button
                        class="btn btn-outline btn-error btn-sm"
                        on:click=move |_| on_logout.call(())
                    >
                        "ログアウト"
                    </button>
                </div>
            </div>
        </div>
    }
}
```

### Step 5: NotificationSection コンポーネント

`app/src/pages/settings/components/notification_section.rs`:

```rust
/**
 * NotificationSection Component
 */

use leptos::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct NotificationSettings {
    pub email_new_discussion: bool,
    pub email_comment_reply: bool,
    pub email_weekly_digest: bool,
    pub browser_notifications: bool,
}

#[component]
pub fn NotificationSection(
    settings: NotificationSettings,
    on_change: Callback<NotificationSettings>,
) -> impl IntoView {
    let (local_settings, set_local_settings) = signal(settings);

    let update_setting = move |field: &'static str, value: bool| {
        set_local_settings.update(|s| {
            match field {
                "email_new_discussion" => s.email_new_discussion = value,
                "email_comment_reply" => s.email_comment_reply = value,
                "email_weekly_digest" => s.email_weekly_digest = value,
                "browser_notifications" => s.browser_notifications = value,
                _ => {}
            }
        });
        on_change.call(local_settings.get());
    };

    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h2 class="card-title">"通知設定"</h2>
                <p class="text-base-content/60 mb-4">
                    "通知の受け取り方を設定します"
                </p>

                <div class="space-y-4">
                    // メール通知
                    <div>
                        <h3 class="font-medium mb-3">"メール通知"</h3>
                        <div class="space-y-3">
                            <label class="flex items-center justify-between cursor-pointer">
                                <span class="label-text">"新しい知見が投稿されたとき"</span>
                                <input
                                    type="checkbox"
                                    class="toggle toggle-primary"
                                    prop:checked=move || local_settings.get().email_new_discussion
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        update_setting("email_new_discussion", checked);
                                    }
                                />
                            </label>
                            <label class="flex items-center justify-between cursor-pointer">
                                <span class="label-text">"コメントに返信があったとき"</span>
                                <input
                                    type="checkbox"
                                    class="toggle toggle-primary"
                                    prop:checked=move || local_settings.get().email_comment_reply
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        update_setting("email_comment_reply", checked);
                                    }
                                />
                            </label>
                            <label class="flex items-center justify-between cursor-pointer">
                                <span class="label-text">"週刊ダイジェスト"</span>
                                <input
                                    type="checkbox"
                                    class="toggle toggle-primary"
                                    prop:checked=move || local_settings.get().email_weekly_digest
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        update_setting("email_weekly_digest", checked);
                                    }
                                />
                            </label>
                        </div>
                    </div>

                    <div class="divider" />

                    // ブラウザ通知
                    <div>
                        <h3 class="font-medium mb-3">"ブラウザ通知"</h3>
                        <label class="flex items-center justify-between cursor-pointer">
                            <span class="label-text">"ブラウザでの通知を許可"</span>
                            <input
                                type="checkbox"
                                class="toggle toggle-primary"
                                prop:checked=move || local_settings.get().browser_notifications
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    update_setting("browser_notifications", checked);
                                }
                            />
                        </label>
                    </div>
                </div>
            </div>
        </div>
    }
}
```

### Step 6: components/mod.rs

```rust
mod account_section;
mod appearance_section;
mod notification_section;
mod profile_section;

pub use account_section::{AccountInfo, AccountSection};
pub use appearance_section::AppearanceSection;
pub use notification_section::{NotificationSection, NotificationSettings};
pub use profile_section::{ProfileData, ProfileSection};
```

### Step 7: ページコンポーネントの更新

`app/src/pages/settings/mod.rs`:

```rust
mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::container::Container;
use crate::concepts::theme::use_theme;
use components::{
    AccountInfo, AccountSection, AppearanceSection, NotificationSection, NotificationSettings,
    ProfileData, ProfileSection,
};
use leptos::prelude::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <AuthGuard>
            <SettingsContent />
        </AuthGuard>
    }
}

#[component]
fn SettingsContent() -> impl IntoView {
    let theme = use_theme();

    // モックデータ
    let mock_profile = ProfileData {
        display_name: "Alice Developer".to_string(),
        bio: "Rustが好きなエンジニアです".to_string(),
        location: "東京都".to_string(),
        website: "https://alice.dev".to_string(),
    };

    let mock_account = AccountInfo {
        github_username: "alice-dev".to_string(),
        github_avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
        connected_at: "2024-01-15".to_string(),
    };

    let mock_notifications = NotificationSettings {
        email_new_discussion: true,
        email_comment_reply: true,
        email_weekly_digest: false,
        browser_notifications: true,
    };

    let handle_profile_save = move |profile: ProfileData| {
        // TODO: Server Function でプロフィールを保存
        log::info!("Profile saved: {:?}", profile);
    };

    let handle_theme_change = move |new_theme: String| {
        theme.set_theme(&new_theme);
    };

    let handle_logout = move |_| {
        // ログアウト処理
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                let _ = window.location().set_href("/auth/logout");
            }
        }
    };

    let handle_notification_change = move |settings: NotificationSettings| {
        // TODO: Server Function で通知設定を保存
        log::info!("Notification settings changed: {:?}", settings);
    };

    view! {
        <Container>
            <div class="max-w-2xl mx-auto space-y-8">
                <div>
                    <h1 class="text-4xl font-bold mb-2">"設定"</h1>
                    <p class="text-base-content/70">
                        "アカウントとアプリケーションの設定"
                    </p>
                </div>

                // プロフィール
                <ProfileSection
                    profile=mock_profile
                    on_save=Callback::new(handle_profile_save)
                />

                // 外観
                <AppearanceSection
                    current_theme=theme.current_theme()
                    on_theme_change=Callback::new(handle_theme_change)
                />

                // 通知
                <NotificationSection
                    settings=mock_notifications
                    on_change=Callback::new(handle_notification_change)
                />

                // アカウント
                <AccountSection
                    account=mock_account
                    on_logout=Callback::new(handle_logout)
                />
            </div>
        </Container>
    }
}
```

---

## 5. 完了条件チェックリスト

- [ ] ProfileSection コンポーネントが実装されている
- [ ] AppearanceSection コンポーネントが実装されている
- [ ] AccountSection コンポーネントが実装されている
- [ ] NotificationSection コンポーネントが実装されている
- [ ] 設定ページが実装されている
- [ ] テーマ切り替えが動作する
- [ ] ログアウトが動作する
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- 画面設計: `docs/02_research/2025_11/20251121_screen-design-proposal.md`
- Theme Concept: `app/src/concepts/theme/`

