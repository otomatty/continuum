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

- `Countdown`: `countdown`クラスを適用、`data-value`属性に残り秒数を設定

### 機能

- 1秒ごとにカウントダウンを更新
- 残り時間を時:分:秒形式で表示
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
- Then: "01:00:00"形式で残り時間が表示される

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
- Then: "00:00:00"が表示される

### TC-006: Countdownのカスタムクラス

- Given: Countdownにclass="countdown-lg"を設定
- When: Countdownコンポーネントをレンダリング
- Then: ベースクラスに加えて"countdown-lg"が追加される

### TC-007: data-value属性の設定

- Given: target_date=現在時刻+100秒
- When: Countdownコンポーネントをレンダリング
- Then: `data-value`属性に100が設定される

### TC-008: タイマーのクリーンアップ

- Given: Countdownコンポーネントがアンマウントされる
- When: コンポーネントが削除される
- Then: タイマーがクリーンアップされる

