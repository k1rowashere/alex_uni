use leptos::AdditionalAttributes as A;
use leptos::*;
use leptos_router::*;

#[component]
fn Input(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(default = false)] required: bool,
    #[prop(optional, into)] attributes: Option<MaybeSignal<AdditionalAttributes>>,
) -> impl IntoView {
    let mut input = view! {
        <input
            class="peer p-2 border border-gray-300 dark:border-gray-500 rounded \
                focus:!border-blue-500 outline-none w-full"
            id=&id
            name=&id
            required=required
            placeholder=" "
        />
    };

    if let Some(attributes) = attributes {
        let attributes = attributes.get();
        for (attr_name, attr_value) in attributes.into_iter() {
            let attr_name = attr_name.to_owned();
            let attr_value = attr_value.to_owned();
            input = input.attr(attr_name, move || attr_value.get());
        }
    }

    view! {
        <div class="relative mb-3 w-full">
            {input}
            <label for=id
                class="input_label cursor-text absolute bottom-2 left-2 \
                    text-gray bg-secondary origin-bottom-left transition-transform"
            >
            {label}
            </label>
        </div>
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct JwtClaims {
    sub: String,
    exp: usize,
    iat: usize,
}

type UserId = String;

#[server(Login, "/api", "Url", "login")]
async fn login(std_id: String, password: String) -> Result<bool, ServerFnError> {
    use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue};
    use bcrypt::{verify, BcryptResult};
    use chrono::{Duration, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use leptos_actix::ResponseOptions;

    fn auth(username: String, password: String) -> BcryptResult<Option<String>> {
        // test user cred: kirowashere:password
        const USERNAME: &str = "kirowashere";
        const HASHED: &str = "$2b$12$QQQ3hgxb8h.XvqzMLPA2Ne2lInO2CAoZXg7cSSZdXjzjLJMf.f.hK";

        // TODO: lookup user on db

        let is_valid = username == USERNAME && verify(password, &HASHED)?;

        if is_valid {
            Ok(Some("0".to_string().into()))
        } else {
            Ok(None)
        }
    }

    let user = auth(std_id, password)?;
    let res = expect_context::<ResponseOptions>();

    match user {
        Some(user_id) => {
            let my_claims = JwtClaims {
                sub: user_id,
                iat: Utc::now().timestamp() as usize,
                exp: (Utc::now() + Duration::minutes(10)).timestamp() as usize,
            };
            // TODO: use rsa key
            let encoding_key = EncodingKey::from_secret(b"secret");
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
            // FIX: Redirecting server side, cause the next page to be rendered without the user state.
            //      because cookies are not set yet.
            // leptos_actix::redirect("/");
            Ok(true)
        }
        None => Ok(false),
    }
}

#[server(Logout, "/api", "Url", "logout")]
async fn logout() -> Result<(), ServerFnError> {
    use leptos_actix::ResponseOptions;
    let res = expect_context::<ResponseOptions>();
    clear_session_cookie(&res);
    Ok(())
}

#[cfg(feature = "ssr")]
fn clear_session_cookie(res: &leptos_actix::ResponseOptions) {
    use actix_web::cookie::Cookie;
    use actix_web::http::header;

    let cookie = Cookie::build("session", "")
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::Lax)
        .finish();

    if let Ok(cookie) = header::HeaderValue::from_str(&cookie.to_string()) {
        res.insert_header(header::SET_COOKIE, cookie);
    }
}

#[server(GetUser, "/api", "Url", "get_user")]
pub async fn get_user() -> Result<Option<UserId>, ServerFnError> {
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use leptos_actix::extract;
    use leptos_actix::ResponseOptions;

    let res = expect_context::<ResponseOptions>();
    let jwt = extract(|req: actix_web::HttpRequest| async move {
        let cookies = req
            .cookies()
            .map_err(|_| ServerFnError::Deserialization("Error parsing session cookie".into()))?;
        let session_cookie = cookies
            .iter()
            .find(|el| el.name() == "session")
            .ok_or(ServerFnError::MissingArg("No session cookie".into()))?;

        Ok::<String, ServerFnError>(session_cookie.value().to_string())
    })
    .await??;
    // TODO: use rsa key
    let key = DecodingKey::from_secret(b"secret");
    let validation = Validation::default();
    let token_data = decode::<JwtClaims>(&jwt, &key, &validation);

    match token_data {
        Ok(t) => Ok(Some(t.claims.sub)),
        Err(_) => {
            clear_session_cookie(&res);
            Ok(None)
        }
    }
}

#[component]
pub fn LoginPage<F>(
    action: Action<Login, Result<bool, ServerFnError>>,
    logged_in: F,
) -> impl IntoView
where
    F: Fn() -> Option<bool> + 'static + Copy,
{
    let form_ref = create_node_ref::<html::Form>();
    let add_submit_class = move |_| {
        form_ref().unwrap().class("submit-attempt", true);
    };

    view! {
        <Suspense fallback=|| ()>
            {move || {
                if matches!(logged_in(), Some(true)) {
                    view! { <Redirect path="/"/> }.into_view()
                } else {
                    ().into_view()
                }
            }}
        </Suspense>
        <div class="h-screen w-screen flex content-center">
            <div class="transition-opacity mx-auto my-auto p-8 bg-secondary rounded-2xl shadow-xl">
                <h1 class="font-bold text-4xl">
                    Alexandria University
                </h1>
                <hr class="border-b-2 border-b-current mr-32"/>
                <h2 class="font-bold text-xl mt-6 mb-4">
                    Login
                </h2>
                <ActionForm node_ref=form_ref class="flex flex-col" action=action>
                    <Input id="std_id" label="Student ID" required=true/>
                    <Input
                        id="password"
                        label="Password"
                        required=true
                        attributes=A::from([("type", "password")])
                    />
                    <span class="text-xs text-red-400 mb-2">
                        {move || {
                            match action.value().get() {
                                None => " ",
                                Some(Ok(true)) => " ",
                                Some(Ok(false)) => "Invalid Username or Password",
                                Some(Err(_)) => "Server Error",
                            }
                        }}
                    </span>
                    <a href="/forgot_password" class="text-gray text-sm mt-2 mb-2">
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
