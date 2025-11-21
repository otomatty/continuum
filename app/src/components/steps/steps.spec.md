# Steps Component Specification

## Related Files

- Implementation: `app/src/components/steps/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/steps/

## Requirements

### 責務

- ステップインジケーターの表示
- DaisyUIの`steps`クラスを使用したスタイリング
- StepItemコンポーネントとの連携
- マルチステップフォームやプロセスの進捗表示

### Props構造

#### Stepsコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ステップの子要素（StepItem）

#### StepItemコンポーネント

- `status` (optional, StepStatus): ステップの状態（デフォルト: Default）
- `class` (optional, String): 追加のCSSクラス
- `children` (Children): ステップアイテムのテキスト

### StepStatus

以下のステータスをサポート：

- `Default`: デフォルト状態
- `Primary`: プライマリ状態
- `Success`: 成功状態
- `Warning`: 警告状態
- `Error`: エラー状態

### スタイル

- `Steps`: `steps`クラスを適用した`<ul>`要素
- `StepItem`: `step`クラスを適用、ステータスに応じて`step-primary`, `step-success`, `step-warning`, `step-error`クラスを追加

## Test Cases

### TC-001: Stepsの基本表示

- Given: Stepsコンポーネントをレンダリング
- When: ページを表示
- Then: `steps`クラスが適用された`<ul>`要素が表示される

### TC-002: StepItemのデフォルト状態

- Given: StepItemにstatusを設定しない
- When: StepItemコンポーネントをレンダリング
- Then: `step`クラスが適用される（ステータスクラスは含まれない）

### TC-003: StepItemのPrimary状態

- Given: status=StepStatus::Primary
- When: StepItemコンポーネントをレンダリング
- Then: `step step-primary`クラスが適用される

### TC-004: StepItemのSuccess状態

- Given: status=StepStatus::Success
- When: StepItemコンポーネントをレンダリング
- Then: `step step-success`クラスが適用される

### TC-005: StepItemのWarning状態

- Given: status=StepStatus::Warning
- When: StepItemコンポーネントをレンダリング
- Then: `step step-warning`クラスが適用される

### TC-006: StepItemのError状態

- Given: status=StepStatus::Error
- When: StepItemコンポーネントをレンダリング
- Then: `step step-error`クラスが適用される

### TC-007: Stepsのカスタムクラス

- Given: Stepsにclass="steps-vertical"を設定
- When: Stepsコンポーネントをレンダリング
- Then: ベースクラスに加えて"steps-vertical"が追加される

### TC-008: StepItemのカスタムクラス

- Given: StepItemにclass="font-bold"を設定
- When: StepItemコンポーネントをレンダリング
- Then: ベースクラスに加えて"font-bold"が追加される

### TC-009: Steps構造の組み合わせ

- Given: Steps内に複数のStepItem（異なるステータス）を配置
- When: Stepsコンポーネントをレンダリング
- Then: すべてのStepItemが正しく表示され、それぞれのステータスが適用される

### TC-010: マルチステップフォームの表示

- Given: Steps内に複数のStepItem（順番にDefault, Primary, Success）を配置
- When: Stepsコンポーネントをレンダリング
- Then: 進捗状況が視覚的に表現される

