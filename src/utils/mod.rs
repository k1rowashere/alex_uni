use std::str::FromStr;

use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;

pub trait Unzip<T1, T2, const N: usize> {
    fn unzip(self) -> ([T1; N], [T2; N]);
}

impl<T1, T2, const N: usize> Unzip<T1, T2, N> for [(T1, T2); N] {
    /// https://lib.rs/crates/unzip-array-of-tuple
    /// unzip an array of tuple into a tuple of (two arrays)
    fn unzip(self) -> ([T1; N], [T2; N]) {
        use std::mem::{self, MaybeUninit};

        let mut first: [MaybeUninit<T1>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };
        let mut second: [MaybeUninit<T2>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for (idx, a) in self.into_iter().enumerate() {
            first[idx] = MaybeUninit::new(a.0);
            second[idx] = MaybeUninit::new(a.1);
        }

        // should be safe, as MaybeUninit doesn't have Drop
        unsafe { (mem::transmute_copy(&first), mem::transmute_copy(&second)) }
    }
}

/// same as `leptos_router::create_query_signal` but with NavigateOptions::replace = true
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

    let get = Memo::new({
        let key = key.clone();
        move |_| {
            query_map
                .with(|map| map.get(&key).and_then(|value| value.parse().ok()))
        }
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
            NavigateOptions { replace: true, ..Default::default() },
        );
    });

    (get, set)
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

/// Returns the value of the selected radio input given the group name
pub fn get_radio_value(name: &str) -> Option<String> {
    document()
        .query_selector(&format!("input[name={name}]:checked"))
        .unwrap()
        .map(|el| el.unchecked_into::<web_sys::HtmlInputElement>().value())
}

/// # Usage:
/// `icon!("path/to/icon", "classes")`
/// ps. path is relative to assets/icons/, `.svg` must be omitted
/// TODO: Replace with `leptos-icon`
#[macro_export]
macro_rules! icon {
    ($icon_name:literal, $class:expr) => {
        icon!($icon_name).classes($class)
    };
    ($icon_name:literal) => {{
        const SVG_REPLACE: &str = r#"svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            focusable="false"
            aria-hidden="true"
            role="graphics-symbol"
            style="width: 1em; height: 1em;"
        "#;

        let svg = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/icons/",
            $icon_name,
            ".svg"
        ))
        .replace("svg", SVG_REPLACE);

        leptos::leptos_dom::html::span().inner_html(svg)
    }};
}
