mod auth;
mod model;
mod pages;
mod profile;
mod utils;

use std::sync::LazyLock;

use actix_files::{Files, NamedFile};
use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::*;
use cookie::time::ext::NumericalDuration;

use tera::Tera;
use utils::*;

type Database = sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();
    let site_addr = get_env("SITE_ADDR");
    let site_root = get_env("SITE_ROOT");

    let db = Database::connect(&get_env("DATABASE_URL"))
        .await
        .expect("Failed to connect to DB");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run sqlx migrations");

    #[cfg(debug_assertions)]
    sqlx::query!(
        r"
        INSERT INTO users (username, password, email, name)
        VALUES
          (
            'kiro',
            '$2b$12$RRqmON35Z6dfJeC/Y95q0.l0ZiTcQTdctto2IfCwIaYd2TeTXtg2i',
            'kiro@gmail.com',
            'Kyrollos Youssef'
          )
        "
    )
    .execute(&db)
    .await
    .expect("Failed to insert user");

    #[cfg(debug_assertions)]
    send_hotreload_message().await;

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/assets", format!("{site_root}/assets")))
            .service(Files::new("/style", format!("{site_root}/style")))
            .service(favicon)
            .service(pages::login)
            .service(
                web::scope("/api")
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout))
                    .route("/profile_icon", web::get().to(profile::get_profile_icon)),
            )
            .service(
                web::scope("")
                    .service(pages::home)
                    .service(pages::class_schedule)
                    .service(pages::grades)
                    .service(pages::calc_item)
                    .wrap(middleware::from_fn(auth::auth_wrapper)),
            )
            .default_service(web::to(pages::not_found))
            .wrap(middleware::Compress::default())
            .wrap(
                IdentityMiddleware::builder()
                    .visit_deadline(100.weeks().try_into().ok())
                    .build(),
            )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), get_secret())
                    .session_lifecycle(PersistentSession::default().session_ttl(100.weeks()))
                    .cookie_secure(false)
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

/// sends a message to the hotreload server to reload the page
#[cfg(debug_assertions)]
async fn send_hotreload_message() {
    use std::{io::Write, process::Stdio};
    let Ok(addr) = std::env::var("HOTRELOAD_ADDR") else {
        return;
    };

    std::process::Command::new("websocat")
        .arg(addr)
        .arg("-t")
        .arg("-1")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .ok()
        .and_then(|child| child.stdin)
        .and_then(|mut stdin| stdin.write_all(b"reload\n").ok())
        .is_none()
        .then(|| eprintln!("Failed to send hotreload"));
}

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
    tera.register_filter("fmt_location", utils::location_string_filter);
    tera.register_filter("day_to_num", utils::days_to_num_filter);
    tera
});
