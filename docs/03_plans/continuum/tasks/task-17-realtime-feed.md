# Task 17: リアルタイムフィード

## 1. 目的と背景

### なぜこのタスクが必要か
ダッシュボードにリアルタイム性を持たせ、組織内のアクティビティをライブで表示することで、コミュニティの活発さを可視化します。

### 完成時のユーザー体験
- 新しいアクティビティが自動的に追加される
- 新着通知がリアルタイムで表示される
- ページをリロードせずに最新情報が得られる

---

## 2. 前提条件

### 依存するタスク
- ✅ Task 16: ダッシュボード強化

### 必要な知識
- WebSocket または Server-Sent Events
- Leptos のリアクティブシステム

---

## 3. 実装方針

### 技術選択
**Server-Sent Events (SSE)** を採用します：

- WebSocket より実装がシンプル
- 一方向通信で十分（サーバー → クライアント）
- HTTP/2 との相性が良い

### アーキテクチャ

```
[GitHub Webhook] → [Server] → [SSE Endpoint] → [Client]
                       ↓
                  [Event Queue]
```

---

## 4. 実装手順

### Step 1: イベント型の定義

`app/src/concepts/realtime/state.rs`:

```rust
use serde::{Deserialize, Serialize};

/// リアルタイムイベントの種類
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RealtimeEvent {
    /// 新しいコミットが追加された
    NewCommit {
        repository: String,
        author: String,
        message: String,
        sha: String,
    },
    /// 新しいPRが作成された
    NewPullRequest {
        repository: String,
        author: String,
        title: String,
        number: i32,
    },
    /// PRがマージされた
    PullRequestMerged {
        repository: String,
        author: String,
        title: String,
        number: i32,
    },
    /// 新しいIssueが作成された
    NewIssue {
        repository: String,
        author: String,
        title: String,
        number: i32,
    },
    /// 新しいDiscussionが投稿された
    NewDiscussion {
        repository: String,
        author: String,
        title: String,
        id: String,
    },
}

impl RealtimeEvent {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::NewCommit { .. } => "📝",
            Self::NewPullRequest { .. } => "🔀",
            Self::PullRequestMerged { .. } => "✅",
            Self::NewIssue { .. } => "📌",
            Self::NewDiscussion { .. } => "💬",
        }
    }

    pub fn title(&self) -> String {
        match self {
            Self::NewCommit { message, .. } => message.clone(),
            Self::NewPullRequest { title, .. } => title.clone(),
            Self::PullRequestMerged { title, .. } => title.clone(),
            Self::NewIssue { title, .. } => title.clone(),
            Self::NewDiscussion { title, .. } => title.clone(),
        }
    }

    pub fn author(&self) -> &str {
        match self {
            Self::NewCommit { author, .. } => author,
            Self::NewPullRequest { author, .. } => author,
            Self::PullRequestMerged { author, .. } => author,
            Self::NewIssue { author, .. } => author,
            Self::NewDiscussion { author, .. } => author,
        }
    }

    pub fn repository(&self) -> &str {
        match self {
            Self::NewCommit { repository, .. } => repository,
            Self::NewPullRequest { repository, .. } => repository,
            Self::PullRequestMerged { repository, .. } => repository,
            Self::NewIssue { repository, .. } => repository,
            Self::NewDiscussion { repository, .. } => repository,
        }
    }
}
```

### Step 2: SSE クライアントフック

`app/src/hooks/use_realtime.rs`:

