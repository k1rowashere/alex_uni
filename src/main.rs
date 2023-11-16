#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use uni_web::app::*;

    let _ = dotenvy::dotenv();

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);
    let pool = sqlx::SqlitePool::connect(
        std::env::var("DATABASE_URL")
            .expect("Missing DATABASE_URL")
            .as_str(),
    )
    .await
    .expect("Failed to connect to DB");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run sqlx migrations");

    HttpServer::new(move || {
        use uni_web::registration::rem_seats_ws::rem_seats_ws;

        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .route("/ws/rem_seats", web::get().to(rem_seats_ws))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root))
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    use actix_files::NamedFile;

    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(NamedFile::open(format!("{site_root}/favicon.webp"))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {}
