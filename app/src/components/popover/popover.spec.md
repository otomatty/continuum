# Popover Component Specification

## Related Files

- Implementation: `app/src/components/popover/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/popover/

## Requirements

### 責務

- ポップオーバーの表示と管理
- DaisyUIの`popover`クラスを使用したスタイリング
- クリック時の情報表示
- 開閉状態の管理

### Props構造

#### Popoverコンポーネント

- `open` (ReadSignal<bool>): 開閉状態のSignal（必須）
- `content` (String): ポップオーバーのコンテンツ（必須）
- `on_toggle` (optional, Callback<()>): トグルイベントのコールバック
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ポップオーバーをトリガーする要素

### スタイル

- `Popover`: `popover`クラスを適用、開いている時は`popover-open`クラスを追加
- トリガー要素: `popover-trigger`クラスを適用
- コンテンツ要素: `popover-content`クラスを適用

### 状態管理

- Popoverコンポーネントは外部から`ReadSignal<bool>`で開閉状態を受け取る
- `open`が`true`の時、`popover-open`クラスが適用され、ポップオーバーが表示される
- トリガー要素をクリックすると`on_toggle`コールバックが呼び出される

## Test Cases

### TC-001: Popoverの閉じた状態

- Given: open=ReadSignal<bool>でfalse
- When: Popoverコンポーネントをレンダリング
- Then: `popover`クラスが適用される（`popover-open`は含まれない）

### TC-002: Popoverの開いた状態

- Given: open=ReadSignal<bool>でtrue
- When: Popoverコンポーネントをレンダリング
- Then: `popover popover-open`クラスが適用される

### TC-003: Popoverのコンテンツ表示

- Given: content="これはポップオーバーです"
- When: Popoverコンポーネントをレンダリング
- Then: `popover-content`クラスが適用された要素に"これはポップオーバーです"が表示される

### TC-004: トリガー要素のクリックイベント

- Given: Popoverにon_toggleコールバックを設定
- When: ユーザーがトリガー要素をクリック
- Then: on_toggleコールバックが呼び出される

### TC-005: Popoverのカスタムクラス

- Given: Popoverにclass="popover-bottom"を設定
- When: Popoverコンポーネントをレンダリング
- Then: ベースクラスに加えて"popover-bottom"が追加される

### TC-006: Popoverの子要素表示

- Given: Popover内にボタン要素を配置
- When: Popoverコンポーネントをレンダリング
- Then: ボタン要素が`popover-trigger`クラスでラップされ、クリック時にポップオーバーが表示される

### TC-007: 開閉状態の変更

- Given: open=ReadSignal<bool>でfalseからtrueに変更
- When: Signalの値が更新される
- Then: Popoverに`popover-open`クラスが追加され、ポップオーバーが表示される

### TC-008: 複数のPopoverの使用

- Given: ページ内に複数のPopoverコンポーネントを配置
- When: Popoverコンポーネントをレンダリング
- Then: それぞれのPopoverが正しく表示され、個別に動作する

