# Accordion Component Specification

## Related Files

- Implementation: `app/src/components/accordion/mod.rs`
- Tests: `app/src/components/accordion/tests.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/collapse/

## Requirements

### 責務

- DaisyUI標準に準拠したアコーディオンの表示と管理
- DaisyUIの`collapse`クラスと`<input type="checkbox">`を使用したスタイリング
- AccordionItem, AccordionTitle, AccordionContentコンポーネントとの連携
- checkboxのchecked状態による開閉状態の管理

### Props構造

#### Accordionコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): アコーディオンの子要素（AccordionItemなど）

#### AccordionItemコンポーネント

- `variant` (optional, AccordionVariant): スタイルバリエーション（デフォルト: None）
- `open` (optional, ReadSignal<bool>): 開閉状態のSignal（外部から制御する場合）
- `set_open` (optional, WriteSignal<bool>): 開閉状態を設定するSignal（外部から制御する場合、openとセットで使用）
- `on_toggle` (optional, Callback<bool>): トグルイベントのコールバック（引数: 新しい開閉状態）
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): アコーディオンアイテムの子要素（AccordionTitle, AccordionContent）

#### AccordionTitleコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ヘッダーのテキスト

#### AccordionContentコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): コンテンツの要素

### Variant

以下のバリアントをサポート：

- `None`: アイコンなし（デフォルト）
- `Arrow`: 矢印アイコン（`collapse-arrow`クラス）
- `Plus`: プラスアイコン（`collapse-plus`クラス）

### スタイル（DaisyUI標準）

- `Accordion`: ラッパー要素（デフォルトクラスなし）
- `AccordionItem`: 
  - 基本: `collapse`クラスを適用
  - variant指定時: `collapse collapse-arrow`または`collapse collapse-plus`クラスを適用
  - DaisyUI標準では`<input type="checkbox">`のchecked状態で開閉を制御するため、`collapse-open`クラスは不要
- `AccordionTitle`: `collapse-title`クラスを適用
- `AccordionContent`: `collapse-content`クラスを適用

### 状態管理（DaisyUI標準）

- DaisyUI標準に従い、`<input type="checkbox">`を使用して開閉状態を管理
- checkboxの`checked`属性をSignalでバインド
- `open`プロパティが指定されている場合は外部から制御、指定されていない場合は内部で管理
- `signal`を使用して状態を管理

## Test Cases

### TC-001: Accordionの基本表示

- Given: Accordionコンポーネントをレンダリング
- When: ページを表示
- Then: ラッパー`<div>`要素が表示される

### TC-002: Arrowバリアントの適用

- Given: variant=AccordionVariant::Arrow
- When: AccordionItemコンポーネントをレンダリング
- Then: `collapse collapse-arrow`クラスが適用される

### TC-003: Plusバリアントの適用

- Given: variant=AccordionVariant::Plus
- When: AccordionItemコンポーネントをレンダリング
- Then: `collapse collapse-plus`クラスが適用される

### TC-004: Noneバリアント（デフォルト）の適用

- Given: variant=AccordionVariant::None（またはvariantを指定しない）
- When: AccordionItemコンポーネントをレンダリング
- Then: `collapse`クラスのみが適用される（アイコンクラスは含まれない）

### TC-005: AccordionItemのチェックボックス

- Given: AccordionItemコンポーネントをレンダリング
- When: ページを表示
- Then: `<input type="checkbox">`要素が含まれる

### TC-006: チェックボックスの状態とSignalの同期

- Given: AccordionItemにopen=ReadSignal<bool>を設定
- When: checkboxをクリック
- Then: Signalの値が更新される

### TC-007: AccordionTitleの表示

- Given: AccordionTitleコンポーネントをレンダリング
- When: ページを表示
- Then: `collapse-title`クラスが適用された`<div>`要素が表示される

### TC-008: AccordionContentの表示

- Given: AccordionContentコンポーネントをレンダリング
- When: ページを表示
- Then: `collapse-content`クラスが適用された`<div>`要素が表示される

### TC-009: 外部から開閉状態を制御

- Given: AccordionItemにopen=ReadSignal<bool>とset_open=WriteSignal<bool>を設定
- When: 外部からSignalの値を変更
- Then: AccordionItemの開閉状態（checkboxのchecked状態）が更新される

### TC-010: 内部で開閉状態を管理

- Given: AccordionItemにopenプロパティを設定しない
- When: checkboxをクリック
- Then: AccordionItemの開閉状態が内部で切り替わる

### TC-011: on_toggleコールバック

- Given: AccordionItemにon_toggle=Callback<bool>を設定
- When: checkboxをクリック
- Then: on_toggleコールバックが新しい開閉状態（bool）を引数として呼び出される

### TC-012: Accordionのカスタムクラス

- Given: Accordionにclass="w-full"を設定
- When: Accordionコンポーネントをレンダリング
- Then: "w-full"クラスが追加される

### TC-013: AccordionItemのカスタムクラス

- Given: AccordionItemにclass="bg-base-200"を設定
- When: AccordionItemコンポーネントをレンダリング
- Then: `collapse bg-base-200`クラス（またはvariant指定時は`collapse collapse-arrow bg-base-200`など）が適用される

### TC-014: Accordion構造の組み合わせ

- Given: Accordion内に複数のAccordionItem（各AccordionItemにAccordionTitleとAccordionContentを含む）を配置
- When: Accordionコンポーネントをレンダリング
- Then: すべてのAccordionItemが正しく表示され、個別に開閉できる

