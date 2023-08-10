use leptos::{
    ev::{FocusEvent, MouseEvent},
    html::Ul,
    *,
};
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::JsCast;

use crate::theme::*;

/// This is a macro that will inline the contents of an svg file
/// adds the `fill="currentColor"` attribute to all paths
/// adds the `aria-hidden="true"` and `focusable="false"` attributes to the svg
macro_rules! icon {
    ($cx:ident, $icon_name:literal, $cl:expr, $($class:expr),+) => {
        icon!($cx, $icon_name, $($class),+ ).class($cl, true)
    };
    ($cx:ident, $icon_name:literal, $class:expr) => {
        icon!($cx, $icon_name).class($class, true)
    };
    ($cx:ident, $icon_name:literal) => {{
        let icon_svg = include_str!(concat!("../assets/icons/", $icon_name, ".svg"));
        let inner_html = icon_svg
            .replace("path", "path fill=\"currentColor\"")
            .replace(
                "svg",
                "svg aria-hidden=\"true\" focusable=\"false\" role=\"img\" style=\"width:1em\"",
            );
        leptos::leptos_dom::html::span($cx)
            .attr("class", ($cx, "flex"))
            .attr("inner_html", ($cx, inner_html))
    }};
}

#[component]
fn ThemeDropdown(cx: Scope) -> impl IntoView {
    let (theme, set_theme) = create_signal(cx, Theme::Light);

    // on the client, get the theme from local storage
    #[cfg(feature = "hydrate")]
    set_theme(get_theme_storage());

    create_effect(cx, move |_| crate::theme::set_theme(theme()));

    let button = move |cx: Scope| {
        view! {cx,
            {
                icon!(cx, "mdi/weather-sunny", "text-2xl", "dark:hidden")
                    .class("text-blue-600", (cx, move || theme() != Theme::System))
            }
            {
                icon!(cx, "mdi/weather-night", "text-2xl", "hidden", "dark:block")
                    .class("text-blue-400", (cx, move || theme() != Theme::System))
            }
        }
    };
    view! {cx,
        <Dropdown button=button>
            <DropdownButtonItem
                selected=move || theme() == Theme::Light
                on_click=move |_| set_theme(Theme::Light)
            >
                { icon!(cx, "mdi/weather-sunny", "mr-2") }
                Light
            </DropdownButtonItem>
            <DropdownButtonItem
                selected=move || theme() == Theme::Dark
                on_click=move |_| set_theme(Theme::Dark)
            >
                { icon!(cx, "mdi/weather-night", "mr-2" ) }
                Dark
            </DropdownButtonItem>
            <DropdownButtonItem
                selected=move || theme() == Theme::System
                on_click=move |_| set_theme(Theme::System)
            >
                { icon!(cx, "mdi/monitor", "mr-2") }
                System
            </DropdownButtonItem>
        </Dropdown>
    }
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SubjectType {
    Lecture,
    Lab,
    Tutorial,
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SubjectLocation {
    building: String,
    floor: u8,
    room: String,
}

#[derive(Clone, Hash, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DayOfWeek {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl From<usize> for DayOfWeek {
    fn from(value: usize) -> Self {
        match value {
            0 => DayOfWeek::Saturday,
            1 => DayOfWeek::Sunday,
            2 => DayOfWeek::Monday,
            3 => DayOfWeek::Tuesday,
            4 => DayOfWeek::Wednesday,
            5 => DayOfWeek::Thursday,
            6 => DayOfWeek::Friday,
            _ => unreachable!(),
        }
    }
}
impl ToString for DayOfWeek {
    fn to_string(&self) -> String {
        match self {
            DayOfWeek::Saturday => "Saturday",
            DayOfWeek::Sunday => "Sunday",
            DayOfWeek::Monday => "Monday",
            DayOfWeek::Tuesday => "Tuesday",
            DayOfWeek::Wednesday => "Wednesday",
            DayOfWeek::Thursday => "Thursday",
            DayOfWeek::Friday => "Friday",
        }
        .to_string()
    }
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    type_: SubjectType,
    name: String,
    prof: String,
    location: SubjectLocation,
    day_of_week: DayOfWeek,
    /// inclusive range, 0-indexed
    period: (usize, usize),
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SubjectOption {
    None,
    JoinMark,
    Some(Subject),
}

#[server(GetTimetableSubjects, "api", "GetJson", "GetTimetableSubjects")]
async fn get_timetable_subjects() -> Result<[[SubjectOption; 12]; 7], ServerFnError> {
    let subjects = vec![
        Subject {
            type_: SubjectType::Lecture,
            name: "CS 101".to_string(),
            prof: "Dr. John Doe".to_string(),
            location: SubjectLocation {
                building: "E1".to_string(),
                floor: 1,
                room: "101".to_string(),
            },
            day_of_week: DayOfWeek::Monday,
            period: (1, 3),
        },
        Subject {
            type_: SubjectType::Lecture,
            name: "CS 101".to_string(),
            prof: "Dr. John Doe".to_string(),
            location: SubjectLocation {
                building: "E1".to_string(),
                floor: 1,
                room: "101".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (0, 0),
        },
    ];

    let mut timetable: [[SubjectOption; 12]; 7] =
        std::array::from_fn(|_| std::array::from_fn(|_| SubjectOption::None));
    for s in subjects {
        timetable[s.day_of_week as usize][s.period.0] = SubjectOption::Some(s.clone());
        for i in s.period.0 + 1..=s.period.1 {
            timetable[s.day_of_week as usize][i] = SubjectOption::JoinMark;
        }
    }
    Ok(timetable)
}

#[component]
fn TimetableItem<'a>(cx: Scope, item: &'a SubjectOption) -> impl IntoView {
    match item {
        SubjectOption::None => view! {cx, <td/>},
        SubjectOption::JoinMark => view! {cx, <td class= "hidden"/>},
        SubjectOption::Some(s) => view! {cx,
            <td colspan=s.period.1 - s.period.0 + 1>
                {s.name.clone()}
            </td>
        },
    }
}

#[component]
fn TimetableRow<'a>(cx: Scope, row: &'a [SubjectOption; 12], day: DayOfWeek) -> impl IntoView {
    view! {cx,
        <tr>
            <th>{day.to_string()}</th>
            {
                row.iter()
                    .map(|s| view!{cx, <TimetableItem item=s/>})
                    .collect_view(cx)
            }
        </tr>
    }
}

fn timetable_periods() -> impl Iterator<Item = String> {
    const T0: (u32, u32) = (8, 30);
    const DT: u32 = 50;
    const BREAK: u32 = 10;
    (0..12).map(|i| {
        // each period is 50 mins, and a 10 mins break after 2 periods
        let dt = i * DT + (i / 2) * BREAK;
        let mut t1 = (T0.0, T0.1 + dt);
        t1 = (t1.0 + t1.1 / 60, t1.1 % 60);
        format!("{:02}:{:02}", (t1.0 - 1) % 12 + 1, t1.1)
    })
}

// timetable_period_times, but const
const fn timetable_periods_const() -> [&'static str; 12] {
    [
        "08:30", "09:20", "10:20", "11:10", "12:10", "13:00", "14:00", "14:50", "15:50", "16:40",
        "17:40", "18:30",
    ]
}

#[component]
fn Timetable(cx: Scope) -> impl IntoView {
    let table = create_resource(cx, || (), |_| async move { get_timetable_subjects().await });
    view! {cx,
    <h1>Timetable</h1>
    <Suspense fallback= move || view! {cx, <p>"loading"</p>}>
        <table class= "w-full">
            <thead>
                <tr>
                    <td/>
                    {(1..=12).map(|i| view!{cx, <th>{i}</th>}).collect_view(cx)}
                </tr>
                <tr>
                    <td/>
                    {timetable_periods_const().map(|t| view!{cx, <th>{t}</th>}).collect_view(cx)}
                </tr>
            </thead>
            <tbody>
                { move || {
                    table.read(cx).map(|table| {
                        table.unwrap().iter().enumerate()
                            .map(|(i, row)| view!{cx, <TimetableRow row=row day=i.into()/>})
                            .collect_view(cx)
                    })
                }}
            </tbody>
        </table>
    </Suspense>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    // only runs on the client
    #[cfg(feature = "hydrate")]
    theme_event_listener();

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
            <Navbar/>
            <main class= "flex-grow grid min-h-[calc(100vh-var(--nav-offset))] \
                grid-cols-[minmax(min-content,_max-content)_auto]"
            >
                <VerticalNavbar/>
                <Routes>
                    <Route path= "/" view=MainWrapper>
                        <Route path= ""                view=move |_cx| view!{_cx, "home"}/>
                        <Route path= "email"           view=move |_cx| view!{_cx, "email"}/>
                        <Route path= "registration"    view=move |_cx| view!{_cx, "registration"}/>
                        <Route path= "timetable"       view=Timetable/>
                        <Route path= "financial"       view=move |_cx| view!{_cx, "financial"}/>
                        <Route path= "grades"          view=move |_cx| view!{_cx, "grades"}/>
                        <Route path= "profile"         view=move |_cx| view!{_cx, "profile"}/>
                        <Route path= "/*any"           view=NotFound/>
                    </Route>
                </Routes>
            </main>
            <footer class= "w-screen text-center py-3">
                <p>"© kirowashere"</p>
            </footer>
        </Router>
    }
}

#[component]
fn MainWrapper(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class= "py-5 px-7">
            <Outlet/>
        </div>
    }
}

