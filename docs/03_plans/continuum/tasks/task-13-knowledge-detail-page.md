# Task 13: ナレッジ詳細ページ

## 1. 目的と背景

### なぜこのタスクが必要か
PRDの「5.2 知見共有機能」に記載されている「プラットフォーム内でのコンテンツ閲覧機能」を実装します。GitHub Discussions の本文（Markdown）をレンダリングして表示します。

### 完成時のユーザー体験
- 知見の本文が Markdown としてレンダリングされて表示される
- コメント一覧が表示される
- GitHub への直接リンクが提供される
- 関連する知見（同じカテゴリ）が表示される

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 11: GitHub Discussions API 連携
- ✅ Task 12: 知見共有一覧ページ

### 必要な知識
- Markdown レンダリング
- Leptos の動的ルーティング

---

## 3. 作成ファイル一覧

| ファイル | 内容 |
|---------|------|
| `app/src/pages/knowledge_detail/mod.rs` | ページコンポーネント |
| `app/src/pages/knowledge_detail/components/mod.rs` | コンポーネント再エクスポート |
| `app/src/pages/knowledge_detail/components/article_header.rs` | 記事ヘッダー |
| `app/src/pages/knowledge_detail/components/article_content.rs` | 記事本文 |
| `app/src/pages/knowledge_detail/components/comment_list.rs` | コメント一覧 |
| `app/src/pages/knowledge_detail/components/related_articles.rs` | 関連記事 |

### 更新ファイル
| ファイル | 変更内容 |
|---------|---------|
| `app/src/pages/mod.rs` | `knowledge_detail` モジュール追加 |
| `app/src/lib.rs` | `/knowledge/:id` ルート追加 |

---

## 4. 実装手順

### Step 1: ディレクトリ構造の作成

```bash
mkdir -p app/src/pages/knowledge_detail/components
```

### Step 2: ArticleHeader コンポーネント

`app/src/pages/knowledge_detail/components/article_header.rs`:

```rust
/**
 * ArticleHeader Component
 */

use crate::concepts::discussion::Discussion;
use leptos::prelude::*;

#[component]
pub fn ArticleHeader(discussion: Discussion) -> impl IntoView {
    let formatted_date = format_date(&discussion.created_at);
    
    let category_display = discussion.category.emoji
        .as_ref()
        .map(|e| format!("{} {}", e, discussion.category.name))
        .unwrap_or_else(|| discussion.category.name.clone());

    view! {
        <header class="mb-8">
            // カテゴリとラベル
            <div class="flex flex-wrap items-center gap-2 mb-4">
                <span class="badge badge-primary">
                    {category_display}
                </span>
                {discussion.labels.iter().map(|label| {
                    view! {
                        <span class="badge badge-outline">{label.clone()}</span>
                    }
                }).collect_view()}
            </div>

            // タイトル
            <h1 class="text-4xl font-bold mb-6">
                {discussion.title.clone()}
            </h1>

            // 投稿者情報
            <div class="flex items-center justify-between flex-wrap gap-4">
                <div class="flex items-center gap-4">
                    // アバター
                    <div class="avatar">
                        <div class="w-12 h-12 rounded-full">
                            <img
                                src=discussion.author.avatar_url.clone()
                                alt=discussion.author.username.clone()
                            />
                        </div>
                    </div>
                    // 名前と日付
                    <div>
                        <p class="font-medium">
                            {discussion.author.display_name.clone()
                                .unwrap_or_else(|| discussion.author.username.clone())}
                        </p>
                        <p class="text-sm text-base-content/60">
                            {formatted_date}
                        </p>
                    </div>
                </div>

                // アクション
                <div class="flex items-center gap-2">
                    // リアクション
                    <div class="flex items-center gap-1 text-base-content/60">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                        </svg>
                        <span>{discussion.reactions_count}</span>
                    </div>
                    // コメント数
                    <div class="flex items-center gap-1 text-base-content/60">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                        </svg>
                        <span>{discussion.comments_count}</span>
                    </div>
                    // GitHub リンク
                    <a
                        href=discussion.url.clone()
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn-outline btn-sm"
                    >
                        "GitHubで見る"
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                        </svg>
                    </a>
                </div>
            </div>
        </header>
    }
}

fn format_date(date_str: &str) -> String {
    if let Some(date_part) = date_str.split('T').next() {
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}年{}月{}日", parts[0], parts[1], parts[2]);
        }
    }
    date_str.to_string()
}
```

