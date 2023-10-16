use leptos::*;
use std::fmt::Write;

/// Standard wrapper around `Suspense` and `ErrorBoundary`
#[component(transparent)]
pub fn sus_err(children: ChildrenFn) -> impl IntoView {
    let children = store_value(children);
    let format_err = |e: RwSignal<Errors>| {
        e().iter().fold(String::new(), |mut acc, e| {
            // writing to a string never fails
            let _ = writeln!(&mut acc, "{e:#?}");
            acc
        })
    };
    view! {
        <Transition fallback=move || "loading...">
            <ErrorBoundary fallback=format_err>
                {children.with_value(|ch| ch())}
            </ErrorBoundary>
        </Transition>
    }
}
