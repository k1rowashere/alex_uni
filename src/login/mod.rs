use leptos::*;
use leptos_router::*;

use crate::components::input::Input;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type), sqlx(transparent))]
pub struct UserId(i64);

impl From<i64> for UserId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct JwtClaims {
    sub: UserId,
    exp: i64,
    iat: i64,
}

#[server]
async fn auth(
    username: String,
    password: String,
) -> Result<Option<UserId>, ServerFnError> {
    let pool = crate::utils::extract_pool().await;

    let query = sqlx::query!(
        "SELECT id, password FROM users WHERE username = ?",
        username
    )
    .fetch_optional(&pool)
    .await?
    .map(|r| (r.id, r.password));

    let user_id = match query {
        Some((id, hash)) => {
            bcrypt::verify(&password, hash.as_str())?.then_some(UserId(id))
        }
        None => None,
    };
    Ok(user_id)
}

#[server]
async fn login(
    std_id: String,
    password: String,
) -> Result<bool, ServerFnError> {
    use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue};
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};

    let user = auth(std_id, password).await?;
    let res = expect_context::<leptos_actix::ResponseOptions>();

    match user {
        Some(user_id) => {
            let my_claims = JwtClaims {
                sub: user_id,
                iat: Utc::now().timestamp(),
                exp: (Utc::now() + Duration::days(30)).timestamp(),
            };
            let encoding_key = EncodingKey::from_secret(
                std::env::var("SECRET_KEY")
                    .expect("Expected SECRET_KEY")
                    .as_bytes(),
            );
            let token = encode(&Header::default(), &my_claims, &encoding_key)?;

            let cookie = Cookie::build("session", token)
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(actix_web::cookie::SameSite::Lax)
                .finish();

            if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
                res.insert_header(header::SET_COOKIE, cookie);
            }
            // INFO: redirection is done on the client side
            //       because it doesn't set the cookie, till after the page loads
            // FIX: Try to use a multi-redirect to solve this issue
            // leptos_actix::redirect("/redirect");
            Ok(true)
        }
        None => Ok(false),
    }
}

#[server]
async fn logout() -> Result<(), ServerFnError> {
    use actix_web::cookie::Cookie;
    use actix_web::http::header;

    let res = expect_context::<leptos_actix::ResponseOptions>();
    let cookie = Cookie::build("session", "")
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::Lax)
        .finish();

    if let Ok(cookie) = header::HeaderValue::from_str(&cookie.to_string()) {
        res.insert_header(header::SET_COOKIE, cookie);
    }
    Ok(())
}

#[cfg(feature = "ssr")]
pub fn user_id_from_jwt(req: &actix_web::HttpRequest) -> Option<UserId> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    let sess = req.cookie("session")?;
    let jwt = sess.value();

    let key = DecodingKey::from_secret(
        std::env::var("SECRET_KEY")
            .expect("Expected SECRET_KEY")
            .as_bytes(),
    );
    let validation = Validation::default();
    let td = decode::<JwtClaims>(jwt, &key, &validation).ok()?;
    Some(td.claims.sub)
}

#[server]
pub async fn get_user_info() -> Result<Option<User>, ServerFnError> {
    let req = expect_context::<actix_web::HttpRequest>();
    let pool = crate::utils::extract_pool().await;

    let user_id = user_id_from_jwt(&req);

    match user_id {
        Some(uid) => {
            let user = sqlx::query_as!(
                User,
                r#"SELECT id, name FROM users WHERE id=?"#,
                uid
            )
            .fetch_optional(&pool)
            .await?;
            Ok(user)
        }
        None => {
            logout().await?;
            Ok(None)
        }
    }
}

#[component]
pub fn LoginPage(
    action: Action<Login, Result<bool, ServerFnError>>,
    user: crate::app::UserResource,
) -> impl IntoView {
    let form_ref = create_node_ref::<html::Form>();
    let add_submit_class = move |_| {
        let _ = form_ref().unwrap().classes("submit-attempt");
    };

    view! {
        <Suspense>
            <Show when=move || user.with(|u| matches!(u, Some(Ok(Some(_)))))>
                 <Redirect path="/"/>
            </Show>
        </Suspense>
        <div class="h-screen w-100 flex content-center">
            <div class="transition-opacity mx-auto my-auto p-8 bg-secondary rounded-2xl shadow-xl">
                <h1 class="font-bold text-4xl">
                    Alexandria University
                </h1>
                <hr class="border-b-2 border-b-current mr-32"/>
                <h2 class="font-bold text-xl mt-6 mb-4">
                    Login
                </h2>
                <ActionForm node_ref=form_ref class="flex flex-col gap-3" action=action>
                    <Input id="std_id" label="Student ID" required=true/>
                    <Input
                        id="password"
                        label="Password"
                        attr:type="password"
                        required=true
                    />
                    <span class="text-xs text-red-400">
                        {move || {
                            match action.value().get() {
                                Some(Ok(true)) | None => " ".to_owned(),
                                Some(Ok(false)) => "Invalid Username or Password".to_owned(),
                                Some(Err(e)) => format!("Server Error: {e}"),
                            }
                        }}
                    </span>
                    <a href="/forgot_password" class="text-gray text-sm">
                        "Forgot Password?"
                    </a>
                    <button class="btn-primary" type="submit" value="" on:click=add_submit_class>
                        {move || if action.pending().get() { "Loading..." } else { "Login" }}
                    </button>
                </ActionForm>
            </div>
        </div>
    }
}