### Step 3: ArticleContent コンポーネント

`app/src/pages/knowledge_detail/components/article_content.rs`:

```rust
/**
 * ArticleContent Component
 *
 * Markdown コンテンツを表示するコンポーネント
 * GitHub API から bodyHTML を取得する場合はそれを使用
 * それ以外の場合は prose クラスでスタイリング
 */

use leptos::prelude::*;

#[component]
pub fn ArticleContent(
    /// Markdown 形式の本文（bodyHTML がない場合のフォールバック）
    body: String,
    /// GitHub がレンダリングした HTML（存在する場合）
    #[prop(optional)]
    body_html: Option<String>,
) -> impl IntoView {
    view! {
        <article class="prose prose-lg max-w-none">
            {match body_html {
                Some(html) => {
                    // GitHub がレンダリングした HTML を使用
                    view! {
                        <div inner_html=html />
                    }.into_any()
                }
                None => {
                    // Markdown をプレーンテキストとして表示（将来的にはMarkdownレンダラーを使用）
                    view! {
                        <div class="whitespace-pre-wrap">
                            {body}
                        </div>
                    }.into_any()
                }
            }}
        </article>
    }
}
```

### Step 4: CommentList コンポーネント

`app/src/pages/knowledge_detail/components/comment_list.rs`:

```rust
/**
 * CommentList Component
 */

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub author_username: String,
    pub author_avatar_url: String,
    pub body: String,
    pub body_html: Option<String>,
    pub created_at: String,
    pub reactions_count: i32,
}

#[component]
pub fn CommentList(comments: Vec<Comment>) -> impl IntoView {
    view! {
        <section class="mt-12 pt-8 border-t border-base-300">
            <h2 class="text-2xl font-bold mb-6">
                "コメント "
                <span class="text-base-content/60">
                    "(" {comments.len()} ")"
                </span>
            </h2>

            {if comments.is_empty() {
                view! {
                    <p class="text-base-content/60 py-8 text-center">
                        "まだコメントがありません"
                    </p>
                }.into_any()
            } else {
                view! {
                    <div class="space-y-6">
                        {comments
                            .into_iter()
                            .map(|comment| {
                                view! { <CommentItem comment=comment /> }
                            })
                            .collect_view()}
                    </div>
                }.into_any()
            }}
        </section>
    }
}

#[component]
fn CommentItem(comment: Comment) -> impl IntoView {
    let formatted_date = format_date(&comment.created_at);

    view! {
        <div class="flex gap-4">
            // アバター
            <div class="avatar shrink-0">
                <div class="w-10 h-10 rounded-full">
                    <img
                        src=comment.author_avatar_url.clone()
                        alt=comment.author_username.clone()
                    />
                </div>
            </div>

            // コメント本文
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-2">
                    <span class="font-medium">{comment.author_username.clone()}</span>
                    <span class="text-sm text-base-content/60">{formatted_date}</span>
                </div>

                {match comment.body_html {
                    Some(html) => {
                        view! {
                            <div class="prose prose-sm max-w-none" inner_html=html />
                        }.into_any()
                    }
                    None => {
                        view! {
                            <div class="prose prose-sm max-w-none whitespace-pre-wrap">
                                {comment.body}
                            </div>
                        }.into_any()
                    }
                }}

                {(comment.reactions_count > 0).then(|| {
                    view! {
                        <div class="flex items-center gap-1 mt-2 text-sm text-base-content/60">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                            </svg>
                            <span>{comment.reactions_count}</span>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

fn format_date(date_str: &str) -> String {
    if let Some(date_part) = date_str.split('T').next() {
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}年{}月{}日", parts[0], parts[1], parts[2]);
        }
    }
    date_str.to_string()
}
```

