# UIコンポーネントのテスト戦略

## 作成日
2025年1月

## 概要

Legible Architectureの原則に基づき、UIコンポーネントのテスト戦略を定義します。

## テストの必要性

### なぜUIコンポーネントのテストが必要か

1. **Legible Architectureの原則に従う**
   - `.spec.md`に定義されたTest Casesを実装することで、仕様と実装の整合性を保証
   - テスト駆動開発により、品質の高いコードを実現

2. **回帰防止**
   - リファクタリング時の安全性を確保
   - 既存機能が壊れていないことを保証

3. **ドキュメントとしての役割**
   - テストコードがコンポーネントの使用方法を示す
   - 新しい開発者がコンポーネントの動作を理解しやすくなる

## Leptosコンポーネントのテストアプローチ

Leptosコンポーネントのテストは、以下の3つのレイヤーで構成します：

### 1. ユニットテスト（ロジック部分）

**対象:**
- variant enumの判定ロジック
- クラス名生成ロジック
- 状態管理ロジック（純粋関数として抽出可能な部分）

**実装場所:**
- `app/src/components/{component-name}/tests.rs`

**例:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant_class_generation() {
        let class = get_variant_class(AccordionVariant::Arrow);
        assert_eq!(class, "collapse-title");
    }
}
```

**メリット:**
- 高速に実行できる
- ブラウザ環境が不要
- CI/CDパイプラインで簡単に実行可能

### 2. E2Eテスト（実際の動作）

**対象:**
- コンポーネントの実際のレンダリング結果
- ユーザーインタラクション（クリック、入力など）
- ブラウザ上での動作確認

**実装場所:**
- `end2end/tests/{component-name}.spec.ts`

**例:**
```typescript
import { test, expect } from "@playwright/test";

test("accordion opens and closes", async ({ page }) => {
  await page.goto("http://localhost:3000/test-accordion");
  
  const accordion = page.locator(".collapse");
  await expect(accordion).not.toHaveClass(/collapse-open/);
  
  await accordion.click();
  await expect(accordion).toHaveClass(/collapse-open/);
});
```

**メリット:**
- 実際のブラウザ環境で動作確認
- ユーザー体験に近いテストが可能
- CSSやスタイリングの問題も検出可能

### 3. ビジュアルリグレッションテスト（オプション）

**対象:**
- コンポーネントの見た目の変化を検出

**実装場所:**
- Playwrightのスクリーンショット機能を使用

**例:**
```typescript
test("accordion visual regression", async ({ page }) => {
  await page.goto("http://localhost:3000/test-accordion");
  await expect(page).toHaveScreenshot("accordion.png");
});
```

## 実装ガイドライン

### テストファイルの構造

```
app/src/components/{component-name}/
├── mod.rs              # コンポーネント実装
├── {component-name}.spec.md  # 仕様書（Test Cases含む）
└── tests.rs            # ユニットテスト
```

### テストケースの記述方法

1. **`.spec.md`のTest Casesに基づいて実装**
   - `.spec.md`に定義されたTest Casesを`tests.rs`に実装
   - Test CasesのID（例: TC-001）をコメントで明記

2. **ロジック部分を純粋関数として抽出**
   - コンポーネント内のロジックを純粋関数として抽出
   - 抽出した関数をテスト可能にする

3. **テストの命名規則**
   - `test_{test_case_id}_{description}`形式で命名
   - 例: `test_tc_001_accordion_basic_display`

### 実装例

#### 1. ロジック部分の抽出

```rust
// mod.rs内で、テスト可能なロジックを抽出
fn get_variant_class(variant: AccordionVariant) -> String {
    match variant {
        AccordionVariant::Arrow => "collapse-title".to_string(),
        AccordionVariant::Plus => "collapse-title collapse-plus".to_string(),
    }
}

#[component]
pub fn AccordionHeader(
    #[prop(optional)] variant: AccordionVariant,
    // ...
) -> impl IntoView {
    let variant_class = get_variant_class(variant);
    // ...
}
```

#### 2. テストの実装

```rust
// tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tc_002_arrow_variant_application() {
        // TC-002: Arrowバリアントの適用
        let class = get_variant_class(AccordionVariant::Arrow);
        assert_eq!(class, "collapse-title");
    }
}
```

## テスト実行方法

### ユニットテストの実行

```bash
# 特定のコンポーネントのテストを実行
cargo test --package app --lib components::accordion::tests

# すべてのコンポーネントのテストを実行
cargo test --package app --lib components

# テストを実行してカバレッジを確認
cargo test --package app --lib components -- --nocapture
```

### E2Eテストの実行

```bash
# 開発環境でE2Eテストを実行
cargo leptos end-to-end

# リリースビルドでE2Eテストを実行
cargo leptos end-to-end --release
```

## テストの優先順位

### 必須テスト（すべてのコンポーネント）

1. **variant enumの判定ロジック**
   - 各variantが正しいクラス名を生成するか

2. **クラス名生成ロジック**
   - デフォルトクラスとカスタムクラスの結合が正しいか

3. **基本状態の確認**
   - デフォルト状態が正しく設定されるか

### 推奨テスト（インタラクティブなコンポーネント）

1. **状態管理ロジック**
   - 開閉状態の切り替えが正しく動作するか

2. **イベントハンドリング**
   - クリックイベントが正しく発火するか

3. **E2Eテスト**
   - 実際のブラウザ上での動作確認

## 注意事項

### Leptosコンポーネントのテストの制限

1. **レンダリング結果の直接テストは困難**
   - Leptosコンポーネントのレンダリング結果を直接テストするのは難しい
   - 代わりに、ロジック部分を純粋関数として抽出してテスト

2. **Signalのテスト**
   - Signalの動作は、E2Eテストで確認する
   - ユニットテストでは、Signalの値の判定ロジックをテスト

3. **ブラウザ環境の必要性**
   - E2Eテストはブラウザ環境が必要
   - CI/CDパイプラインで実行する場合は、Playwrightのセットアップが必要

## まとめ

UIコンポーネントのテストは、以下の理由で重要です：

1. **品質保証**: 仕様と実装の整合性を保証
2. **回帰防止**: リファクタリング時の安全性を確保
3. **ドキュメント**: テストコードがコンポーネントの使用方法を示す

実装時は、以下のアプローチを推奨します：

1. **ロジック部分のユニットテスト**: 高速で実行可能
2. **E2Eテスト**: 実際の動作を確認
3. **`.spec.md`との整合性**: Test Casesに基づいて実装

## 関連ドキュメント

- Legible Architecture原則: `docs/00_prompts/legible-architecture.md`
- Accordion Component Spec: `app/src/components/accordion/accordion.spec.md`
- Accordion Component Tests: `app/src/components/accordion/tests.rs`

