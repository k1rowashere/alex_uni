mod class_schedule;
mod grades;
pub use class_schedule::*;
pub use grades::*;

use crate::{auth, Database, TEMPLATES};
use actix_identity::Identity;
use actix_web::*;
use error::ErrorInternalServerError;
use web::Data;

#[get("/")]
pub async fn home(id: Identity, db: Data<Database>) -> impl Responder {
    let user = auth::get_user(&id.id()?, &db)
        .await
        .map_err(ErrorInternalServerError)?;

    let mut ctx = tera::Context::new();
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
pub async fn login() -> impl Responder {
    TEMPLATES
        .render("login.html", &tera::Context::new())
        .inspect_err(|e| eprintln!("{:?}", e)) // TODO: logging
        .map_err(error::ErrorInternalServerError)
        .map(web::Html::new)
}
