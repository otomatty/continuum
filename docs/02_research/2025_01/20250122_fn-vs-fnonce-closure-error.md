# Leptos `Fn` vs `FnOnce` クロージャエラーの解決方法

## 概要

Leptosの`Show`コンポーネントで`fallback`プロパティにクロージャを渡す際に、以下のエラーが発生しました：

```
expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
required for `TypedChildrenFn<impl leptos::IntoView>` to implement `ToChildren<{closure}>`
```

## エラーの原因

### `Fn`と`FnOnce`の違い

- **`Fn`**: クロージャが複数回呼び出し可能（immutable borrow）
- **`FnOnce`**: クロージャが1回だけ呼び出し可能（ownershipを取得）

### 問題のコード

```rust
view! {
    <Show
        when=is_authenticated
        fallback=move || {
            view! {
                <Button
                    variant=ButtonVariant::Primary
                    class=combined_class_for_children  // ❌ この値が移動される
                    on_click=authenticated_callback_clone
                >
                    "ダッシュボードへ"
                </Button>
            }
        }
    >
        // ...
    </Show>
}
```

**問題点:**
- `combined_class_for_children`がクロージャ内で直接使用され、所有権が移動（move）される
- これにより、クロージャが2回目以降呼び出せなくなり、`FnOnce`になってしまう
- `Show`コンポーネントは`fallback`クロージャが`Fn`トレイトを実装することを要求する

## 解決方法

### 解決策1: クロージャ内で`.clone()`を呼び出す

クロージャ内で値を直接使用するのではなく、`.clone()`を呼び出してクローンを作成します。

```rust
view! {
    <Show
        when=is_authenticated
        fallback=move || {
            let class = fallback_class.clone();  // ✅ クロージャ内でクローン
            let callback = fallback_callback.clone();
            let btn_text = fallback_text.clone();
            view! {
                <Button
                    variant=ButtonVariant::Primary
                    class=class
                    on_click=callback
                >
                    {btn_text}
                </Button>
            }
        }
    >
        // ...
    </Show>
}
```

### 解決策2: 使用箇所で直接`.clone()`を呼び出す

クロージャの外で事前にクローンを作成するのではなく、使用箇所で直接`.clone()`を呼び出します。

```rust
view! {
    <Show
        when=is_authenticated
        fallback=move || {
            // fallback内の処理
        }
    >
        <Button
            variant=ButtonVariant::Primary
            class=combined_class.clone()  // ✅ 使用箇所で直接クローン
            on_click=authenticated_callback_clone
        >
            "ダッシュボードへ"
        </Button>
    </Show>
}
```

## 実装例

### 最終的な解決コード

```rust
// Clone values for fallback - clone before move closure to ensure Fn instead of FnOnce
let fallback_class = combined_class.clone();
let fallback_callback = login_callback.clone();
let fallback_text = text.clone();

view! {
    <Show
        when=is_authenticated
        fallback=move || {
            // クロージャ内でクローンを作成してから使用
            let class = fallback_class.clone();
            let callback = fallback_callback.clone();
            let btn_text = fallback_text.clone();
            view! {
                <Button
                    variant=ButtonVariant::Primary
                    class=class
                    on_click=callback
                >
                    {btn_text}
                </Button>
            }
        }
    >
        <Button
            variant=ButtonVariant::Primary
            class=combined_class.clone()  // 使用箇所で直接クローン
            on_click=authenticated_callback_clone
        >
            "ダッシュボードへ"
        </Button>
    </Show>
}
```

## 重要なポイント

1. **`String`は`Copy`トレイトを実装していない**: `String`型の値は所有権を移動するため、クロージャ内で直接使用すると`FnOnce`になる

2. **`.clone()`で所有権を保持**: クロージャ内で`.clone()`を呼び出すことで、元の値の所有権を保持し、クロージャを`Fn`にできる

3. **`ReadSignal`は`Copy`**: `ReadSignal`などのシグナル型は`Copy`トレイトを実装しているため、クロージャ内で直接使用しても問題ない

4. **`Callback`は`Clone`を実装**: `Callback`型は`Clone`トレイトを実装しているため、`.clone()`で複製可能

## チェックリスト

Leptosでクロージャを使用する際は、以下を確認してください：

- [ ] クロージャ内で使用する`String`型の値は`.clone()`でクローンしているか？
- [ ] クロージャが`Fn`トレイトを実装する必要がある場合、値の所有権が移動していないか？
- [ ] `Copy`トレイトを実装している型（`ReadSignal`など）は直接使用できるか？
- [ ] `Clone`トレイトを実装している型（`Callback`など）は`.clone()`で複製できるか？

## 関連ファイル

- `app/src/components/github_login_button/mod.rs`: このエラーが発生した実装ファイル
- `app/src/components/button/mod.rs`: `Button`コンポーネントの定義

## 参考資料

- [Rust Book - Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Leptos Documentation - Show Component](https://leptos.dev/leptos/view/fn.Show.html)

## 日付

2025年1月22日