### Step 5: RelatedArticles コンポーネント

`app/src/pages/knowledge_detail/components/related_articles.rs`:

```rust
/**
 * RelatedArticles Component
 */

use crate::concepts::discussion::Discussion;
use leptos::prelude::*;

#[component]
pub fn RelatedArticles(
    articles: Vec<Discussion>,
    #[prop(optional)] on_click: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <aside class="mt-12 pt-8 border-t border-base-300">
            <h2 class="text-2xl font-bold mb-6">"関連する知見"</h2>

            {if articles.is_empty() {
                view! {
                    <p class="text-base-content/60">"関連する知見がありません"</p>
                }.into_any()
            } else {
                view! {
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {articles
                            .into_iter()
                            .map(|article| {
                                let id = article.id.clone();
                                let callback = on_click.clone();

                                let handle_click = move |_| {
                                    if let Some(cb) = &callback {
                                        cb.call(id.clone());
                                    }
                                };

                                view! {
                                    <div
                                        class="card bg-base-200 hover:bg-base-300 cursor-pointer transition-colors"
                                        on:click=handle_click
                                    >
                                        <div class="card-body p-4">
                                            <h3 class="card-title text-base line-clamp-2">
                                                {article.title.clone()}
                                            </h3>
                                            <p class="text-sm text-base-content/60 line-clamp-2">
                                                {article.body_preview.clone()}
                                            </p>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                }.into_any()
            }}
        </aside>
    }
}
```

### Step 6: components/mod.rs

```rust
mod article_content;
mod article_header;
mod comment_list;
mod related_articles;

pub use article_content::ArticleContent;
pub use article_header::ArticleHeader;
pub use comment_list::{Comment, CommentList};
pub use related_articles::RelatedArticles;
```

### Step 7: ページコンポーネント

`app/src/pages/knowledge_detail/mod.rs`:

```rust
mod components;

use crate::components::auth_guard::AuthGuard;
use crate::components::breadcrumbs::Breadcrumbs;
use crate::components::container::Container;
use crate::concepts::discussion::{Discussion, DiscussionAuthor, DiscussionCategory};
use components::{ArticleContent, ArticleHeader, Comment, CommentList, RelatedArticles};
use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_params_map};

#[component]
pub fn KnowledgeDetailPage() -> impl IntoView {
    view! {
        <AuthGuard>
            <KnowledgeDetailContent />
        </AuthGuard>
    }
}

#[component]
fn KnowledgeDetailContent() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let id = move || params.get().get("id").unwrap_or_default();

    // モックデータ（Task 11完了後にServer Functionに置換）
    let mock_discussion = Discussion {
        id: id(),
        title: "Rust で非同期処理を効率的に書くコツ".to_string(),
        body: r#"# Rust での非同期処理

## はじめに

Rust の async/await は非常に強力ですが、いくつかのコツを知っておくと、より効率的なコードが書けます。

## 1. Tokio の使い方

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("開始");
    sleep(Duration::from_secs(1)).await;
    println!("1秒経過");
}
```

## 2. 並行処理

`tokio::join!` マクロを使うと、複数の非同期処理を並行して実行できます。

## まとめ

