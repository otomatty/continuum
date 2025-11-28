# Discussion Concept Specification

## Related Files

- Implementation: `src/concepts/discussion/mod.rs`
- State: `src/concepts/discussion/state.rs`
- Actions: `src/concepts/discussion/actions.rs`
- Tests: `src/concepts/discussion/tests.rs`

## Related Documentation

- Plan: `docs/03_plans/continuum/tasks/task-10-discussion-concepts.md`
- PRD: `PRD.md` - セクション 5.2 知見共有機能
- Synchronizations:
  - (将来) discussion_category_sync: `src/synchronizations/discussion_category_sync.rs`

## Requirements

### 責務

- GitHub Discussions から取得した知見データの管理
- Discussion 一覧の保持・更新
- ページネーション状態の管理
- ローディング・エラー状態の管理

### 状態構造

- **DiscussionState**: メインの状態

  - `discussions: Vec<Discussion>` - Discussion 一覧
  - `loading: bool` - 読み込み中フラグ
  - `error: Option<String>` - エラーメッセージ
  - `pagination: PaginationInfo` - ページネーション情報

- **Discussion**: 個別の Discussion

  - `id: String` - GitHub Discussion ID
  - `title: String` - タイトル
  - `body: String` - 本文（Markdown 形式）
  - `body_preview: String` - 本文のプレビュー
  - `author: DiscussionAuthor` - 投稿者
  - `category: DiscussionCategory` - カテゴリ
  - `created_at: String` - 作成日時
  - `updated_at: String` - 更新日時
  - `comments_count: i32` - コメント数
  - `reactions_count: i32` - リアクション数
  - `url: String` - GitHub の URL
  - `labels: Vec<String>` - タグ

- **DiscussionAuthor**: 投稿者情報

  - `username: String`
  - `display_name: Option<String>`
  - `avatar_url: String`

- **DiscussionCategory**: カテゴリ情報

  - `id: String`
  - `name: String`
  - `description: Option<String>`
  - `emoji: Option<String>`

- **PaginationInfo**: ページネーション情報
  - `has_next_page: bool`
  - `has_previous_page: bool`
  - `end_cursor: Option<String>`
  - `start_cursor: Option<String>`
  - `total_count: i32`

### アクション

| アクション                | 説明                                              |
| ------------------------- | ------------------------------------------------- |
| `set_discussions`         | Discussion 一覧を設定し、loading/error をリセット |
| `append_discussions`      | 既存の一覧に追加（ページネーション用）            |
| `set_loading`             | ローディング状態を設定                            |
| `set_error`               | エラーを設定し、loading を false に               |
| `clear_error`             | エラーをクリア                                    |
| `set_pagination`          | ページネーション情報を更新                        |
| `find_discussion_by_id`   | ID で Discussion を検索                           |
| `filter_by_author`        | 投稿者でフィルタリング                            |
| `filter_by_category`      | カテゴリでフィルタリング                          |
| `generate_preview`        | 本文からプレビューを生成                          |
| `sort_by_created_at_desc` | 作成日時でソート（新しい順）                      |
| `sort_by_updated_at_desc` | 更新日時でソート（新しい順）                      |
| `sort_by_comments_desc`   | コメント数でソート（多い順）                      |
| `sort_by_reactions_desc`  | リアクション数でソート（多い順）                  |

## Test Cases

### TC-001: set_discussions

- **Given**: 空の DiscussionState
- **When**: `set_discussions(state, discussions)`を実行
- **Then**: discussions 配列が設定され、loading=false、error=None

### TC-002: append_discussions

- **Given**: 既存の Discussion を含む State
- **When**: `append_discussions(state, new_discussions)`を実行
- **Then**: 既存の discussions に新しい discussions が追加される

### TC-003: set_loading

- **Given**: 任意の DiscussionState
- **When**: `set_loading(state, true/false)`を実行
- **Then**: loading フラグが指定値に更新される

### TC-004: set_error

- **Given**: loading=true の State
- **When**: `set_error(state, "error message")`を実行
- **Then**: error が設定され、loading=false になる

### TC-005: clear_error

- **Given**: error が設定された State
- **When**: `clear_error(state)`を実行
- **Then**: error=None になる

### TC-006: set_pagination

- **Given**: 任意の DiscussionState
- **When**: `set_pagination(state, pagination)`を実行
- **Then**: pagination が更新される

### TC-007: find_discussion_by_id - found

- **Given**: 複数の Discussion を含む配列
- **When**: 存在する ID で`find_discussion_by_id`を実行
- **Then**: 該当する Discussion が返される

### TC-008: find_discussion_by_id - not found

- **Given**: 複数の Discussion を含む配列
- **When**: 存在しない ID で`find_discussion_by_id`を実行
- **Then**: None が返される

### TC-009: filter_by_author

- **Given**: 異なる投稿者の Discussion を含む配列
- **When**: `filter_by_author(discussions, "username")`を実行
- **Then**: 指定した投稿者の Discussion のみが返される

### TC-010: filter_by_category

- **Given**: 異なるカテゴリの Discussion を含む配列
- **When**: `filter_by_category(discussions, "category_id")`を実行
- **Then**: 指定したカテゴリの Discussion のみが返される

### TC-011: generate_preview - short text

- **Given**: max_length より短いテキスト
- **When**: `generate_preview(body, max_length)`を実行
- **Then**: そのままのテキストが返される

### TC-012: generate_preview - long text

- **Given**: max_length より長いテキスト
- **When**: `generate_preview(body, max_length)`を実行
- **Then**: max_length 文字で切り詰められ、"..."が付加される

### TC-013: generate_preview - markdown filtering

- **Given**: Markdown 記法を含むテキスト（#見出し、`コード`など）
- **When**: `generate_preview(body, max_length)`を実行
- **Then**: Markdown 記法が除去されたプレーンテキストが返される

### TC-014: sort_by_created_at_desc

- **Given**: 異なる作成日時の Discussion 配列
- **When**: `sort_by_created_at_desc(discussions)`を実行
- **Then**: 作成日時の新しい順にソートされる

### TC-015: sort_by_updated_at_desc

- **Given**: 異なる更新日時の Discussion 配列
- **When**: `sort_by_updated_at_desc(discussions)`を実行
- **Then**: 更新日時の新しい順にソートされる

### TC-016: sort_by_comments_desc

- **Given**: 異なるコメント数の Discussion 配列
- **When**: `sort_by_comments_desc(discussions)`を実行
- **Then**: コメント数の多い順にソートされる

### TC-017: sort_by_reactions_desc

- **Given**: 異なるリアクション数の Discussion 配列
- **When**: `sort_by_reactions_desc(discussions)`を実行
- **Then**: リアクション数の多い順にソートされる
