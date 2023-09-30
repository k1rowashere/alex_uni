use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::navbar::{Navbar, SideNavbar};

use crate::login::{get_user_info, Login, LoginPage, Logout};
use crate::profile::ProfilePage;
use crate::registration::RegistrationPage;
use crate::timetable::TimetablePage;

pub type UserResource =
    Resource<(usize, usize), Result<Option<String>, ServerFnError>>;
pub type LogoutAction = Action<Logout, Result<(), ServerFnError>>;

#[derive(Copy, Clone)]
struct UserContext(UserResource);

#[component]
pub fn app() -> impl IntoView {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let user = create_blocking_resource(
        move || (login.version().get(), logout.version().get()),
        move |_| get_user_info(),
    );
    let logged_in = move || user.map(|u| matches!(u, Ok(Some(_))));
    provide_context(UserContext(user));
    provide_context(logout);

    provide_meta_context();

    // only runs on the client
    #[cfg(feature = "hydrate")]
    crate::theme::theme_event_listener();

    view! {
        <Stylesheet id="leptos" href="/pkg/uni_web.css"/>
        <Title text="Alexandria University"/>
        <Script>
            // Blocking javascript to prevent flash of wrong theme on page load
            {include_str!("./theme.js")}
        </Script>
        <Body class="flex flex-col bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-white"/>
        <Router>
            <Routes>
                <Route
                    path="login"
                    view=move || view! { <LoginPage action=login logged_in=logged_in/> }
                />
                <Route path="/" view=move || view! { <MainWrapper logged_in=logged_in/> }>
                    <Route path="" view=ProfilePage/>
                    <Route path="email" view=move || view! { "email" }/>
                    <Route path="registration" view=RegistrationPage/>
                    <Route path="timetable" view=TimetablePage/>
                    <Route path="financial" view=move || view! { "financial" }/>
                    <Route path="grades" view=move || view! { "grades" }/>
                    <Route path="profile" view=move || view! { "profile" }/>
                    <Route path="/*any" view=NotFound/>
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
fn main_wrapper<F>(logged_in: F) -> impl IntoView
where
    F: Fn() -> Option<bool> + 'static + Copy,
{
    // TODO: add bottom margin to main if sidebar is fixed to bottom
    view! {
        <Suspense fallback=|| ()>
            {move || {
                if matches!(logged_in(), Some(false)) {
                    view! { <Redirect path="/login"/> }
                } else {
                    ().into_view()
                }
            }}
        </Suspense>
        <Navbar/>
        <main class="bg-inherit min-h-[calc(100vh-var(--nav-offset))] flex-grow grid md:grid-cols-[minmax(min-content,_max-content)_auto]">
            <SideNavbar/>
            <div class="py-5 px-7 mx-auto w-full max-w-[90rem] overflow-x-auto">
                <Outlet/>
            </div>
        </main>
    }
}

#[component]
fn not_found() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
