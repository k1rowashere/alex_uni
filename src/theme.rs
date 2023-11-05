use crate::components::dropdown::*;
use crate::icon;
use leptos::*;
use leptos_use::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

fn set_light_theme() {
    document()
        .document_element()
        .unwrap()
        .class_list()
        .remove_1("dark")
        .unwrap();
}

fn set_dark_theme() {
    document()
        .document_element()
        .unwrap()
        .class_list()
        .add_1("dark")
        .unwrap();
}

pub type ThemeSignal = (Signal<Theme>, WriteSignal<Theme>);

pub fn theme_listener() -> ThemeSignal {
    let prefers_dark = use_media_query("(prefers-color-scheme: dark)");
    let (theme, set_theme, _) =
        storage::use_local_storage("theme", Theme::System);

    create_effect(move |_| match theme() {
        Theme::System => match prefers_dark() {
            true => set_dark_theme(),
            false => set_light_theme(),
        },
        Theme::Light => set_light_theme(),
        Theme::Dark => set_dark_theme(),
    });
    (theme, set_theme)
}

#[component]
pub fn ThemeDropdown() -> impl IntoView {
    let (get_theme, set_theme) = expect_context::<ThemeSignal>();
    let button = move || {
        view! {
            {icon!("mdi/weather-sunny", "text-2xl dark:hidden")
                .class("text-indigo-600", move || get_theme() != Theme::System)
            }
            {icon!("mdi/weather-night", "text-2xl hidden dark:block")
                .class("text-indigo-500", move || get_theme() != Theme::System)
            }
        }
    };
    view! {
        <Dropdown button=button label="Theme Select Dropdown">
            <DropdownButtonItem
                selected=move || get_theme() == Theme::Light
                on_click=move |_| set_theme(Theme::Light)
            >
                {icon!("mdi/weather-sunny", "mr-2")}
                Light
            </DropdownButtonItem>
            <DropdownButtonItem
                selected=move || get_theme() == Theme::Dark
                on_click=move |_| set_theme(Theme::Dark)
            >
                {icon!("mdi/weather-night", "mr-2")}
                Dark
            </DropdownButtonItem>
            <DropdownButtonItem
                selected=move || get_theme() == Theme::System
                on_click=move |_| set_theme(Theme::System)
            >
                {icon!("mdi/monitor", "mr-2")}
                System
            </DropdownButtonItem>
        </Dropdown>
    }
}

/// Script to set the theme based on local storage and system theme
/// This is blocking by design: to avoid a flash of light theme
#[component]
pub fn ThemeScript() -> impl IntoView {
    use leptos_meta::Script;
    const JS: &str = r#"
        const systemTheme = window.matchMedia("(prefers-color-scheme: dark)").matches;
        const theme = localStorage.getItem("theme");
        if (
            theme === '"Dark"' ||
            ((theme === null || theme === '"System"') && systemTheme)
        ) {
            document.documentElement.classList.add("dark");
        }
    "#;
    view! { <Script>{JS}</Script>}
}
