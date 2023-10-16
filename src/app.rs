use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::navbar::{Navbar, SideNavbar};

use crate::grades::GradesPage;
use crate::login::{get_user_info, Login, LoginPage, Logout, User};
use crate::profile::ProfilePage;
use crate::registration::RegistrationPage;
use crate::timetable::TimetablePage;

pub type UserResource =
    Resource<(usize, usize), Result<Option<User>, ServerFnError>>;
pub type LogoutAction = Action<Logout, Result<(), ServerFnError>>;

#[component]
pub fn app() -> impl IntoView {
    let login = create_server_action::<Login>();
    let logout = create_server_action::<Logout>();
    let user: UserResource = create_blocking_resource(
        move || (login.version().get(), logout.version().get()),
        move |_| get_user_info(),
    );
    provide_context(user);
    provide_context(logout);
    provide_meta_context();

    let themes = crate::theme::theme_listener();
    provide_context(themes);

    view! {
        <Stylesheet id="leptos" href="/pkg/uni_web.css"/>
        <Title text="Alexandria University"/>
        <crate::theme::ThemeScript/>
        <Body class="flex flex-col bg-gray-50 text-gray-800 dark:bg-gray-950 dark:text-white"/>
        <Router>
            <Routes>
                <Route
                    path="login"
                    view=move || view! { <LoginPage action=login user/> }
                />
                <Route path="/" view=MainWrapper>
                    <Route path="" view=ProfilePage/>
                    <Route path="email" view=move || view! { "email" }/>
                    <Route path="registration" view=RegistrationPage/>
                    <Route path="timetable" view=TimetablePage/>
                    <Route path="financial" view=move || view! { "financial" }/>
                    <Route path="grades" view=GradesPage/>
                    <Route path="profile" view=move || view! { "profile" }/>
                    <Route path="/*any" view=NotFound/>
                </Route>
                <Route path="reset" view=|| "reset" />
            </Routes>
        </Router>
    }
}

#[component]
fn main_wrapper() -> impl IntoView {
    let user = expect_context::<UserResource>();
    // TODO: add bottom margin to main if sidebar is fixed to bottom
    //       add serverside login guard
    view! {
        // login guard
        <Suspense fallback=|| ()>
            <Show when=move || user.with(|u| matches!(u, Some(Ok(None)))) fallback=||()>
                 <Redirect path="/login"/>
            </Show>
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
