# Accordion Component Specification

## Related Files

- Implementation: `app/src/components/accordion/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/collapse/

## Requirements

### 責務

- アコーディオンの表示と管理
- DaisyUIの`collapse`クラスを使用したスタイリング
- AccordionItem, AccordionHeader, AccordionContentコンポーネントとの連携
- 開閉状態の管理

### Props構造

#### Accordionコンポーネント

- `variant` (optional, AccordionVariant): スタイルバリエーション（デフォルト: Arrow）
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): アコーディオンの子要素（AccordionItemなど）

#### AccordionItemコンポーネント

- `open` (optional, ReadSignal<bool>): 開閉状態のSignal（外部から制御する場合）
- `set_open` (optional, WriteSignal<bool>): 開閉状態を設定するSignal（外部から制御する場合、openとセットで使用）
- `on_toggle` (optional, Callback<()>): トグルイベントのコールバック
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): アコーディオンアイテムの子要素（AccordionHeader, AccordionContent）

#### AccordionHeaderコンポーネント

- `variant` (optional, AccordionVariant): スタイルバリエーション（デフォルト: Arrow）
- `class` (optional, String): 追加のCSSクラス
- `on_click` (optional, Callback<MouseEvent>): クリックイベントのコールバック
- `children` (Children): ヘッダーのテキスト

#### AccordionContentコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): コンテンツの要素

### Variant

以下のバリアントをサポート：

- `Arrow`: 矢印アイコン（デフォルト）
- `Plus`: プラスアイコン

### スタイル

- `Accordion`: ラッパー要素（デフォルトクラスなし）
- `AccordionItem`: `collapse`クラスを適用、開いている時は`collapse-open`クラスを追加
- `AccordionHeader`: `collapse-title`クラスを適用、Plusバリアント時は`collapse-plus`クラスを追加
- `AccordionContent`: `collapse-content`クラスを適用

### 状態管理

- AccordionItemコンポーネント内で開閉状態を管理
- `open`プロパティが指定されている場合は外部から制御、指定されていない場合は内部で管理
- `create_signal`を使用して状態を管理

## Test Cases

### TC-001: Accordionの基本表示

- Given: Accordionコンポーネントをレンダリング
- When: ページを表示
- Then: ラッパー`<div>`要素が表示される

### TC-002: Arrowバリアントの適用

- Given: variant=AccordionVariant::Arrow
- When: AccordionHeaderコンポーネントをレンダリング
- Then: `collapse-title`クラスが適用される

### TC-003: Plusバリアントの適用

- Given: variant=AccordionVariant::Plus
- When: AccordionHeaderコンポーネントをレンダリング
- Then: `collapse-title collapse-plus`クラスが適用される

### TC-004: AccordionItemの閉じた状態

- Given: AccordionItemコンポーネントが閉じている
- When: AccordionItemコンポーネントをレンダリング
- Then: `collapse`クラスが適用される（`collapse-open`は含まれない）

### TC-005: AccordionItemの開いた状態

- Given: AccordionItemコンポーネントが開いている
- When: AccordionItemコンポーネントをレンダリング
- Then: `collapse collapse-open`クラスが適用される

### TC-006: AccordionHeaderのクリックイベント

- Given: AccordionHeaderコンポーネントにon_clickコールバックを設定
- When: ユーザーがヘッダーをクリック
- Then: on_clickコールバックが呼び出される

### TC-007: AccordionContentの表示

- Given: AccordionContentコンポーネントをレンダリング
- When: ページを表示
- Then: `collapse-content`クラスが適用された`<div>`要素が表示される

### TC-008: 外部から開閉状態を制御

- Given: AccordionItemにopen=ReadSignal<bool>を設定
- When: 外部からSignalの値を変更
- Then: AccordionItemの開閉状態が更新される

### TC-009: 内部で開閉状態を管理

- Given: AccordionItemにopenプロパティを設定しない
- When: ユーザーがヘッダーをクリック
- Then: AccordionItemの開閉状態が内部で切り替わる

### TC-010: Accordionのカスタムクラス

- Given: Accordionにclass="w-full"を設定
- When: Accordionコンポーネントをレンダリング
- Then: "w-full"クラスが追加される

### TC-011: Accordion構造の組み合わせ

- Given: Accordion内に複数のAccordionItem（各AccordionItemにAccordionHeaderとAccordionContentを含む）を配置
- When: Accordionコンポーネントをレンダリング
- Then: すべてのAccordionItemが正しく表示され、個別に開閉できる

