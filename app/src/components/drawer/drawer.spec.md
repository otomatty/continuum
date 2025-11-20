# Drawer Component Specification

## Related Files

- Implementation: `app/src/components/drawer/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/drawer/

## Requirements

### 責務

- サイドドロワーの表示と管理
- DaisyUIの`drawer`クラスを使用したスタイリング
- DrawerSide, DrawerContent, DrawerToggleコンポーネントとの連携
- 開閉状態の管理

### Props構造

#### Drawerコンポーネント

- `open` (ReadSignal<bool>): 開閉状態のSignal（必須）
- `side` (optional, DrawerSide): ドロワーの位置（デフォルト: Left）
- `on_close` (optional, Callback<()>): 閉じるイベントのコールバック
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ドロワーの子要素（DrawerSide, DrawerContent）

#### DrawerSideコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): サイドメニューのコンテンツ

#### DrawerContentコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): メインコンテンツ

#### DrawerToggleコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `on_click` (optional, Callback<MouseEvent>): クリックイベントのコールバック
- `children` (Children): トグルボタンのテキスト

### DrawerSide

以下の位置をサポート：

- `Left`: 左側（デフォルト）
- `Right`: 右側

### スタイル

- `Drawer`: `drawer`クラスを適用、開いている時は`drawer-open`クラスを追加
- `DrawerSide`: `drawer-side`クラスを適用、Rightの場合は`drawer-end`クラスを追加、内部に`drawer-overlay`と`menu`を含む
- `DrawerContent`: `drawer-content`クラスを適用
- `DrawerToggle`: `btn btn-primary drawer-button`クラスを適用した`<label>`要素

### 状態管理

- Drawerコンポーネントは外部から`ReadSignal<bool>`で開閉状態を受け取る
- `open`が`true`の時、`drawer-open`クラスが適用され、ドロワーが表示される
- `provide_context`で子コンポーネントに状態を提供
- バックドロップをクリックすると`on_close`コールバックが呼び出される

## Test Cases

### TC-001: Drawerの閉じた状態

- Given: open=ReadSignal<bool>でfalse
- When: Drawerコンポーネントをレンダリング
- Then: `drawer`クラスが適用される（`drawer-open`は含まれない）

### TC-002: Drawerの開いた状態

- Given: open=ReadSignal<bool>でtrue
- When: Drawerコンポーネントをレンダリング
- Then: `drawer drawer-open`クラスが適用される

### TC-003: Leftサイドの適用

- Given: side=DrawerSide::Left
- When: DrawerSideコンポーネントをレンダリング
- Then: `drawer-side`クラスが適用される（`drawer-end`は含まれない）

### TC-004: Rightサイドの適用

- Given: side=DrawerSide::Right
- When: DrawerSideコンポーネントをレンダリング
- Then: `drawer-side drawer-end`クラスが適用される

### TC-005: DrawerSideの表示

- Given: DrawerSideコンポーネントをレンダリング
- When: ページを表示
- Then: `drawer-side`クラスが適用された`<div>`要素が表示され、内部に`drawer-overlay`と`menu`が含まれる

### TC-006: DrawerContentの表示

- Given: DrawerContentコンポーネントをレンダリング
- When: ページを表示
- Then: `drawer-content`クラスが適用された`<div>`要素が表示される

### TC-007: DrawerToggleの表示

- Given: DrawerToggleコンポーネントをレンダリング
- When: ページを表示
- Then: `btn btn-primary drawer-button`クラスが適用された`<label>`要素が表示される

### TC-008: バックドロップのクリックイベント

- Given: Drawerにon_closeコールバックを設定
- When: ユーザーがバックドロップをクリック
- Then: on_closeコールバックが呼び出される

### TC-009: DrawerToggleのクリックイベント

- Given: DrawerToggleにon_clickコールバックを設定
- When: ユーザーがトグルボタンをクリック
- Then: on_clickコールバックが呼び出され、on_closeコールバックも呼び出される

### TC-010: Drawerのカスタムクラス

- Given: Drawerにclass="drawer-mobile"を設定
- When: Drawerコンポーネントをレンダリング
- Then: ベースクラスに加えて"drawer-mobile"が追加される

### TC-011: Drawer構造の組み合わせ

- Given: Drawer内にDrawerSideとDrawerContent（DrawerToggleを含む）を配置
- When: Drawerコンポーネントをレンダリング
- Then: すべてのコンポーネントが正しく表示される

### TC-012: 開閉状態の変更

- Given: open=ReadSignal<bool>でfalseからtrueに変更
- When: Signalの値が更新される
- Then: Drawerに`drawer-open`クラスが追加され、ドロワーが表示される

