use wasm_bindgen::JsCast;

/// This is a macro that will inline the contents of an svg file
/// adds the `fill="currentColor"` attribute to all paths
/// adds the `aria-hidden="true"` and `focusable="false"` attributes to the svg
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
