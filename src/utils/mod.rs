pub mod htmx_headers;
use actix_web::cookie::Key;
use std::collections::HashMap;
use tera::Value;

use crate::model::class::Location;

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

pub fn days_to_num_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    match value.as_str() {
        Some("Saturday") => Ok(0.into()),
        Some("Sunday") => Ok(1.into()),
        Some("Monday") => Ok(2.into()),
        Some("Tuesday") => Ok(3.into()),
        Some("Wednesday") => Ok(4.into()),
        Some("Thursday") => Ok(5.into()),
        Some("Friday") => Ok(6.into()),
        _ => Err("not a string/invalid day".into()),
    }
}

pub fn location_string_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    serde_json::from_value(value.clone())
        .map(|location: Location| format!("{location}").into())
        .map_err(|e| e.into())
}

pub fn svg_icon_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    match value {
        Value::String(icon_str) => {
            let icon = match icon_str.as_str() {
                "FiPlus" => icondata::FiPlus,
                "BsCalculator" => icondata::BsCalculator,
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
                "BsArrowRight" => icondata::BsArrowRight,
                "TbCalendarTime" => icondata::TbCalendarTime,
                "LuPenSquare" => icondata::LuPenSquare,
                "BiMedalRegular" => icondata::BiMedalRegular,
                _ => icondata::BiSquareRegular,
            };
            let obj = serde_json::to_value(icon).expect("Valid Objects Garanteed");
            assert!(obj.is_object());

            Ok(obj)
        }
        _ => Err("Expected a string".into()),
    }
}
