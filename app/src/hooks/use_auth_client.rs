use super::{AuthContext, AuthStatus};
use leptos::prelude::*;

/// Provide authentication context to the component tree (client version)
/// Returns the AuthContext that was created
pub fn provide_auth_context(initial_status: Option<AuthStatus>) -> AuthContext {
    let (status, set_status) = signal(initial_status.clone());
    let cached_status = RwSignal::new(initial_status.clone());
    let cache_timestamp = RwSignal::new({
        if initial_status.is_some() {
            Some(get_current_timestamp())
        } else {
            None
        }
    });

    let context = AuthContext {
        status,
        set_status,
        cached_status,
        cache_timestamp,
    };

    provide_context(context.clone());
    context
}

/// Get current timestamp in seconds since Unix epoch
pub(crate) fn get_current_timestamp() -> u64 {
    use js_sys::Date;

    let date = Date::new_0();
    let timestamp_ms = date.get_time();
    (timestamp_ms / 1000.0) as u64
}

/// Check if cache is still valid (within 5 minutes)
fn is_cache_valid(timestamp: Option<u64>) -> bool {
    if let Some(ts) = timestamp {
        let current = get_current_timestamp();
        let cache_duration_secs = 300; // 5 minutes
        current.saturating_sub(ts) < cache_duration_secs
    } else {
        false
    }
}

/// Refresh auth status from server if cache is expired
pub async fn refresh_auth_status_if_needed(
    context: &AuthContext,
) -> Result<(), leptos::prelude::ServerFnError> {
    use super::get_auth_status;

    let should_refresh = {
        let cached = context.cached_status.get();
        let timestamp = context.cache_timestamp.get();

        // If no cache or cache expired, refresh
        cached.is_none() || !is_cache_valid(timestamp)
    };

    if should_refresh {
        match get_auth_status().await {
            Ok(new_status) => {
                context.set_status.set(Some(new_status.clone()));
                context.cached_status.set(Some(new_status));
                context.cache_timestamp.set(Some(get_current_timestamp()));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

/// Clear auth cache (used on logout)
pub fn clear_auth_cache(context: &AuthContext) {
    context.set_status.set(None);
    context.cached_status.set(None);
    context.cache_timestamp.set(None);
}

/// Read auth status from HTML data attribute
/// This should be called on the client side after hydration
pub fn read_auth_status_from_html() -> Option<AuthStatus> {
    use web_sys::window;

    if let Some(window_obj) = window() {
        if let Some(document) = window_obj.document() {
            if let Some(html_element) = document.document_element() {
                if let Some(auth_status_attr) = html_element.get_attribute("data-auth-status") {
                    if let Ok(auth_status) = serde_json::from_str::<AuthStatus>(&auth_status_attr) {
                        return Some(auth_status);
                    }
                }
            }
        }
    }

    None
}
