# Pagination Component Specification

## Related Files

- Implementation: `app/src/components/pagination/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/pagination/

## Requirements

### 責務

- ページネーションの表示と管理
- DaisyUIの`pagination`クラスを使用したスタイリング
- ページ番号の表示とナビゲーション
- 前後ページへの移動ボタン

### Props構造

#### Paginationコンポーネント

- `current_page` (usize): 現在のページ番号（必須）
- `total_pages` (usize): 総ページ数（必須）
- `on_page_change` (optional, Callback<usize>): ページ変更イベントのコールバック
- `class` (optional, String): 追加のCSSクラス

### スタイル

- `Pagination`: `pagination`クラスを適用した`<div>`要素
- ページ番号ボタン: `btn`クラスを適用、アクティブなページは`btn-active`クラスを追加
- 前後ボタン: `btn`クラスを適用、無効時は`disabled`属性を設定

### 機能

- ページ番号ボタンの表示（1からtotal_pagesまで）
- 現在のページをハイライト表示
- 前のページへ移動するボタン（«）
- 次のページへ移動するボタン（»）
- 最初のページでは前へボタンを無効化
- 最後のページでは次へボタンを無効化

## Test Cases

### TC-001: Paginationの基本表示

- Given: current_page=1, total_pages=5
- When: Paginationコンポーネントをレンダリング
- Then: `pagination`クラスが適用された`<div>`要素が表示される

### TC-002: ページ番号の表示

- Given: current_page=1, total_pages=5
- When: Paginationコンポーネントをレンダリング
- Then: 1から5までのページ番号ボタンが表示される

### TC-003: アクティブなページのハイライト

- Given: current_page=3, total_pages=5
- When: Paginationコンポーネントをレンダリング
- Then: ページ番号3のボタンに`btn-active`クラスが適用される

### TC-004: 前へボタンの有効化

- Given: current_page=2, total_pages=5
- When: Paginationコンポーネントをレンダリング
- Then: 前へボタン（«）が有効（disabled属性なし）

### TC-005: 前へボタンの無効化

- Given: current_page=1, total_pages=5
- When: Paginationコンポーネントをレンダリング
- Then: 前へボタン（«）が無効（disabled属性あり）

### TC-006: 次へボタンの有効化

- Given: current_page=1, total_pages=5
- When: Paginationコンポーネントをレンダリング
- Then: 次へボタン（»）が有効（disabled属性なし）

### TC-007: 次へボタンの無効化

- Given: current_page=5, total_pages=5
- When: Paginationコンポーネントをレンダリング
- Then: 次へボタン（»）が無効（disabled属性あり）

### TC-008: ページ変更イベント

- Given: Paginationにon_page_changeコールバックを設定
- When: ユーザーがページ番号ボタンをクリック
- Then: on_page_changeコールバックが呼び出され、選択されたページ番号が渡される

### TC-009: 前へボタンのクリックイベント

- Given: current_page=3, total_pages=5, on_page_changeコールバックを設定
- When: ユーザーが前へボタンをクリック
- Then: on_page_changeコールバックが呼び出され、ページ番号2が渡される

### TC-010: 次へボタンのクリックイベント

- Given: current_page=3, total_pages=5, on_page_changeコールバックを設定
- When: ユーザーが次へボタンをクリック
- Then: on_page_changeコールバックが呼び出され、ページ番号4が渡される

### TC-011: Paginationのカスタムクラス

- Given: Paginationにclass="justify-center"を設定
- When: Paginationコンポーネントをレンダリング
- Then: ベースクラスに加えて"justify-center"が追加される

### TC-012: 単一ページの場合

- Given: current_page=1, total_pages=1
- When: Paginationコンポーネントをレンダリング
- Then: ページ番号1のボタンが表示され、前後ボタンは無効化される

