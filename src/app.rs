#![allow(non_snake_case, clippy::needless_lifetimes)]
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::navbar::{Navbar, SideNavbar};
use crate::timetable::TimetablePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // only runs on the client
    #[cfg(feature = "hydrate")]
    crate::theme::theme_event_listener();

    view! {
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
                        <Route path= ""                view=move || view!{"home"}/>
                        <Route path= "email"           view=move || view!{"email"}/>
                        <Route path= "registration"    view=move || view!{"registration"}/>
                        <Route path= "timetable"       view=TimetablePage/>
                        <Route path= "financial"       view=move || view!{"financial"}/>
                        <Route path= "grades"          view=move || view!{"grades"}/>
                        <Route path= "profile"         view=move || view!{"profile"}/>
                        <Route path= "/*any"           view=NotFound/>
                    </Route>
                </Routes>
        </Router>
    }
}

#[component]
fn MainWrapper() -> impl IntoView {
    view! {
        <Navbar/>
        <main class="min-h-[calc(100vh-var(--nav-offset))] flex-grow grid md:grid-cols-[minmax(min-content,_max-content)_auto]">
            <SideNavbar/>
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
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