#[component]
fn VerticalNavbar(cx: Scope) -> impl IntoView {
    // let (open, set_open) = create_signal(cx, false);
    const LINK_CLASS: &str = "h-12 flex items-center overflow-hidden \
                        rounded-l pl-1 \
                        hover:bg-gray-200 focus:bg-gray-200 \
                        dark:hover:bg-gray-600 dark:focus:bg-gray-600 focus:outline-none";

    const ACTIVE_CLASS: &str = "text-blue-800 dark:text-blue-400";
    // TODO: why do wasm_bindgen be like that
    let unfocus = move |e: MouseEvent| {
        let a = e
            .target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlElement>()
            .closest("a")
            .unwrap();
        match a {
            Some(a) => a.unchecked_into::<web_sys::HtmlElement>().blur().unwrap(),
            None => (),
        }
    };

    // const SEP_CLASS: &str = "mt-[calc(theme(spacing.2)+1px)] relative before:absolute \
    //                     before:bottom-full before:mb-1 before:inset-x-0 before:h-px \
    //                     before:bg-gray-100 dark:before:bg-gray-600/30 \
    //                     before:pointer-events-none";
    view! {cx,
    <nav class= "sticky overflow-y-auto py-5 pl-1 \
                    grid content-start gap-6 top-[var(--nav-offset)] \
                    max-h-[calc(100vh-var(--nav-offset))] \
                    border-r-2 border-gray-200 dark:border-gray-600 border-opacity-25 \
                    vert_nav"
        on:click = unfocus
    >
        // <button class= "flex items-center"
        //     on:click = move |_| set_open.update(|x| *x = !*x)
        // >
        //     {icon!(cx, "mdi/menu", "mx-2 text-4xl")}
        // </button>
        // <A class=LINK_CLASS href="/email" active_class=ACTIVE_CLASS>
        //     {icon!(cx, "mdi/email-open-multiple-outline", "mr-2", "text-3xl")}
        //     <span class="vert_nav__label">Email</span>
        // </A>
        <A class=LINK_CLASS href="/registration" active_class=ACTIVE_CLASS>
            {icon!(cx, "mdi/file-document-edit-outline", "mr-2", "text-3xl")}
            <span class="vert_nav__label">Course Registration</span>
        </A>
        <A class=LINK_CLASS href="/timetable" active_class=ACTIVE_CLASS>
            {icon!(cx, "mdi/timetable", "mr-2", "text-3xl")}
            <span class="vert_nav__label">Study Timetable</span>
        </A>
        <A class=LINK_CLASS href="/financial" active_class=ACTIVE_CLASS>
            {icon!(cx, "mdi/cash-multiple", "mr-2", "text-3xl")}
            <span class="vert_nav__label">Financial Status</span>
        </A>
        <A class=LINK_CLASS href="/grades" active_class=ACTIVE_CLASS>
            {icon!(cx, "mdi/trophy-outline", "mr-2", "text-3xl")}
            <span class="vert_nav__label">Grades</span>
        </A>
    </nav>
    }
}

