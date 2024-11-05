use crate::{auth, TEMPLATES};
use actix_identity::Identity;
use actix_web::*;
use error::ErrorInternalServerError;
use web::Data;

#[derive(serde::Deserialize)]
struct LoginQuery {
    next: Option<String>,
}

#[derive(serde::Serialize)]
struct NavLink<'a> {
    name: &'a str,
    url: &'a str,
    icon: &'a str,
}

const NAVLINKS: [NavLink<'_>; 2] = [
    NavLink {
        name: "Home",
        url: "/",
        icon: "BiHomeAlt2Regular",
    },
    NavLink {
        name: "Course Registration",
        url: "/registration",
        icon: "BiBookAddRegular",
    },
];

#[get("/")]
pub async fn home(id: Identity, db: Data<sqlx::SqlitePool>) -> impl Responder {
    let user = auth::get_user(&id.id()?, &db)
        .await
        .map_err(ErrorInternalServerError)?;

    let mut ctx = tera::Context::new();
    ctx.insert("nav_links", &NAVLINKS);
    ctx.insert("user", &user);
    TEMPLATES
        .render("home.html", &ctx)
        .inspect_err(|e| eprintln!("{:?}", e))
        .map_err(error::ErrorInternalServerError)
        .map(web::Html::new)
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("404: Page Not Found")
}

#[get("/login")]
pub async fn login(web::Query(query): web::Query<LoginQuery>) -> impl Responder {
    // let is_redirect = match &query.next {
    //     Some(s) => s != "/",
    //     None => false,
    // };
    let mut ctx = tera::Context::new();
    //ctx.insert("is_redirect", &is_redirect);
    TEMPLATES
        .render("login.html", &ctx)
        .inspect_err(|e| eprintln!("{:?}", e))
        .map_err(error::ErrorInternalServerError)
        .map(web::Html::new)
}

#[get("/signup")]
pub async fn signup() -> impl Responder {
    let mut ctx = tera::Context::new();
    TEMPLATES
        .render("signup.html", &ctx)
        .inspect_err(|e| eprintln!("{:?}", e))
        .map_err(error::ErrorInternalServerError)
        .map(web::Html::new)
}
