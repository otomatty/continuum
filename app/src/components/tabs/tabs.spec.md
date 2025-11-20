# Tabs Component Specification

## Related Files

- Implementation: `app/src/components/tabs/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/tabs/

## Requirements

### 責務

- タブの表示と管理
- DaisyUIの`tabs`クラスを使用したスタイリング
- Tab, TabList, TabPanelコンポーネントとの連携
- アクティブなタブの状態管理

### Props構造

#### Tabsコンポーネント

- `variant` (optional, TabsVariant): スタイルバリエーション（デフォルト: Bordered）
- `default_active` (optional, usize): デフォルトでアクティブなタブのインデックス（デフォルト: 0）
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): タブの子要素（TabList, TabPanelなど）

#### TabListコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): タブのリスト（Tabコンポーネント）

#### Tabコンポーネント

- `index` (usize): タブのインデックス（必須）
- `class` (optional, String): 追加のCSSクラス
- `on_click` (optional, Callback<MouseEvent>): クリックイベントのコールバック
- `children` (Children): タブのラベルテキスト

#### TabPanelコンポーネント

- `index` (usize): パネルのインデックス（必須）
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): パネルのコンテンツ

### Variant

以下のバリアントをサポート：

- `Bordered`: ボーダー付きタブ（デフォルト）
- `Lifted`: リフトされたタブ

### スタイル

- `Tabs`: `tabs`クラスとバリアントクラス（`tabs-bordered`, `tabs-lifted`）を適用
- `Tab`: `tab`クラスを適用、アクティブ時は`tab-active`クラスを追加
- `TabList`: ラッパー要素（デフォルトクラスなし）
- `TabPanel`: パネル要素（デフォルトクラスなし、hidden属性で表示/非表示を制御）

### 状態管理

- Tabsコンポーネント内でアクティブなタブのインデックスを管理
- `create_signal`を使用して状態を管理
- `provide_context`で子コンポーネントに状態を提供

## Test Cases

### TC-001: Tabsの基本表示

- Given: variantが指定されていない
- When: Tabsコンポーネントをレンダリング
- Then: `tabs tabs-bordered`クラスが適用された`<div>`要素が表示される

### TC-002: Borderedバリアントの適用

- Given: variant=TabsVariant::Bordered
- When: Tabsコンポーネントをレンダリング
- Then: `tabs tabs-bordered`クラスが適用される

### TC-003: Liftedバリアントの適用

- Given: variant=TabsVariant::Lifted
- When: Tabsコンポーネントをレンダリング
- Then: `tabs tabs-lifted`クラスが適用される

### TC-004: デフォルトアクティブタブの設定

- Given: default_active=1
- When: Tabsコンポーネントをレンダリング
- Then: インデックス1のタブがアクティブになる

### TC-005: Tabのアクティブ状態

- Given: Tabコンポーネント（index=0）がアクティブ
- When: Tabコンポーネントをレンダリング
- Then: `tab tab-active`クラスが適用される

### TC-006: Tabの非アクティブ状態

- Given: Tabコンポーネント（index=1）が非アクティブ（アクティブはindex=0）
- When: Tabコンポーネントをレンダリング
- Then: `tab`クラスが適用される（`tab-active`は含まれない）

### TC-007: Tabのクリックイベント

- Given: Tabコンポーネントにon_clickコールバックを設定
- When: ユーザーがタブをクリック
- Then: on_clickコールバックが呼び出され、そのタブがアクティブになる

### TC-008: TabPanelの表示/非表示

- Given: TabPanelコンポーネント（index=0）がアクティブ
- When: TabPanelコンポーネントをレンダリング
- Then: `hidden`属性がfalseになる

### TC-009: TabPanelの非表示

- Given: TabPanelコンポーネント（index=1）が非アクティブ（アクティブはindex=0）
- When: TabPanelコンポーネントをレンダリング
- Then: `hidden`属性がtrueになる

### TC-010: Tabsのカスタムクラス

- Given: Tabsにclass="w-full"を設定
- When: Tabsコンポーネントをレンダリング
- Then: ベースクラスに加えて"w-full"が追加される

### TC-011: Tabs構造の組み合わせ

- Given: Tabs内にTabList（複数のTabを含む）と複数のTabPanelを配置
- When: Tabsコンポーネントをレンダリング
- Then: TabListとTabPanelが正しく表示され、アクティブなタブとパネルが連動する

