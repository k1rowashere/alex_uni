use core::fmt;
use std::{collections::HashMap, str::FromStr};

use actix_identity::Identity;
use actix_web::error::ErrorInternalServerError;
use actix_web::*;
use body::MessageBody;
use dev::{ServiceRequest, ServiceResponse};
use http::{header, header::ContentType, StatusCode, Uri};
use middleware::Next;
use sqlx::SqlitePool;
use tera::Context;
use web::Html;

use crate::{
    utils::{self, htmx_headers},
    TEMPLATES,
};

#[derive(serde::Serialize)]
pub struct User {
    username: String,
    id: i64,
    name: String,
    //program: String,
}

#[derive(serde::Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct SignupInfo {
    username: String,
    email: String,
    name: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct ValidateUsernameInfo {
    username: String,
}

async fn is_valid_username(username: &str, db: &SqlitePool) -> sqlx::Result<bool> {
    let user_exists = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?) as user_exists",
        username
    )
    .fetch_one(db)
    .await?;
    Ok(user_exists.user_exists == 0 && username.len() > 3)
}

pub async fn validate_username(
    username: web::Query<ValidateUsernameInfo>,
    db: web::Data<SqlitePool>,
) -> Result<impl Responder> {
    let username = &username.username;
    let valid_username = is_valid_username(username, db.get_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    let str = TEMPLATES.render_str(
            if valid_username {
                r#"{% import "partial/components.html" as cmp %}{{ cmp::icon(icon="FiCheck", class="text-green-500") }}"#
            } else {
                r#"{% import "partial/components.html" as cmp %}{{ cmp::icon(icon="FiX", class="text-red-500") }}"#
            },
            &Context::new(),
        )
        .map_err(ErrorInternalServerError)?;

    Ok(Html::new(str))
}

pub async fn signup(
    req: HttpRequest,
    signup_info: web::Form<SignupInfo>,
    db: web::Data<SqlitePool>,
) -> Result<impl Responder> {
    let SignupInfo {
        username,
        email,
        name,
        password,
    } = signup_info.into_inner();

    let is_valid = {
        let valid_username = is_valid_username(&username, &db)
            .await
            .map_err(ErrorInternalServerError)?;

        valid_username
            && email.contains("@")
            && email.contains(".")
            && name.len() > 3
            && password.len() >= 8
    };

    if !is_valid {
        return Ok(HttpResponse::BadRequest().body("One of the parameters entered is incorrect."));
    }

    let hashed_password =
        bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(ErrorInternalServerError)?;
    let res = sqlx::query!(
        "INSERT INTO users (username, password, email, name) VALUES (?, ?, ?, ?)",
        username,
        hashed_password,
        email,
        name
    )
    .execute(db.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    let rows_affected = res.rows_affected();

    if rows_affected == 0 {
        return Ok(HttpResponse::Conflict().body("Username Already Taken."));
    }

    let id = res.last_insert_rowid();
    Identity::login(&req.extensions(), id.to_string())?;

    Ok(HttpResponse::Created()
        .append_header((utils::htmx_headers::HX_REDIRECT, "/"))
        .finish())
}

pub async fn login(
    req: HttpRequest,
    login_info: web::Form<LoginInfo>,
    db: web::Data<SqlitePool>,
) -> Result<impl Responder> {
    let LoginInfo { username, password } = login_info.into_inner();

    let user = sqlx::query!(
        "SELECT id, password FROM users WHERE username = ?",
        username
    )
    .fetch_optional(db.get_ref())
    .await
    .map_err(ErrorInternalServerError)?
    .and_then(|res| match bcrypt::verify(password, &res.password) {
        Ok(true) => Some(res.id),
        _ => None,
    });

    let redirect = req
        .headers()
        .get(header::REFERER)
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|uri_str| Uri::from_str(uri_str).ok())
        .and_then(|uri| web::Query::<HashMap<String, String>>::from_query(uri.query()?).ok())
        .and_then(|query| query.get("next").map(String::to_owned))
        .unwrap_or_else(|| "/".to_string());

    match user {
        Some(u) => {
            let header = match req.headers().get(htmx_headers::HX_REQUEST) {
                Some(_) => htmx_headers::HX_LOCATION,
                None => header::LOCATION,
            };
            Identity::login(&req.extensions(), u.to_string())?;
            Ok(HttpResponseBuilder::new(StatusCode::SEE_OTHER)
                .append_header((header, redirect))
                .body(""))
        }
        None => Ok(HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
            .content_type(ContentType::html())
            .body("Invalid Username or Password.")),
    }
}

pub async fn logout(id: Option<Identity>) -> impl Responder {
    if let Some(id) = id {
        id.logout();
    }
    web::Redirect::to("/login").see_other()
}

/// Wrapper around a &str, that percent encodes the string on format
struct EscapedQuery<'a>(&'a str);

impl<'a> fmt::Display for EscapedQuery<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.0.as_bytes();
        header::http_percent_encode(f, bytes)
    }
}

pub async fn get_user(id: &str, db: &SqlitePool) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, name FROM users WHERE id = ?",
        id
    )
    .fetch_one(db)
    .await?;
    Ok(user)
}

pub async fn auth_wrapper(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let path = req
        .uri()
        .path_and_query()
        .map(|m| EscapedQuery(m.as_str()))
        .filter(|follow_up| follow_up.0 != "/")
        .map(|follow_up| format!("/login?next={follow_up}"))
        .unwrap_or_else(|| "/login".into());

    let res = next.call(req).await;
    // if the response is a 401, then redirect to log in
    match res {
        Ok(res) => {
            let res = if res.status() == StatusCode::UNAUTHORIZED {
                res.into_response(
                    HttpResponse::Found()
                        .append_header((header::LOCATION, path))
                        .finish(),
                )
                .map_into_left_body()
            } else {
                res.map_into_right_body()
            };
            Ok(res)
        }
        Err(err) => Err(err),
    }
}