Rust の非同期処理は学習曲線が急ですが、マスターすれば非常に効率的なコードが書けます。
"#.to_string(),
        body_preview: "Rust の async/await の使い方について解説します。".to_string(),
        author: DiscussionAuthor {
            username: "alice-dev".to_string(),
            display_name: Some("Alice Developer".to_string()),
            avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alice".to_string(),
        },
        category: DiscussionCategory {
            id: "cat-1".to_string(),
            name: "Tips & Tricks".to_string(),
            description: None,
            emoji: Some("💡".to_string()),
        },
        created_at: "2025-11-20T10:00:00Z".to_string(),
        updated_at: "2025-11-20T10:00:00Z".to_string(),
        comments_count: 2,
        reactions_count: 24,
        url: "https://github.com/org/repo/discussions/1".to_string(),
        labels: vec!["Rust".to_string(), "async".to_string()],
    };

    let mock_comments = vec![
        Comment {
            id: "comment-1".to_string(),
            author_username: "bob-dev".to_string(),
            author_avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
            body: "とても参考になりました！特に `tokio::join!` の使い方は知りませんでした。".to_string(),
            body_html: None,
            created_at: "2025-11-21T09:00:00Z".to_string(),
            reactions_count: 5,
        },
        Comment {
            id: "comment-2".to_string(),
            author_username: "charlie".to_string(),
            author_avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Charlie".to_string(),
            body: "エラーハンドリングについても書いていただけると嬉しいです。".to_string(),
            body_html: None,
            created_at: "2025-11-22T14:30:00Z".to_string(),
            reactions_count: 2,
        },
    ];

    let mock_related = vec![
        Discussion {
            id: "disc-2".to_string(),
            title: "Leptos で状態管理をシンプルにする方法".to_string(),
            body: "".to_string(),
            body_preview: "Leptos フレームワークにおける状態管理のベストプラクティス".to_string(),
            author: DiscussionAuthor {
                username: "bob-dev".to_string(),
                display_name: None,
                avatar_url: "https://api.dicebear.com/7.x/avataaars/svg?seed=Bob".to_string(),
            },
            category: DiscussionCategory {
                id: "cat-1".to_string(),
                name: "Tips & Tricks".to_string(),
                description: None,
                emoji: Some("💡".to_string()),
            },
            created_at: "2025-11-18T14:30:00Z".to_string(),
            updated_at: "2025-11-18T14:30:00Z".to_string(),
            comments_count: 12,
            reactions_count: 45,
            url: "".to_string(),
            labels: vec![],
        },
    ];

    let breadcrumb_items = vec![
        ("知見共有".to_string(), Some("/knowledge".to_string())),
        (mock_discussion.title.chars().take(30).collect::<String>() + "...", None),
    ];

    let handle_related_click = move |id: String| {
        navigate(&format!("/knowledge/{}", id), Default::default());
    };

    view! {
        <Container>
            <div class="max-w-4xl mx-auto">
                <Breadcrumbs items=breadcrumb_items />

                <ArticleHeader discussion=mock_discussion.clone() />

                <ArticleContent
                    body=mock_discussion.body.clone()
                    body_html=None
                />

                <CommentList comments=mock_comments />

                <RelatedArticles
                    articles=mock_related
                    on_click=Callback::new(handle_related_click)
                />
            </div>
        </Container>
    }
}
```

### Step 8: ルーティングの追加

`app/src/lib.rs`:

```rust
use pages::KnowledgeDetailPage;

// Routes 内に追加
<Route path=path!("/knowledge/:id") view=KnowledgeDetailPage/>
```

---

## 5. 完了条件チェックリスト

- [ ] ArticleHeader コンポーネントが実装されている
- [ ] ArticleContent コンポーネントが実装されている
- [ ] CommentList コンポーネントが実装されている
- [ ] RelatedArticles コンポーネントが実装されている
- [ ] ページコンポーネントが実装されている
- [ ] 動的ルーティング（`:id`）が設定されている
- [ ] パンくずリストが表示される
- [ ] 関連記事をクリックすると遷移する
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- PRD: `PRD.md` - セクション 5.2
- Task 12: `task-12-knowledge-list-page.md`

