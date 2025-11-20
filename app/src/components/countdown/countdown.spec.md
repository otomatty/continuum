# Countdown Component Specification

## Related Files

- Implementation: `app/src/components/countdown/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/countdown/

## Requirements

### 責務

- カウントダウンタイマーの表示
- DaisyUIの`countdown`クラスを使用したスタイリング
- イベントまでの残り時間表示
- タイマー機能の実装

### Props構造

#### Countdownコンポーネント

- `target_date` (i64): ターゲット日時（Unixタイムスタンプ、秒単位、必須）
- `on_complete` (optional, Callback<()>): カウントダウン完了時のコールバック
- `class` (optional, String): 追加のCSSクラス

### スタイル

- `Countdown`: `countdown font-mono text-2xl`クラスを適用（デフォルト）
- 各時間単位（時、分、秒）を別々の`span`要素で表示
- 各`span`要素に`--value` CSS変数を設定（0-999の範囲）
- 分と秒には`--digits: 2`を設定して2桁表示を保証
- アクセシビリティ属性（`aria-live="polite"`, `aria-label`）を設定

### 機能

- 1秒ごとにカウントダウンを更新
- 残り時間を時:分:秒形式で表示（例: "10h 24m 59s"）
- 各時間単位をdaisyUIのカウントダウンアニメーションで表示
- カウントダウンが0になったら`on_complete`コールバックを呼び出す
- `set_interval`を使用してタイマーを実装

## Test Cases

### TC-001: Countdownの基本表示

- Given: target_date=未来のUnixタイムスタンプ
- When: Countdownコンポーネントをレンダリング
- Then: `countdown`クラスが適用され、残り時間が表示される

### TC-002: 残り時間の表示形式

- Given: target_date=現在時刻+3600秒（1時間後）
- When: Countdownコンポーネントをレンダリング
- Then: "1h 00m 00s"形式で残り時間が表示される

### TC-003: カウントダウンの更新

- Given: target_date=現在時刻+5秒
- When: Countdownコンポーネントをレンダリング
- Then: 1秒ごとに残り時間が更新される

### TC-004: カウントダウン完了時のコールバック

- Given: target_date=現在時刻+2秒, on_completeコールバックを設定
- When: カウントダウンが0になる
- Then: on_completeコールバックが呼び出される

### TC-005: カウントダウン完了時の表示

- Given: target_date=過去のUnixタイムスタンプ
- When: Countdownコンポーネントをレンダリング
- Then: "0h 00m 00s"が表示される

### TC-006: Countdownのカスタムクラス

- Given: Countdownにclass="text-4xl"を設定
- When: Countdownコンポーネントをレンダリング
- Then: デフォルトクラス`countdown font-mono text-2xl`に加えて"text-4xl"が追加される

### TC-007: CSS変数の設定

- Given: target_date=現在時刻+3661秒（1時間1分1秒）
- When: Countdownコンポーネントをレンダリング
- Then: 各`span`要素に適切な`--value` CSS変数が設定される（時=1, 分=1, 秒=1）

### TC-008: タイマーのクリーンアップ

- Given: Countdownコンポーネントがアンマウントされる
- When: コンポーネントが削除される
- Then: タイマーがクリーンアップされる

### TC-009: アクセシビリティ属性の設定

- Given: target_date=現在時刻+3661秒（1時間1分1秒）
- When: Countdownコンポーネントをレンダリング
- Then: 各`span`要素に`aria-live="polite"`と適切な`aria-label`が設定される

### TC-010: 2桁表示の保証

- Given: target_date=現在時刻+3665秒（1時間1分5秒）
- When: Countdownコンポーネントをレンダリング
- Then: 時は"1"、分と秒は"01"と"05"のように2桁で表示される（`--digits: 2`の効果）

