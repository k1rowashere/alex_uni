#![cfg(not(feature = "login"))]
use crate::utils::UserId;
use leptos::*;

#[server(Login, "/api", "Url", "login")]
pub async fn login(
    _std_id: String,
    _password: String,
) -> Result<bool, ServerFnError> {
    Ok(true)
}

#[server(Logout, "/api", "Url", "logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    Ok(())
}

#[server(GetUserInfo, "/api", "Url", "get_user_info")]
pub async fn get_user_info() -> Result<Option<UserId>, ServerFnError> {
    Ok(Some("0".to_string()))
}

#[component]
pub fn login_page<F>(
    action: Action<Login, Result<bool, ServerFnError>>,
    logged_in: F,
) -> impl IntoView
where
    F: Fn() -> Option<bool> + 'static + Copy,
{
    let _ = action;
    let _ = logged_in;
    view! {"The login feature is disabled"}
}
