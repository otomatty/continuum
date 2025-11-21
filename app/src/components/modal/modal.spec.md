# Modal Component Specification

## Related Files

- Implementation: `app/src/components/modal/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/modal/

## Requirements

### 責務

- モーダルダイアログの表示と管理
- DaisyUIの`modal`クラスを使用したスタイリング
- ModalBox, ModalHeader, ModalBody, ModalFooter, ModalActionコンポーネントとの連携
- 開閉状態の管理

### Props構造

#### Modalコンポーネント

- `open` (ReadSignal<bool>): 開閉状態のSignal（必須）
- `on_close` (optional, Callback<()>): 閉じるイベントのコールバック
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): モーダルの子要素（ModalBoxなど）

#### ModalBoxコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): モーダルボックスの子要素（ModalHeader, ModalBody, ModalFooter）

#### ModalHeaderコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ヘッダーのテキスト

#### ModalBodyコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ボディのコンテンツ

#### ModalFooterコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): フッターのコンテンツ（ModalActionなど）

#### ModalActionコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `on_click` (optional, Callback<MouseEvent>): クリックイベントのコールバック
- `children` (Children): アクションボタンのテキスト

### スタイル

- `Modal`: `modal`クラスを適用、開いている時は`modal-open`クラスを追加、内部に`modal-backdrop`を含む
- `ModalBox`: `modal-box`クラスを適用
- `ModalHeader`: `font-bold text-lg mb-4`クラスを適用（デフォルト）
- `ModalBody`: `py-4`クラスを適用（デフォルト）
- `ModalFooter`: `modal-action`クラスを適用
- `ModalAction`: `btn`クラスを適用

### 状態管理

- Modalコンポーネントは外部から`ReadSignal<bool>`で開閉状態を受け取る
- `open`が`true`の時、`modal-open`クラスが適用され、モーダルが表示される
- バックドロップをクリックすると`on_close`コールバックが呼び出される

## Test Cases

### TC-001: Modalの閉じた状態

- Given: open=ReadSignal<bool>でfalse
- When: Modalコンポーネントをレンダリング
- Then: `modal`クラスが適用される（`modal-open`は含まれない）

### TC-002: Modalの開いた状態

- Given: open=ReadSignal<bool>でtrue
- When: Modalコンポーネントをレンダリング
- Then: `modal modal-open`クラスが適用される

### TC-003: ModalBoxの表示

- Given: ModalBoxコンポーネントをレンダリング
- When: ページを表示
- Then: `modal-box`クラスが適用された`<div>`要素が表示される

### TC-004: ModalHeaderの表示

- Given: ModalHeaderコンポーネントをレンダリング
- When: ページを表示
- Then: `font-bold text-lg mb-4`クラスが適用された`<h3>`要素が表示される

### TC-005: ModalBodyの表示

- Given: ModalBodyコンポーネントをレンダリング
- When: ページを表示
- Then: `py-4`クラスが適用された`<div>`要素が表示される

### TC-006: ModalFooterの表示

- Given: ModalFooterコンポーネントをレンダリング
- When: ページを表示
- Then: `modal-action`クラスが適用された`<div>`要素が表示される

### TC-007: ModalActionの表示

- Given: ModalActionコンポーネントをレンダリング
- When: ページを表示
- Then: `btn`クラスが適用された`<button>`要素が表示される

### TC-008: バックドロップのクリックイベント

- Given: Modalにon_closeコールバックを設定
- When: ユーザーがバックドロップをクリック
- Then: on_closeコールバックが呼び出される

### TC-009: ModalActionのクリックイベント

- Given: ModalActionにon_clickコールバックを設定
- When: ユーザーがアクションボタンをクリック
- Then: on_clickコールバックが呼び出される

### TC-010: Modalのカスタムクラス

- Given: Modalにclass="modal-bottom"を設定
- When: Modalコンポーネントをレンダリング
- Then: ベースクラスに加えて"modal-bottom"が追加される

### TC-011: Modal構造の組み合わせ

- Given: Modal内にModalBox（ModalHeader, ModalBody, ModalFooterを含む、ModalFooter内に複数のModalAction）を配置
- When: Modalコンポーネントをレンダリング
- Then: すべてのコンポーネントが正しく表示される

### TC-012: 開閉状態の変更

- Given: open=ReadSignal<bool>でfalseからtrueに変更
- When: Signalの値が更新される
- Then: Modalに`modal-open`クラスが追加され、モーダルが表示される