#[component]
fn Navbar(cx: Scope) -> impl IntoView {
    const ICON: &str = "mr-2 flex content-center";
    view! { cx,
    <nav class= "sticky top-0 z-50 bg-inherit w-screen px-5 py-2 rounded-b flex font-semibold gap-2">
        <img class= "w-8 aspect-[64/83] my-auto" src = "assets/alex_logo_min.webp"/>
        <a class= "font-extrabold text-2xl flex-grow my-auto">Alexandria University</a>
        <ThemeDropdown/>
        <Dropdown button=move |cx| icon!(cx, "mdi/web", "text-2xl")>
            <DropdownLinkItem href="#">
                <span class=ICON>"🇺🇸"</span>
                English
            </DropdownLinkItem>
            <DropdownLinkItem href="#">
                <span class=ICON>"🇪🇬"</span>
                Arabic
            </DropdownLinkItem>
        </Dropdown>
        <Dropdown button = move |cx| icon!(cx, "mdi/account", "text-4xl")>
            <li class="grid py-2">
                <span class= "px-2 block text-xs text-gray-500 dark:text-gray-400">Signed in as:</span>
                <span class= "text-sm mx-4 justify-self-center font-bold">
                    John Doe dlskfjsdlkfjsdlkfjlksdjf lksdjlkfjsdlkflsjlfsdjl
                </span>
            </li>
            <DropdownLinkItem href="#" separator=true>
                { icon!(cx, "mdi/card-account-details-outline", "mr-2") }
                My Profile
            </DropdownLinkItem>
            <DropdownLinkItem href="#">
                { icon!(cx, "mdi/form-textbox-password", "mr-2") }
                Change Password
            </DropdownLinkItem>
            <DropdownLinkItem href="#" separator=true>
                { icon!(cx, "mdi/logout", "mr-2") }
                Logout
            </DropdownLinkItem>
        </Dropdown>
    </nav>
    }
}

