/// This is a macro that will inline the contents of an svg file
/// adds the `fill="currentColor"` attribute to all paths
/// adds the `aria-hidden="true"` and `focusable="false"` attributes to the svg
/// # Usage:
/// `icon!(cx, "path/to/icon", ...classes)`
/// ps. path is relative to assets/icons/, `.svg` must be omitted
#[macro_export]
macro_rules! icon {
    ($cx:ident, $icon_name:literal, $cl:expr, $($class:expr),+) => {
        icon!($cx, $icon_name, $($class),+ ).class($cl, true)
    };
    ($cx:ident, $icon_name:literal, $class:expr) => {
        icon!($cx, $icon_name).class($class, true)
    };
    ($cx:ident, $icon_name:literal) => {{
        let icon_svg = include_str!(concat!("../assets/icons/", $icon_name, ".svg"));
        let inner_html = icon_svg
            .replace("path", "path fill=\"currentColor\"")
            .replace(
                "svg",
                "svg aria-hidden=\"true\" focusable=\"false\" role=\"img\" style=\"width:1em\"",
            );
        leptos::leptos_dom::html::span($cx)
            .attr("class", ($cx, "flex"))
            .attr("inner_html", ($cx, inner_html))
    }};
}
