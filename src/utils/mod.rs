pub mod htmx_headers;
use actix_web::cookie::Key;
use std::collections::HashMap;
use tera::Value;

pub fn get_secret() -> Key {
    use base64::{engine::general_purpose::STANDARD, Engine};

    let private_key = get_env("SECRET_KEY");
    let secret = STANDARD
        .decode(private_key.as_bytes())
        .expect("Invalid SECRET");
    assert!(secret.len() >= 32, "Invalid SECRET");

    Key::derive_from(&secret)
}

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("Expected {} to be set", key))
}

pub fn svg_icon_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    match value {
        Value::String(icon_str) => {
            let icon = match icon_str.as_str() {
                "BiHideRegular" => icondata::BiHideRegular,
                "BiShowRegular" => icondata::BiShowRegular,
                "FiMenu" => icondata::FiMenu,
                "FiX" => icondata::FiX,
                "FiCheck" => icondata::FiCheck,
                "BiBookAddRegular" => icondata::BiBookAddRegular,
                "BiHomeAlt2Regular" => icondata::BiHomeAlt2Regular,
                "BiHashRegular" => icondata::BiHashRegular,
                "BsSun" => icondata::BsSun,
                "BiMoonRegular" => icondata::BiMoonRegular,
                "CgDarkMode" => icondata::CgDarkMode,
                "BsChevronRight" => icondata::BsChevronRight,
                "BsChevronDown" => icondata::BsChevronDown,
                _ => icondata::BiSquareRegular,
            };
            let obj = serde_json::to_value(icon).expect("Valid Objects Garanteed");
            assert!(obj.is_object());

            Ok(obj)
        }
        _ => Err("Expected a string".into()),
    }
}