```rust
/**
 * useRealtime Hook
 *
 * Server-Sent Events を使用してリアルタイムイベントを受信
 */

use crate::concepts::realtime::RealtimeEvent;
use leptos::prelude::*;

#[derive(Clone)]
pub struct RealtimeContext {
    pub events: ReadSignal<Vec<RealtimeEvent>>,
    pub connected: ReadSignal<bool>,
    pub error: ReadSignal<Option<String>>,
}

pub fn use_realtime() -> RealtimeContext {
    let (events, set_events) = signal(Vec::<RealtimeEvent>::new());
    let (connected, set_connected) = signal(false);
    let (error, set_error) = signal(None::<String>);

    #[cfg(feature = "hydrate")]
    {
        use leptos::task::spawn_local;
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use web_sys::EventSource;

        spawn_local(async move {
            let event_source = match EventSource::new("/api/events") {
                Ok(es) => es,
                Err(_) => {
                    set_error.set(Some("Failed to connect".to_string()));
                    return;
                }
            };

            // 接続成功
            let on_open = Closure::wrap(Box::new(move || {
                set_connected.set(true);
            }) as Box<dyn Fn()>);
            event_source.set_onopen(Some(on_open.as_ref().unchecked_ref()));
            on_open.forget();

            // メッセージ受信
            let on_message = Closure::wrap(Box::new(move |ev: web_sys::MessageEvent| {
                if let Some(data) = ev.data().as_string() {
                    if let Ok(event) = serde_json::from_str::<RealtimeEvent>(&data) {
                        set_events.update(|events| {
                            events.insert(0, event);
                            // 最大50件保持
                            if events.len() > 50 {
                                events.pop();
                            }
                        });
                    }
                }
            }) as Box<dyn Fn(web_sys::MessageEvent)>);
            event_source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
            on_message.forget();

            // エラー
            let on_error = Closure::wrap(Box::new(move || {
                set_connected.set(false);
                set_error.set(Some("Connection lost".to_string()));
            }) as Box<dyn Fn()>);
            event_source.set_onerror(Some(on_error.as_ref().unchecked_ref()));
            on_error.forget();
        });
    }

    RealtimeContext {
        events: events.into(),
        connected: connected.into(),
        error: error.into(),
    }
}
```

### Step 3: LiveFeed コンポーネント

`app/src/components/live_feed/mod.rs`:

```rust
/**
 * LiveFeed Component
 *
 * リアルタイムでイベントを表示するフィードコンポーネント
 */

use crate::hooks::use_realtime;
use leptos::prelude::*;

#[component]
pub fn LiveFeed() -> impl IntoView {
    let realtime = use_realtime();

    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="card-title">
                        "📡 Live Feed"
                    </h2>
                    <div class="flex items-center gap-2">
                        {move || {
                            if realtime.connected.get() {
                                view! {
                                    <span class="badge badge-success badge-sm gap-1">
                                        <span class="w-2 h-2 rounded-full bg-success animate-pulse" />
                                        "接続中"
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <span class="badge badge-error badge-sm gap-1">
                                        "切断"
                                    </span>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>

                <div class="space-y-2 max-h-80 overflow-y-auto">
                    {move || {
                        let events = realtime.events.get();
                        if events.is_empty() {
                            view! {
                                <div class="text-center text-base-content/60 py-8">
                                    "イベントを待機中..."
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                {events
                                    .into_iter()
                                    .map(|event| {
                                        view! {
                                            <div class="flex items-start gap-3 p-3 bg-base-100 rounded-lg animate-fade-in">
                                                <span class="text-xl">{event.icon()}</span>
                                                <div class="flex-1 min-w-0">
                                                    <p class="text-sm font-medium truncate">
                                                        {event.title()}
                                                    </p>
                                                    <p class="text-xs text-base-content/60">
                                                        {event.author()}
                                                        " • "
                                                        {event.repository()}
                                                    </p>
                                                </div>
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                            }.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
```

### Step 4: サーバーサイド SSE エンドポイント

`server/src/events/mod.rs`:

```rust
use axum::{
    response::sse::{Event, Sse},
    Extension,
};
use futures::stream::{self, Stream};
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::StreamExt;

pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // 実際の実装では、Redis Pub/Sub や内部チャンネルからイベントを受信
    let stream = stream::repeat_with(|| {
        // デモ用：定期的にダミーイベントを送信
        Event::default()
            .data(r#"{"type":"NewCommit","repository":"continuum","author":"demo","message":"Demo commit","sha":"abc123"}"#)
    })
    .map(Ok)
    .throttle(Duration::from_secs(30));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}
```

---

## 5. 完了条件チェックリスト

- [ ] RealtimeEvent 型が定義されている
- [ ] use_realtime フックが実装されている
- [ ] LiveFeed コンポーネントが実装されている
- [ ] SSE エンドポイントが実装されている
- [ ] 接続状態が表示される
- [ ] 新しいイベントがリアルタイムで表示される
- [ ] ビルドエラーがない

---

## 6. 参照ドキュメント

- MDN Server-Sent Events: https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events
- Axum SSE: https://docs.rs/axum/latest/axum/response/sse/

---

## 7. 注意点

- **パフォーマンス**: 多数のクライアントが接続する場合はスケーリングを考慮
- **再接続**: EventSource は自動再接続するが、エラー時のUI表示を適切に
- **セキュリティ**: 認証済みユーザーのみがSSEエンドポイントにアクセスできるように

