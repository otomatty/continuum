# Card Component Specification

## Related Files

- Implementation: `app/src/components/card.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/card/

## Requirements

### 責務

- カードコンテナの表示と管理
- DaisyUIの`card`クラスを使用したスタイリング
- CardTitleとCardBodyコンポーネントとの連携

### Props構造

#### Cardコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): カードの子要素（CardTitle, CardBodyなど）

#### CardTitleコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): タイトルのテキスト

#### CardBodyコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): カード本文のコンテンツ

### スタイル

- `Card`: `card`クラスを適用
- `CardTitle`: `text-2xl font-bold mb-4`クラスを適用（デフォルト）
- `CardBody`: `card-body`クラスを適用

## Test Cases

### TC-001: Cardの基本表示

- Given: Cardコンポーネントをレンダリング
- When: ページを表示
- Then: `card`クラスが適用された`<div>`要素が表示される

### TC-002: CardTitleの表示

- Given: CardTitleコンポーネントをレンダリング
- When: ページを表示
- Then: `text-2xl font-bold mb-4`クラスが適用された`<h2>`要素が表示される

### TC-003: CardBodyの表示

- Given: CardBodyコンポーネントをレンダリング
- When: ページを表示
- Then: `card-body`クラスが適用された`<div>`要素が表示される

### TC-004: CardTitleのカスタムクラス

- Given: CardTitleにclass="text-3xl"を設定
- When: CardTitleコンポーネントをレンダリング
- Then: デフォルトクラスに加えて"text-3xl"が追加される

### TC-005: CardBodyのカスタムクラス

- Given: CardBodyにclass="p-6"を設定
- When: CardBodyコンポーネントをレンダリング
- Then: ベースクラスに加えて"p-6"が追加される

### TC-006: Cardのカスタムクラス

- Given: Cardにclass="bg-base-200"を設定
- When: Cardコンポーネントをレンダリング
- Then: ベースクラスに加えて"bg-base-200"が追加される

### TC-007: Card構造の組み合わせ

- Given: Card内にCardTitleとCardBodyを配置
- When: Cardコンポーネントをレンダリング
- Then: CardTitleとCardBodyが正しく表示される

