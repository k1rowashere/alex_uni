// mod app;
// mod class;
// mod components;
// mod grades;
mod auth;
mod profile;
// mod registration;
// mod timetable;
mod pages;
mod utils;

use std::sync::LazyLock;

use actix_files::{Files, NamedFile};
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::*;
use cookie::time::Duration;

use tera::Tera;
use utils::*;

const WEEK: u64 = 60 * 60 * 24 * 7;

pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let site_root = get_env("SITE_ROOT");
    let icons = format!("{site_root}/assets/icons");

    let mut tera = Tera::new(&format!("{site_root}/pages/**/*.html")).unwrap();
    tera.add_template_files([
        (format!("{icons}/spinner.svg"), Some("spinner.svg")),
        (format!("{icons}/show_hide.svg"), Some("show_hide.svg")),
    ])
    .unwrap();

    tera.register_filter("svg_icon", utils::svg_icon_filter);
    tera
});

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();
    let site_addr = get_env("SITE_ADDR");
    let site_root = get_env("SITE_ROOT");

    let db = sqlx::SqlitePool::connect(&get_env("DATABASE_URL"))
        .await
        .expect("Failed to connect to DB");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run sqlx migrations");

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/assets", format!("{site_root}/assets")))
            .service(Files::new("/style", format!("{site_root}/style")))
            .service(favicon)
            .service(pages::login)
            .service(pages::signup)
            .service(
                web::scope("/api")
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout))
                    .route("/profile_icon", web::get().to(profile::get_profile_icon)),
            )
            .service(
                web::scope("")
                    .service(pages::home)
                    .wrap(middleware::from_fn(auth::auth_wrapper)),
            )
            .default_service(web::to(pages::not_found))
            .wrap(middleware::Compress::default())
            .wrap(
                IdentityMiddleware::builder()
                    .visit_deadline(Some(std::time::Duration::new(20 * WEEK, 0)))
                    .build(),
            )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), get_secret())
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::weeks(100)),
                    )
                    .build(),
            )
            .app_data(web::Data::new(db.clone()))
    })
    .bind(&site_addr)?
    .run()
    .await
}

#[get("favicon.ico")]
async fn favicon() -> actix_web::Result<NamedFile> {
    let site_root = get_env("SITE_ROOT");
    Ok(NamedFile::open(format!("{site_root}/assets/favicon.svg"))?)
}
