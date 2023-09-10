use std::str::FromStr;

use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;

pub type UserId = String;

/// same as `leptos_router::create_query_signal` but with NavigateOptions::replace = true
/// imo this is a more sensible default
pub fn create_query_signal<T>(
    key: impl Into<Oco<'static, str>>,
) -> (Memo<Option<T>>, SignalSetter<Option<T>>)
where
    T: FromStr + ToString + PartialEq,
{
    let key = key.into();
    let query_map = use_query_map();
    let navigate = use_navigate();
    let location = use_location();

    let get = create_memo({
        let key = key.clone();
        move |_| query_map.with(|map| map.get(&key).and_then(|value| value.parse().ok()))
    });

    let set = SignalSetter::map(move |value: Option<T>| {
        let mut new_query_map = query_map.get();
        match value {
            Some(value) => {
                new_query_map.insert(key.to_string(), value.to_string());
            }
            None => {
                new_query_map.remove(&key);
            }
        }
        let qs = new_query_map.to_query_string();
        let path = location.pathname.get();
        let new_url = format!("{path}{qs}");
        navigate(
            &new_url,
            NavigateOptions {
                replace: true,
                ..Default::default()
            },
        );
    });

    (get, set)
}

/// This is a macro that will inline the contents of an svg file.
/// Adds useful attributes to svg container
/// # Usage:
/// `icon!("path/to/icon", ...classes)`
/// ps. path is relative to assets/icons/, `.svg` must be omitted
#[macro_export]
macro_rules! icon {
    ($icon_name:literal, $cl:expr, $($class:expr),+) => {
        icon!($icon_name, $($class),+ ).class($cl, true)
    };
    ($icon_name:literal, $class:expr) => {
        icon!($icon_name).class($class, true)
    };
    ($icon_name:literal) => {{
        let icon_svg = include_str!(concat!("../assets/icons/", $icon_name, ".svg"));
        let inner_html = icon_svg
            .replace("path", "path fill=\"currentColor\"")
            .replace(
                "svg",
                "svg aria-hidden=\"true\" focusable=\"false\" role=\"img\" style=\"width:1em\"",
            );
        leptos::leptos_dom::html::span()
            .attr("class", ("flex"))
            .attr("inner_html", (inner_html))
    }};
}

/// Blurs this element upon click of a child button or anchor elements
pub fn unfocus_on_select(e: web_sys::MouseEvent) {
    let el = e
        .target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>()
        .closest("button, a")
        .unwrap();

    if let Some(el) = el {
        el.unchecked_into::<web_sys::HtmlElement>().blur().unwrap()
    }
}

/// Appends a list of arbitrary attributes to an HTML element
pub fn append_attributes<T>(
    mut element: HtmlElement<T>,
    attributes: Option<MaybeSignal<AdditionalAttributes>>,
) -> HtmlElement<T>
where
    T: leptos::html::ElementDescriptor + 'static,
{
    if let Some(attributes) = attributes {
        let attributes = attributes.get();
        for (attr_name, attr_value) in attributes.into_iter() {
            let attr_name = attr_name.to_owned();
            let attr_value = attr_value.to_owned();
            element = element.attr(attr_name, move || attr_value.get());
        }
    }
    element
}
