use leptos::*;

pub fn _error(e: RwSignal<Errors>) -> impl IntoView {
    view! {
        <div>
        {move || e()
            .iter()
            .map(|(_, e)| e.to_string())
            .collect::<Vec<_>>()
            .join(", ")}
        </div>
    }
}
