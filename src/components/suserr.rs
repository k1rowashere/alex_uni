use leptos::*;
use std::fmt::Write;

/// Wrapper around `Suspense` and `ErrorBoundary`
#[component(transparent)]
pub fn SusErr<S, T, C, IV>(
    children: C,
    resource: Resource<S, Result<T, ServerFnError>>,
) -> impl IntoView
where
    S: 'static + Clone,
    T: 'static,
    C: Fn(&T) -> IV + 'static,
    IV: IntoView + 'static,
{
    let children = store_value(children);
    let format_err = |e: RwSignal<Errors>| {
        e().iter().fold(String::new(), |mut acc, e| {
            // writing to a string never fails
            let _ = writeln!(&mut acc, "{e:#?}");
            acc
        })
    };
    view! {
        <Suspense fallback=move || "loading...">
            <ErrorBoundary fallback=format_err>
                {move || resource.and_then(|resource| children.with_value(|ch| ch(resource)))}
            </ErrorBoundary>
        </Suspense>
    }
}

/// Wrapper around `Transition` and `ErrorBoundary`
#[component(transparent)]
pub fn TransErr<S, T, C, IV>(
    children: C,
    resource: Resource<S, Result<T, ServerFnError>>,
) -> impl IntoView
where
    S: 'static + Clone,
    T: 'static,
    C: Fn(&T) -> IV + 'static,
    IV: IntoView + 'static,
{
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
                {move || resource.and_then(|resource| children.with_value(|ch| ch(resource)))}
            </ErrorBoundary>
        </Transition>
    }
}

/// Wrapper around `Transition` and `ErrorBoundary`
/// This is a utility component that accepts 2 resources
#[component(transparent)]
pub fn TransErrs<S1, T1, S2, T2, C, IV>(
    children: C,
    r1: Resource<S1, Result<T1, ServerFnError>>,
    r2: Resource<S2, Result<T2, ServerFnError>>,
) -> impl IntoView
where
    S1: 'static + Clone,
    S2: 'static + Clone,
    T1: 'static,
    T2: 'static,
    C: Fn(&T1, &T2) -> IV + 'static,
    IV: IntoView + 'static,
{
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
                {move || r1.and_then(|r1| r2.and_then(|r2| children.with_value(|ch| ch(r1, r2))))}
            </ErrorBoundary>
        </Transition>
    }
}
