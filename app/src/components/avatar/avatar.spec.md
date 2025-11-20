# Avatar Component Specification

## Related Files

- Implementation: `app/src/components/avatar/mod.rs`
- Module: `app/src/components/mod.rs`

## Related Documentation

- DaisyUI Documentation: https://daisyui.com/components/avatar/

## Requirements

### 責務

- アバター画像の表示と管理
- DaisyUIの`avatar`クラスを使用したスタイリング
- AvatarGroupコンポーネントとの連携による複数アバターの表示

### Props構造

#### Avatarコンポーネント

- `src` (required, String): 画像のURL（必須）
- `alt` (optional, String): 画像の代替テキスト
- `class` (optional, String): 追加のCSSクラス

#### AvatarGroupコンポーネント

- `class` (optional, String): 追加のCSSクラス
- `children` (Children): Avatarコンポーネントの子要素

### スタイル

- `Avatar`: `avatar`クラスを適用し、内部に`w-12 rounded-full`クラスを持つ`<div>`で画像をラップ
- `AvatarGroup`: `avatar-group -space-x-6`クラスを適用して複数のアバターを重ねて表示

## Test Cases

### TC-001: Avatarの基本表示

- Given: src="https://example.com/avatar.jpg"
- When: Avatarコンポーネントをレンダリング
- Then: `avatar`クラスが適用された`<div>`要素が表示され、内部に画像が表示される

### TC-002: Avatarの画像表示

- Given: src="https://example.com/avatar.jpg", alt="User Avatar"
- When: Avatarコンポーネントをレンダリング
- Then: `<img>`要素が表示され、srcとalt属性が正しく設定される

### TC-003: Avatarのサイズとスタイル

- Given: src="https://example.com/avatar.jpg"
- When: Avatarコンポーネントをレンダリング
- Then: 内部の`<div>`に`w-12 rounded-full`クラスが適用される

### TC-004: Avatarのカスタムクラス

- Given: src="https://example.com/avatar.jpg", class="avatar-lg"
- When: Avatarコンポーネントをレンダリング
- Then: ベースクラスに加えて"avatar-lg"が追加される

### TC-005: AvatarGroupの基本表示

- Given: AvatarGroupコンポーネントをレンダリング
- When: ページを表示
- Then: `avatar-group -space-x-6`クラスが適用された`<div>`要素が表示される

### TC-006: AvatarGroup内の複数Avatar

- Given: AvatarGroup内に複数のAvatarコンポーネントを配置
- When: AvatarGroupコンポーネントをレンダリング
- Then: 複数のアバターが重ねて表示される

### TC-007: AvatarGroupのカスタムクラス

- Given: AvatarGroupにclass="justify-end"を設定
- When: AvatarGroupコンポーネントをレンダリング
- Then: ベースクラスに加えて"justify-end"が追加される

