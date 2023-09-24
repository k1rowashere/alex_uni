use leptos::*;
use wasm_bindgen::JsCast;

/// Returns the value of a checkbox input element
fn is_checked(e: web_sys::Event) -> Option<bool> {
    e.target()
        .map(|t| t.unchecked_into::<web_sys::HtmlInputElement>().checked())
}

#[component]
pub fn checkbox(
    id: &'static str,
    #[prop(into)] getter: Signal<bool>,
    setter: SignalSetter<Option<bool>>,
    children: Children,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            id=id
            checked=getter
            on:change=move |e| setter(is_checked(e))
        />
        <label for=id>{children()}</label>
    }
}