#[inline]
fn focus_within(e: FocusEvent) -> bool {
    e.current_target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>()
        .matches(":focus-within")
        .unwrap()
}

#[component]
fn Dropdown<F, IV>(
    cx: Scope,
    #[prop(default = String::new(), into)] class: String,
    button: F,
    children: ChildrenFn,
) -> impl IntoView
where
    F: Fn(Scope) -> IV + 'static,
    IV: IntoView,
{
    const DROPDOWN_CLASS: &str = "absolute top-10 right-0 z-50 min-w-[10em] max-w-[20em] \
                                font-normal text-sm overflow-hidden \
                                bg-white dark:bg-gray-800 rounded-md shadow-lg \
                                border border-gray-200 dark:border-gray-700";
    const BUTTON_CLASS: &str = "text-gray-600 hover:text-gray-800 focus:text-gray-800 \
                                dark:text-gray-400 dark:hover:text-white dark:focus:text-white \
                                text-xl rounded-md";

    let (visible, set_visible) = create_signal(cx, false);
    let list_ref = create_node_ref::<Ul>(cx);

    let open_menu = move |_| {
        set_visible(true);
        list_ref.get().unwrap().focus().unwrap();
    };
    let close_menu = move |e| {
        if !focus_within(e) {
            set_visible(false);
        }
    };

    let close_on_select = move |e: MouseEvent| {
        let clicked = e
            .target()
            .unwrap()
            .unchecked_into::<web_sys::Element>()
            .tag_name();

        match clicked.as_str() {
            "BUTTON" | "A" => set_visible(false),
            _ => (),
        }
    };

    view! {cx,
    <div class="relative flex content-center">
        <button class=class class=BUTTON_CLASS on:click=open_menu>{button(cx)}</button>
        // <Show when=visible fallback= |_| ()>
            <ul ref = list_ref
                class= DROPDOWN_CLASS
                class:hidden = move || !visible()
                tabindex = 0
                on:focusout = close_menu
                on:click = close_on_select
            >
                {children(cx)}
            </ul>
        // </Show>
    </div>
    }
}

#[component]
fn DropdownButtonItem<S, F>(
    cx: Scope,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = false)] separator: bool,
    selected: S,
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) -> () + 'static,
    S: Fn() -> bool + 'static,
{
    const ITEM: &str = "w-full flex px-4 py-2 hover:bg-gray-200 focus:bg-gray-200 \
                        dark:hover:bg-gray-600 dark:focus:bg-gray-600 focus:outline-none \
                        aria-selected:text-blue-800 dark:aria-selected:text-blue-400";
    const TOP_SEPARATOR: &str = "mt-[calc(theme(spacing.2)+1px)] relative before:absolute \
                        before:bottom-full before:mb-1 before:inset-x-0 before:h-px \
                        before:bg-gray-100 dark:before:bg-gray-600/30 \
                        before:pointer-events-none";

    let class = format!(
        "{} {} {}",
        class,
        ITEM,
        if separator { TOP_SEPARATOR } else { "" }
    );
    let selected = move || selected().to_string();

    view! {cx,
    <li class="m-0">
        <button class=class aria-selected=selected tabindex=0 on:click=on_click>
            {children(cx)}
        </button>
    </li>
    }
}

#[component]
fn DropdownLinkItem(
    cx: Scope,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = false)] separator: bool,
    #[prop(into)] href: String,
    children: Children,
) -> impl IntoView {
    const ITEM: &str = "w-full flex text-left px-4 py-2 hover:bg-gray-200 focus:bg-gray-200 \
                        dark:hover:bg-gray-600 dark:focus:bg-gray-600 focus:outline-none \
                        aria-selected:text-blue-800 dark:aria-selected:text-blue-400";
    const TOP_SEPARATOR: &str = "mt-[calc(theme(spacing.2)+1px)] relative before:absolute \
                        before:bottom-full before:mb-1 before:inset-x-0 before:h-px \
                        before:bg-gray-100 dark:before:bg-gray-600/30 \
                        before:pointer-events-none";

    let class = format!(
        "{} {} {}",
        class,
        ITEM,
        if separator { TOP_SEPARATOR } else { "" }
    );

    view! {cx, <ul><a href=href class=class>{children(cx)}</a></ul> }
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
