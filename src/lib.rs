#![feature(array_try_map)]
#[macro_use]
extern crate macro_rules_attribute;

pub mod app;
mod components;
mod theme;
mod utils;

pub mod class;

#[cfg_attr(not(feature = "login"), path = "login/mock.rs")]
mod login;
mod profile;
mod registration;
mod timetable;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen]
    pub fn hydrate() {
        use crate::app::*;
        use leptos::*;

        console_error_panic_hook::set_once();

        leptos::mount_to_body(move || view! { <App/> });
    }
}
}
