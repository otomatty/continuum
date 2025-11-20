# Skeleton Component Specification

## Related Files

- Implementation: `app/src/components/skeleton/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/skeleton/

## Requirements

### 責務

- スケルトンローディングの表示と管理
- DaisyUIの`skeleton`クラスを使用したスタイリング
- コンテンツ読み込み中のプレースホルダー表示

### Props構造

- `class` (optional, String): 追加のCSSクラス
- `children` (optional, Children): コンテンツ（通常は空）

### スタイル

- `Skeleton`: `skeleton`クラスを適用
- 幅や高さは`class`プロパティでカスタマイズ可能

## Test Cases

### TC-001: Skeletonの基本表示

- Given: classとchildrenが指定されていない
- When: Skeletonコンポーネントをレンダリング
- Then: `skeleton`クラスが適用された`<div>`要素が表示される

### TC-002: カスタムクラスの追加

- Given: class="h-4 w-32"
- When: Skeletonコンポーネントをレンダリング
- Then: ベースクラスに加えて"h-4 w-32"が追加される

### TC-003: 子要素の表示

- Given: childrenにテキストを設定
- When: Skeletonコンポーネントをレンダリング
- Then: 子要素が表示される

### TC-004: 子要素なしの表示

- Given: childrenが指定されていない
- When: Skeletonコンポーネントをレンダリング
- Then: 空の`<div>`要素が表示される

### TC-005: 幅と高さのカスタマイズ

- Given: class="h-8 w-full"
- When: Skeletonコンポーネントをレンダリング
- Then: `skeleton h-8 w-full`クラスが適用され、幅と高さが設定される

### TC-006: 角丸のカスタマイズ

- Given: class="rounded-lg"
- When: Skeletonコンポーネントをレンダリング
- Then: ベースクラスに加えて"rounded-lg"が追加される

