# Category Concept Specification

## Related Files

- Implementation: `src/concepts/category/mod.rs`
- State: `src/concepts/category/state.rs`
- Actions: `src/concepts/category/actions.rs`
- Tests: `src/concepts/category/tests.rs`

## Related Documentation

- Plan: `docs/03_plans/continuum/tasks/task-10-discussion-concepts.md`
- PRD: `PRD.md` - セクション 5.2 知見共有機能
- Related Concepts:
  - Discussion: `src/concepts/discussion/discussion.spec.md`

## Requirements

### 責務

- GitHub Discussions のカテゴリデータの管理
- カテゴリ一覧の保持・更新
- カテゴリ選択状態の管理
- ローディング・エラー状態の管理

### 状態構造

- **CategoryState**: メインの状態

  - `categories: Vec<Category>` - カテゴリ一覧
  - `selected_category_id: Option<String>` - 選択中のカテゴリ ID
  - `loading: bool` - 読み込み中フラグ
  - `error: Option<String>` - エラーメッセージ

- **Category**: 個別のカテゴリ
  - `id: String` - カテゴリ ID
  - `name: String` - カテゴリ名
  - `description: Option<String>` - 説明
  - `emoji: Option<String>` - 絵文字アイコン
  - `discussions_count: i32` - このカテゴリの Discussion 数

### アクション

| アクション                       | 説明                                           |
| -------------------------------- | ---------------------------------------------- |
| `set_categories`                 | カテゴリ一覧を設定し、loading/error をリセット |
| `select_category`                | カテゴリを選択（None でクリ A ア）             |
| `set_loading`                    | ローディング状態を設定                         |
| `set_error`                      | エラーを設定し、loading を false に            |
| `clear_error`                    | エラーをクリア                                 |
| `find_category_by_id`            | ID でカテゴリを検索                            |
| `find_category_by_name`          | 名前でカテゴリを検索                           |
| `sort_by_discussions_count_desc` | Discussion 数でソート（多い順）                |
| `sort_by_name`                   | 名前でソート（昇順）                           |
| `get_selected_category`          | 選択中のカテゴリを取得                         |
| `add_category`                   | カテゴリを追加                                 |
| `update_discussions_count`       | カテゴリの Discussion 数を更新                 |

### メソッド

| メソッド                   | 説明                          |
| -------------------------- | ----------------------------- |
| `Category::display_name()` | 表示名を取得（絵文字 + 名前） |

## Test Cases

### TC-001: set_categories

- **Given**: 空の CategoryState
- **When**: `set_categories(state, categories)`を実行
- **Then**: categories 配列が設定され、loading=false、error=None

### TC-002: select_category

- **Given**: カテゴリを含む State
- **When**: `select_category(state, Some("id"))`を実行
- **Then**: selected_category_id が設定される

### TC-003: select_category - deselect

- **Given**: カテゴリが選択された State
- **When**: `select_category(state, None)`を実行
- **Then**: selected_category_id=None

### TC-004: set_loading

- **Given**: 任意の CategoryState
- **When**: `set_loading(state, true/false)`を実行
- **Then**: loading フラグが指定値に更新される

### TC-005: set_error

- **Given**: loading=true の State
- **When**: `set_error(state, "error message")`を実行
- **Then**: error が設定され、loading=false になる

### TC-006: clear_error

- **Given**: error が設定された State
- **When**: `clear_error(state)`を実行
- **Then**: error=None になる

### TC-007: find_category_by_id - found

- **Given**: 複数のカテゴリを含む配列
- **When**: 存在する ID で`find_category_by_id`を実行
- **Then**: 該当するカテゴリが返される

### TC-008: find_category_by_id - not found

- **Given**: 複数のカテゴリを含む配列
- **When**: 存在しない ID で`find_category_by_id`を実行
- **Then**: None が返される

### TC-009: find_category_by_name - found

- **Given**: 複数のカテゴリを含む配列
- **When**: 存在する名前で`find_category_by_name`を実行
- **Then**: 該当するカテゴリが返される

### TC-010: find_category_by_name - not found

- **Given**: 複数のカテゴリを含む配列
- **When**: 存在しない名前で`find_category_by_name`を実行
- **Then**: None が返される

### TC-011: sort_by_discussions_count_desc

- **Given**: 異なる Discussion 数のカテゴリ配列
- **When**: `sort_by_discussions_count_desc(categories)`を実行
- **Then**: Discussion 数の多い順にソートされる

### TC-012: sort_by_name

- **Given**: 異なる名前のカテゴリ配列
- **When**: `sort_by_name(categories)`を実行
- **Then**: 名前の昇順にソートされる

### TC-013: Category display_name with emoji

- **Given**: 絵文字を持つ Category
- **When**: `category.display_name()`を実行
- **Then**: "絵文字 名前"形式の文字列が返される

### TC-014: Category display_name without emoji

- **Given**: 絵文字を持たない Category
- **When**: `category.display_name()`を実行
- **Then**: 名前のみが返される

### TC-015: get_selected_category - found

- **Given**: カテゴリが選択された State
- **When**: `get_selected_category(&state)`を実行
- **Then**: 選択中のカテゴリが返される

### TC-016: get_selected_category - not selected

- **Given**: カテゴリが選択されていない State
- **When**: `get_selected_category(&state)`を実行
- **Then**: None が返される

### TC-017: add_category

- **Given**: カテゴリを含む State
- **When**: `add_category(state, new_category)`を実行
- **Then**: 新しいカテゴリが追加される

### TC-018: update_discussions_count

- **Given**: カテゴリを含む State
- **When**: `update_discussions_count(state, category_id, count)`を実行
- **Then**: 指定したカテゴリの discussions_count が更新される
