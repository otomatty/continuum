# Dropdown Component Specification

## Related Files

- Implementation: `app/src/components/dropdown/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/dropdown/

## Requirements

### 責務

- ドロップダウンメニューの表示と管理
- DaisyUIの`dropdown`クラスを使用したスタイリング
- DropdownButton, DropdownMenu, DropdownItemコンポーネントとの連携
- 開閉状態の管理

### Props構造

#### Dropdownコンポーネント

- `open` (ReadSignal<bool>): 開閉状態のSignal（必須）
- `variant` (optional, DropdownVariant): スタイルバリエーション（デフォルト: Click）
- `on_toggle` (optional, Callback<()>): トグルイベントのコールバック
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ドロップダウンの子要素（DropdownButton, DropdownMenu）

#### DropdownButtonコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `on_click` (optional, Callback<MouseEvent>): クリックイベントのコールバック
- `children` (Children): ボタンのテキスト

#### DropdownMenuコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): メニューの子要素（DropdownItem）

#### DropdownItemコンポーネント

- `href` (optional, String): リンク先URL
- `class` (optional, String): 追加のCSSクラス
- `on_click` (optional, Callback<MouseEvent>): クリックイベントのコールバック
- `children` (Children): アイテムのテキスト

### Variant

以下のバリアントをサポート：

- `Click`: クリックで開閉（デフォルト）
- `Hover`: ホバーで開閉

### スタイル

- `Dropdown`: `dropdown`クラスを適用、Hoverバリアント時は`dropdown-hover`クラスを追加、開いている時は`dropdown-open`クラスを追加
- `DropdownButton`: `btn`クラスを適用した`<label>`要素
- `DropdownMenu`: `dropdown-content menu bg-base-200 rounded-box z-[1] w-52 p-2 shadow`クラスを適用した`<ul>`要素
- `DropdownItem`: `<li>`要素、内部に`<a>`要素

### 状態管理

- Dropdownコンポーネントは外部から`ReadSignal<bool>`で開閉状態を受け取る
- `open`が`true`の時、`dropdown-open`クラスが適用され、メニューが表示される
- `provide_context`で子コンポーネントに状態を提供

## Test Cases

### TC-001: Dropdownの閉じた状態

- Given: open=ReadSignal<bool>でfalse
- When: Dropdownコンポーネントをレンダリング
- Then: `dropdown`クラスが適用される（`dropdown-open`は含まれない）

### TC-002: Dropdownの開いた状態

- Given: open=ReadSignal<bool>でtrue
- When: Dropdownコンポーネントをレンダリング
- Then: `dropdown dropdown-open`クラスが適用される

### TC-003: Clickバリアントの適用

- Given: variant=DropdownVariant::Click
- When: Dropdownコンポーネントをレンダリング
- Then: `dropdown`クラスが適用される

### TC-004: Hoverバリアントの適用

- Given: variant=DropdownVariant::Hover
- When: Dropdownコンポーネントをレンダリング
- Then: `dropdown dropdown-hover`クラスが適用される

### TC-005: DropdownButtonの表示

- Given: DropdownButtonコンポーネントをレンダリング
- When: ページを表示
- Then: `btn`クラスが適用された`<label>`要素が表示される

### TC-006: DropdownButtonのクリックイベント

- Given: Dropdownにon_toggleコールバックを設定
- When: ユーザーがDropdownButtonをクリック
- Then: on_toggleコールバックが呼び出される

### TC-007: DropdownMenuの表示

- Given: DropdownMenuコンポーネントをレンダリング
- When: ページを表示
- Then: `dropdown-content menu bg-base-200 rounded-box z-[1] w-52 p-2 shadow`クラスが適用された`<ul>`要素が表示される

### TC-008: DropdownItemのリンク付き表示

- Given: DropdownItemにhref="https://example.com"を設定
- When: DropdownItemコンポーネントをレンダリング
- Then: `<li>`要素内に`<a>`要素が表示され、href属性が設定される

### TC-009: DropdownItemのクリックイベント

- Given: DropdownItemにon_clickコールバックを設定
- When: ユーザーがアイテムをクリック
- Then: on_clickコールバックが呼び出される

### TC-010: Dropdownのカスタムクラス

- Given: Dropdownにclass="dropdown-end"を設定
- When: Dropdownコンポーネントをレンダリング
- Then: ベースクラスに加えて"dropdown-end"が追加される

### TC-011: Dropdown構造の組み合わせ

- Given: Dropdown内にDropdownButtonとDropdownMenu（複数のDropdownItemを含む）を配置
- When: Dropdownコンポーネントをレンダリング
- Then: すべてのコンポーネントが正しく表示される

### TC-012: 開閉状態の変更

- Given: open=ReadSignal<bool>でfalseからtrueに変更
- When: Signalの値が更新される
- Then: Dropdownに`dropdown-open`クラスが追加され、メニューが表示される

