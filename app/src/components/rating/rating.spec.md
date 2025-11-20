# Rating Component Specification

## Related Files

- Implementation: `app/src/components/rating/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/rating/

## Requirements

### 責務

- レーティング表示
- DaisyUIの`rating`クラスを使用したスタイリング
- 評価値の表示と編集
- 星評価の表示

### Props構造

#### Ratingコンポーネント

- `value` (f64): 評価値（0-5、必須）
- `readonly` (optional, bool): 読み取り専用かどうか（デフォルト: false）
- `on_change` (optional, Callback<f64>): 評価値変更時のコールバック
- `class` (optional, String): 追加のCSSクラス

### スタイル

- `Rating`: `rating`クラスを適用
- 星アイコン: `mask mask-star-2`クラスを適用
- 半分の星: `rating-half`クラスを追加
- 非表示の星: `rating-hidden`クラスを追加

### 機能

- 5つの星で評価値を表示
- `value`に基づいて星を塗りつぶし
- 0.5刻みの評価値に対応（`rating-half`クラス）
- `readonly`が`true`の場合、クリックできない
- `readonly`が`false`の場合、星をクリックして評価値を変更できる

## Test Cases

### TC-001: Ratingの基本表示

- Given: value=3.0, readonly=false
- When: Ratingコンポーネントをレンダリング
- Then: `rating`クラスが適用され、3つの星が塗りつぶされる

### TC-002: 最大評価値の表示

- Given: value=5.0
- When: Ratingコンポーネントをレンダリング
- Then: 5つの星すべてが塗りつぶされる

### TC-003: 最小評価値の表示

- Given: value=0.0
- When: Ratingコンポーネントをレンダリング
- Then: 星が塗りつぶされない

### TC-004: 半分の星の表示

- Given: value=3.5
- When: Ratingコンポーネントをレンダリング
- Then: 3つの星が塗りつぶされ、4つ目の星が半分塗りつぶされる（`rating-half`クラス）

### TC-005: 読み取り専用モード

- Given: readonly=true, value=4.0
- When: Ratingコンポーネントをレンダリング
- Then: 星がクリックできず、5つ目の星が非表示になる

### TC-006: 評価値の変更

- Given: value=2.0, readonly=false, on_changeコールバックを設定
- When: ユーザーが4つ目の星をクリック
- Then: on_changeコールバックが呼び出され、値4.0が渡される

### TC-007: Ratingのカスタムクラス

- Given: Ratingにclass="rating-lg"を設定
- When: Ratingコンポーネントをレンダリング
- Then: ベースクラスに加えて"rating-lg"が追加される

### TC-008: 読み取り専用モードでのクリック無効化

- Given: readonly=true, on_changeコールバックを設定
- When: ユーザーが星をクリック
- Then: on_changeコールバックが呼び出されない

### TC-009: 複数のRatingの使用

- Given: ページ内に複数のRatingコンポーネントを配置
- When: Ratingコンポーネントをレンダリング
- Then: それぞれのRatingが正しく表示され、個別に動作する

