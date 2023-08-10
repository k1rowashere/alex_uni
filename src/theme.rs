use leptos::document;
use leptos::window;
#[cfg(feature = "hydrate")]
use wasm_bindgen::{closure::Closure, JsCast};

#[derive(Clone, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl From<&str> for Theme {
    fn from(str: &str) -> Self {
        match str {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            "system" => Theme::System,
            _ => Theme::System,
        }
    }
}

impl From<String> for Theme {
    fn from(str: String) -> Self {
        str.as_str().into()
    }
}

impl From<Theme> for &str {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }
}

pub(crate) fn set_theme(theme: Theme) {
    match theme {
        Theme::Light => set_light_theme(),
        Theme::Dark => set_dark_theme(),
        Theme::System => set_system_theme(),
    };

    window()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item("theme", theme.into())
        .unwrap();
}

pub(crate) fn set_system_theme() {
    let system_theme = window()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap();

    if system_theme.matches() {
        set_dark_theme();
    } else {
        set_light_theme();
    }
}

pub(crate) fn set_light_theme() {
    document()
        .document_element()
        .unwrap()
        .class_list()
        .remove_1("dark")
        .unwrap();
}

pub(crate) fn set_dark_theme() {
    document()
        .document_element()
        .unwrap()
        .class_list()
        .add_1("dark")
        .unwrap();
}

#[cfg(feature = "hydrate")]
pub(crate) fn get_theme_storage() -> Theme {
    let theme = window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("theme")
        .unwrap();

    match theme {
        Some(theme) => theme.into(),
        None => Theme::System,
    }
}

#[cfg(feature = "hydrate")]
pub(crate) fn theme_event_listener() {
    let f = Closure::wrap(Box::new(|e: web_sys::MediaQueryList| {
        let theme = window()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("theme")
            .unwrap();

        if theme.is_none() || theme.unwrap() == "system" {
            if e.matches() {
                set_dark_theme();
            } else {
                set_light_theme();
            }
        }
    }) as Box<dyn FnMut(_)>);

    window()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .add_event_listener_with_callback("change", &f.as_ref().unchecked_ref())
        .unwrap();

    f.forget();
}
