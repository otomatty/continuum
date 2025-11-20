# Loading Component Specification

## Related Files

- Implementation: `app/src/components/loading/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/loading/

## Requirements

### 責務

- ローディングスピナーの表示と管理
- DaisyUIの`loading`クラスを使用したスタイリング
- 非同期処理中の視覚的なフィードバック

### Props構造

- `variant` (optional, LoadingVariant): アニメーションタイプ（デフォルト: Spinner）
- `size` (optional, LoadingSize): サイズ（デフォルト: Md）
- `class` (optional, String): 追加のCSSクラス

### Variant

以下のバリアントをサポート：

- `Spinner`: スピナーアニメーション（デフォルト）
- `Dots`: ドットアニメーション
- `Ring`: リングアニメーション
- `Ball`: ボールアニメーション
- `Bars`: バーアニメーション
- `Infinity`: インフィニティアニメーション

### Size

以下のサイズをサポート：

- `Xs`: エクストラスモール
- `Sm`: スモール
- `Md`: ミディアム（デフォルト）
- `Lg`: ラージ

### スタイル

- `Loading`: `loading`クラス、バリアントクラス（`loading-spinner`, `loading-dots`, `loading-ring`, `loading-ball`, `loading-bars`, `loading-infinity`）、サイズクラス（`loading-xs`, `loading-sm`, `loading-md`, `loading-lg`）を適用

## Test Cases

### TC-001: Loadingの基本表示

- Given: variantとsizeが指定されていない
- When: Loadingコンポーネントをレンダリング
- Then: `loading loading-spinner loading-md`クラスが適用された`<span>`要素が表示される

### TC-002: Spinnerバリアントの適用

- Given: variant=LoadingVariant::Spinner
- When: Loadingコンポーネントをレンダリング
- Then: `loading-spinner`クラスが適用される

### TC-003: Dotsバリアントの適用

- Given: variant=LoadingVariant::Dots
- When: Loadingコンポーネントをレンダリング
- Then: `loading-dots`クラスが適用される

### TC-004: Ringバリアントの適用

- Given: variant=LoadingVariant::Ring
- When: Loadingコンポーネントをレンダリング
- Then: `loading-ring`クラスが適用される

### TC-005: Ballバリアントの適用

- Given: variant=LoadingVariant::Ball
- When: Loadingコンポーネントをレンダリング
- Then: `loading-ball`クラスが適用される

### TC-006: Barsバリアントの適用

- Given: variant=LoadingVariant::Bars
- When: Loadingコンポーネントをレンダリング
- Then: `loading-bars`クラスが適用される

### TC-007: Infinityバリアントの適用

- Given: variant=LoadingVariant::Infinity
- When: Loadingコンポーネントをレンダリング
- Then: `loading-infinity`クラスが適用される

### TC-008: Xsサイズの適用

- Given: size=LoadingSize::Xs
- When: Loadingコンポーネントをレンダリング
- Then: `loading-xs`クラスが適用される

### TC-009: Smサイズの適用

- Given: size=LoadingSize::Sm
- When: Loadingコンポーネントをレンダリング
- Then: `loading-sm`クラスが適用される

### TC-010: Mdサイズの適用

- Given: size=LoadingSize::Md
- When: Loadingコンポーネントをレンダリング
- Then: `loading-md`クラスが適用される

### TC-011: Lgサイズの適用

- Given: size=LoadingSize::Lg
- When: Loadingコンポーネントをレンダリング
- Then: `loading-lg`クラスが適用される

### TC-012: カスタムクラスの追加

- Given: class="text-primary"
- When: Loadingコンポーネントをレンダリング
- Then: ベースクラスに加えて"text-primary"が追加される

### TC-013: バリアントとサイズの組み合わせ

- Given: variant=LoadingVariant::Dots, size=LoadingSize::Lg
- When: Loadingコンポーネントをレンダリング
- Then: `loading loading-dots loading-lg`クラスが適用される

