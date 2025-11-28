#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::components::github_login_button::utils;
    use crate::hooks::AuthStatus;

    // TC-009: 認証済みの場合、「ダッシュボードへ」ボタンが表示される
    #[test]
    fn test_tc009_authenticated_button_display() {
        // Given: 認証状態がauthenticated: trueである
        let auth_status = AuthStatus {
            authenticated: true,
            user_id: Some("user123".to_string()),
        };

        // When: is_authenticated()を呼び出す
        let result = utils::is_authenticated(Some(auth_status.clone()));

        // Then: trueが返される（ダッシュボードへボタンが表示される条件）
        assert!(result, "認証済みユーザーの場合、trueが返されるべき");

        // And: ダッシュボードURLが正しい
        assert_eq!(utils::get_dashboard_url(), "/dashboard");
    }

    // TC-010: 未認証の場合、「GitHubでログイン」ボタンが表示される
    #[test]
    fn test_tc010_unauthenticated_button_display() {
        // Given: 認証状態がauthenticated: falseまたはNoneである
        let unauthenticated_status = AuthStatus {
            authenticated: false,
            user_id: None,
        };

        // When: is_authenticated()を呼び出す（未認証状態）
        let result_false = utils::is_authenticated(Some(unauthenticated_status.clone()));

        // Then: falseが返される（GitHubでログインボタンが表示される条件）
        assert!(!result_false, "未認証ユーザーの場合、falseが返されるべき");

        // When: is_authenticated()を呼び出す（None）
        let result_none = utils::is_authenticated(None);

        // Then: falseが返される
        assert!(!result_none, "認証状態がNoneの場合、falseが返されるべき");

        // And: ログインURLが正しい
        assert_eq!(utils::get_login_url(), "/auth/login");
    }

    // TC-011: 認証状態が変更された場合、ボタンの表示が切り替わる
    #[test]
    fn test_tc011_auth_status_change() {
        // Given: 初期状態が未認証である
        let unauthenticated_status = AuthStatus {
            authenticated: false,
            user_id: None,
        };

        let initial_result = utils::is_authenticated(Some(unauthenticated_status.clone()));
        assert!(!initial_result, "初期状態は未認証であるべき");

        // When: 認証状態をauthenticated: trueに更新
        let authenticated_status = AuthStatus {
            authenticated: true,
            user_id: Some("user123".to_string()),
        };

        let updated_result = utils::is_authenticated(Some(authenticated_status.clone()));

        // Then: trueが返される（ダッシュボードへボタンに切り替わる）
        assert!(
            updated_result,
            "認証状態が更新された場合、trueが返されるべき"
        );

        // When: 再度未認証状態に戻す
        let back_to_unauthenticated = AuthStatus {
            authenticated: false,
            user_id: None,
        };

        let final_result = utils::is_authenticated(Some(back_to_unauthenticated));

        // Then: falseが返される（GitHubでログインボタンに戻る）
        assert!(!final_result, "未認証状態に戻した場合、falseが返されるべき");
    }

    // 追加のエッジケーステスト

    #[test]
    fn test_is_authenticated_with_none() {
        // Noneの場合、falseが返されるべき
        assert!(!utils::is_authenticated(None));
    }

    #[test]
    fn test_is_authenticated_with_authenticated_true() {
        let auth_status = AuthStatus {
            authenticated: true,
            user_id: Some("user123".to_string()),
        };
        assert!(utils::is_authenticated(Some(auth_status)));
    }

    #[test]
    fn test_is_authenticated_with_authenticated_false() {
        let auth_status = AuthStatus {
            authenticated: false,
            user_id: Some("user123".to_string()), // user_idがあってもauthenticatedがfalseならfalse
        };
        assert!(!utils::is_authenticated(Some(auth_status)));
    }

    #[test]
    fn test_is_authenticated_with_authenticated_true_no_user_id() {
        let auth_status = AuthStatus {
            authenticated: true,
            user_id: None, // user_idがなくてもauthenticatedがtrueならtrue
        };
        assert!(utils::is_authenticated(Some(auth_status)));
    }

    #[test]
    fn test_url_constants() {
        // URL定数が正しいことを確認
        assert_eq!(utils::get_dashboard_url(), "/dashboard");
        assert_eq!(utils::get_login_url(), "/auth/login");
    }
}
