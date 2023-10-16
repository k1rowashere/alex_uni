#[macro_use]
extern crate macro_rules_attribute;

pub mod app;
mod components;
mod theme;
mod utils;

mod class;

mod grades;
mod login;
mod profile;
pub mod registration;
mod timetable;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(crate::app::App);
}
