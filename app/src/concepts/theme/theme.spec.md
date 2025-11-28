# Theme Concept Specification

## Related Files

- Implementation: `src/concepts/theme/mod.rs`
- State: `src/concepts/theme/state.rs`
- Actions: `src/concepts/theme/actions.rs`
- Tests: `src/concepts/theme/tests.rs`

## Related Documentation

- Issue: (To be created if needed)
- Plan: (To be created if needed)

## Requirements

### 責務
- アプリケーションのテーマ（ライト/ダーク/システム）の状態管理
- テーマの切り替えロジック

### 状態構造
- ThemeState: { current_theme: Theme }
- Theme: Light | Dark | System

### アクション
- set_theme: テーマを設定
- toggle_theme: ライトとダークを切り替え（Systemの場合はLightに設定）
- get_effective_theme: システム設定を考慮した実効テーマを取得（LightまたはDark）

## Test Cases

### TC-001: set_theme
- Given: ThemeState { current_theme: Theme::Light }
- When: set_theme(state, Theme::Dark)を実行
- Then: ThemeState { current_theme: Theme::Dark }が返される

### TC-002: toggle_theme_from_light
- Given: ThemeState { current_theme: Theme::Light }
- When: toggle_theme(state)を実行
- Then: ThemeState { current_theme: Theme::Dark }が返される

### TC-003: toggle_theme_from_dark
- Given: ThemeState { current_theme: Theme::Dark }
- When: toggle_theme(state)を実行
- Then: ThemeState { current_theme: Theme::Light }が返される

### TC-004: toggle_theme_from_system
- Given: ThemeState { current_theme: Theme::System }
- When: toggle_theme(state)を実行
- Then: ThemeState { current_theme: Theme::Light }が返される

### TC-005: get_effective_theme_light
- Given: ThemeState { current_theme: Theme::Light }
- When: get_effective_theme(state)を実行
- Then: Theme::Lightが返される

### TC-006: get_effective_theme_dark
- Given: ThemeState { current_theme: Theme::Dark }
- When: get_effective_theme(state)を実行
- Then: Theme::Darkが返される

### TC-007: get_effective_theme_system
- Given: ThemeState { current_theme: Theme::System }
- When: get_effective_theme(state)を実行（システム設定がダークの場合）
- Then: Theme::Darkが返される（システム設定に従う）

