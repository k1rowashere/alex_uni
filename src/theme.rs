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

/// A 3-way theme switcher (pill button)
#[component]
pub fn ThemeSwitch() -> impl IntoView {
    let (theme, set_theme) = crate::theme::theme_listener();
    const BUTTON: &str = "switch_button";
    const SEPERATOR: &str = "h-4 w-px bg-gray-500 dark:bg-gray-300";

    view! {
        <div class="text-2xl flex w-min items-center rounded-xl bg-secondary shadow">
            <button
                class="rounded-l-xl ".to_owned() + BUTTON
                data-selected=move || theme() == Theme::Light
                on:click=move |_| set_theme(Theme::Light)
            >
                {icon!("mdi/weather-sunny", "inline-block")}
            </button>
            <div class=SEPERATOR/>
            <button
                class="rounded-none ".to_owned() + BUTTON
                data-selected=move || theme() == Theme::System
                on:click=move |_| set_theme(Theme::System)
            >
                {icon!("mdi/monitor", "inline-block")}
            </button>
            <div class=SEPERATOR/>
            <button
                class="rounded-r-xl ".to_owned() + BUTTON
                data-selected=move || theme() == Theme::Dark
                on:click=move |_| set_theme(Theme::Dark)
            >
                {icon!("mdi/weather-night", "inline-block")}
            </button>
        </div>
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
