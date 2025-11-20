# Breadcrumbs Component Specification

## Related Files

- Implementation: `app/src/components/breadcrumbs/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/breadcrumbs/

## Requirements

### 責務

- パンくずリストの表示
- DaisyUIの`breadcrumbs`クラスを使用したスタイリング
- BreadcrumbItemコンポーネントとの連携
- ナビゲーションパスの表示

### Props構造

#### Breadcrumbsコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): パンくずリストの子要素（BreadcrumbItem）

#### BreadcrumbItemコンポーネント

- `href` (optional, String): リンク先URL（指定されない場合は`<span>`要素）
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): パンくずアイテムのテキスト

### スタイル

- `Breadcrumbs`: `breadcrumbs`クラスを適用した`<nav>`要素、内部に`<ul>`要素を含む
- `BreadcrumbItem`: `<li>`要素、`href`が指定されている場合は`<a>`要素、指定されていない場合は`<span>`要素

## Test Cases

### TC-001: Breadcrumbsの基本表示

- Given: Breadcrumbsコンポーネントをレンダリング
- When: ページを表示
- Then: `breadcrumbs`クラスが適用された`<nav>`要素が表示され、内部に`<ul>`要素が含まれる

### TC-002: BreadcrumbItemのリンク付き表示

- Given: BreadcrumbItemにhref="https://example.com"を設定
- When: BreadcrumbItemコンポーネントをレンダリング
- Then: `<li>`要素内に`<a>`要素が表示され、href属性が設定される

### TC-003: BreadcrumbItemのリンクなし表示

- Given: BreadcrumbItemにhrefを設定しない
- When: BreadcrumbItemコンポーネントをレンダリング
- Then: `<li>`要素内に`<span>`要素が表示される

### TC-004: Breadcrumbsのカスタムクラス

- Given: Breadcrumbsにclass="text-sm"を設定
- When: Breadcrumbsコンポーネントをレンダリング
- Then: ベースクラスに加えて"text-sm"が追加される

### TC-005: BreadcrumbItemのカスタムクラス

- Given: BreadcrumbItemにclass="font-bold"を設定
- When: BreadcrumbItemコンポーネントをレンダリング
- Then: `<li>`要素に"font-bold"クラスが追加される

### TC-006: Breadcrumbs構造の組み合わせ

- Given: Breadcrumbs内に複数のBreadcrumbItemを配置
- When: Breadcrumbsコンポーネントをレンダリング
- Then: すべてのBreadcrumbItemが正しく表示される

### TC-007: パンくずリストの階層表示

- Given: Breadcrumbs内に複数のBreadcrumbItem（一部はhref付き、一部はhrefなし）を配置
- When: Breadcrumbsコンポーネントをレンダリング
- Then: 階層的なパンくずリストが正しく表示される

