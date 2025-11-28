# Container Component Specification

## Related Files

- Implementation: `app/src/components/container/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- Tailwind CSS Container: https://tailwindcss.com/docs/container

## Requirements

### 責務

- セクション内で使用されるコンテナコンポーネント
- 最大幅や余白を一定に保つ役割を担う
- レスポンシブデザインに対応したレイアウト制御
- コンテンツの中央揃えとパディングの管理

### Props構造

#### Containerコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): コンテナの子要素

### スタイル

- `Container`: デフォルトで以下のクラスを適用
  - `max-w-7xl`: 最大幅を設定（1280px）
  - `mx-auto`: 中央揃え
  - `px-4`: 左右のパディング（モバイル）
  - `md:px-6`: 左右のパディング（タブレット以上）
  - `lg:px-8`: 左右のパディング（デスクトップ以上）

### 使用例

```rust
view! {
    <section class="py-16">
        <Container>
            <h1>"タイトル"</h1>
            <p>"コンテンツ"</p>
        </Container>
    </section>
}
```

## Test Cases

### TC-001: Containerの基本表示

- Given: Containerコンポーネントをレンダリング
- When: ページを表示
- Then: デフォルトのクラス（max-w-7xl, mx-auto, px-4, md:px-6, lg:px-8）が適用された`<div>`要素が表示される

### TC-002: Containerのカスタムクラス

- Given: Containerにclass="max-w-5xl"を設定
- When: Containerコンポーネントをレンダリング
- Then: デフォルトクラスに加えて"max-w-5xl"が追加される

### TC-003: Containerの子要素表示

- Given: Container内に子要素を配置
- When: Containerコンポーネントをレンダリング
- Then: 子要素が正しく表示される

### TC-004: Containerのレスポンシブ対応

- Given: Containerコンポーネントをレンダリング
- When: 画面サイズを変更（モバイル→タブレット→デスクトップ）
- Then: 適切なパディングが適用される（px-4 → md:px-6 → lg:px-8）

### TC-005: Containerの中央揃え

- Given: Containerコンポーネントをレンダリング
- When: ページを表示
- Then: mx-autoクラスによりコンテンツが中央に配置される

### TC-006: Containerの最大幅制限

- Given: Containerコンポーネントをレンダリング
- When: 画面幅が1280pxを超える
- Then: max-w-7xlによりコンテンツの幅が1280pxに制限される

