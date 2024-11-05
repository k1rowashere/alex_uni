use actix_identity::Identity;
use actix_web::*;
use http::header;
use web::Bytes;

// TODO:
/// Resoponds with profile image binary read from db
pub async fn get_profile_icon(_: Identity) -> impl Responder {
    let img = Bytes::from_static(include_bytes!("../public/assets/profile.jpg"));

    HttpResponse::Ok()
        .content_type(header::ContentType::jpeg())
        .append_header(header::CacheControl(vec![
            header::CacheDirective::Public,
            header::CacheDirective::MaxAge(3600),
        ]))
        .body(img)
}
