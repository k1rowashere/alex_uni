use core::fmt;
use std::{collections::HashMap, str::FromStr};

use actix_identity::Identity;
use actix_web::error::ErrorInternalServerError;
use actix_web::*;
use body::MessageBody;
use dev::{ServiceRequest, ServiceResponse};
use http::{header, header::ContentType, StatusCode, Uri};
use middleware::Next;

use crate::{utils::htmx_headers, Database};

#[derive(serde::Serialize)]
pub struct User {
    username: String,
    id: i32,
    name: String,
    //program: String,
}

#[derive(serde::Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

pub async fn login(
    req: HttpRequest,
    login_info: web::Form<LoginInfo>,
    db: web::Data<Database>,
) -> Result<impl Responder> {
    let LoginInfo { username, password } = login_info.into_inner();

    let user = sqlx::query!(
        r#"
            SELECT users.id, password 
            FROM users 
            LEFT JOIN students ON students.id = users.id
            WHERE username = $1
               OR (
                   student_id IS NOT NULL
                   AND $1 ~ '^[0-9]+$'
                   AND student_id = $1::INTEGER
               );
        "#,
        username
    )
    .fetch_optional(db.get_ref())
    .await
    .map_err(ErrorInternalServerError)?
    .and_then(|res| {
        bcrypt::verify(password, &res.password)
            .ok()
            .filter(|&v| v)
            .map(|_| res.id)
    });

    let redirect = req
        .headers()
        .get(header::REFERER)
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|uri_str| Uri::from_str(uri_str).ok())
        .and_then(|uri| web::Query::<HashMap<String, String>>::from_query(uri.query()?).ok())
        .and_then(|query| query.get("next").map(String::to_owned))
        .unwrap_or_else(|| "/".to_string());

    let res = match user {
        Some(u) => {
            let header = match req.headers().get(htmx_headers::HX_REQUEST) {
                Some(_) => htmx_headers::HX_LOCATION,
                None => header::LOCATION,
            };
            Identity::login(&req.extensions(), u.to_string())?;
            HttpResponseBuilder::new(StatusCode::SEE_OTHER)
                .append_header((header, redirect))
                .append_header((htmx_headers::HX_REFRESH, "true"))
                .finish()
        }
        None => HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
            .content_type(ContentType::html())
            .body("Invalid Username or Password."),
    };
    Ok(res)
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

// TODO: generalize this function to get any user type
pub async fn get_user(id: &str, db: &Database) -> Result<User, sqlx::Error> {
    let id: i32 = id.parse().map_err(|_| sqlx::Error::RowNotFound)?;
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT COALESCE(student_id, users.id) AS "id!", username, name  -- id is never NULL
            FROM users
            LEFT JOIN students ON students.id = users.id
            WHERE users.id = $1
        "#,
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
