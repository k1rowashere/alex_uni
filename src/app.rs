#![allow(non_snake_case, clippy::needless_lifetimes)]
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::navbar::{Navbar, VerticalNavbar};
use crate::timetable::TimetablePage;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    // only runs on the client
    #[cfg(feature = "hydrate")]
    crate::theme::theme_event_listener();

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/uni_web.css"/>
        <Title text = "Alexandria University"/>
        // TODO: move to a seperate file
        // Blocking javascript to prevent flash of wrong theme on page load
        <Script>
            {r#"
                const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches;
                const theme = localStorage.getItem('theme');
                if (theme === 'dark' || ((theme === null || theme === 'system') && systemTheme)) {
                    const theme = localStorage.getItem('system');
                    document.documentElement.classList.add('dark');
                }
            "#}
        </Script>
        <Body class= "flex flex-col bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-white"/>
        <Router>
                <Routes>
                    <Route path= "/" view=MainWrapper>
                        <Route path= ""                view=move |_cx| view!{_cx, "home"}/>
                        <Route path= "email"           view=move |_cx| view!{_cx, "email"}/>
                        <Route path= "registration"    view=move |_cx| view!{_cx, "registration"}/>
                        <Route path= "timetable"       view=TimetablePage/>
                        <Route path= "financial"       view=move |_cx| view!{_cx, "financial"}/>
                        <Route path= "grades"          view=move |_cx| view!{_cx, "grades"}/>
                        <Route path= "profile"         view=move |_cx| view!{_cx, "profile"}/>
                        <Route path= "/*any"           view=NotFound/>
                    </Route>
                </Routes>
        </Router>
    }
}

#[component]
fn MainWrapper(cx: Scope) -> impl IntoView {
    view! { cx,
        <Navbar/>
        <main class="min-h-[calc(100vh-var(--nav-offset))] flex-grow grid md:grid-cols-[minmax(min-content,_max-content)_auto]">
            <VerticalNavbar/>
            <div class="py-5 px-7 w-auto overflow-x-auto">
                <Outlet/>
            </div>
        </main>
        <footer class="w-screen text-center py-3 max-md:mb-[var(--vert-nav-offset)]">
            <p>"© kirowashere"</p>
        </footer>
    }
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
