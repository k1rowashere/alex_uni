use leptos::*;
use leptos_router::*;

use crate::app::LogoutAction;
use crate::components::dropdown::*;
use crate::icon;
use crate::login::Logout;
use crate::theme::*;
use crate::utils::unfocus_on_select;

#[component]
pub fn Navbar() -> impl IntoView {
    let logout = expect_context::<LogoutAction>();
    const ICON: &str = "mr-2 flex content-center";
    view! {
        <nav class="sticky top-0 z-50 bg-inherit w-screen px-5 py-2 rounded-b flex font-semibold gap-2">
            <A class="font-extrabold text-2xl flex gap-2 flex-grow my-auto" href="/" exact=true>
                <img
                    alt="Alexandria University logo"
                    class="w-8 aspect-[64/83] my-auto"
                    src="assets/alex_logo_min.webp"
                />
                <span>"Alexandria University"</span>
            </A>
            <ThemeDropdown/>
            <Dropdown button=move || icon!("mdi/web", "text-2xl") label="Language Select Dropdown">
                <DropdownLinkItem href="#">
                    <span class=ICON>"🇺🇸"</span>
                    <span>"English"</span>
                </DropdownLinkItem>
                <DropdownLinkItem href="#">
                    <span class=ICON>"🇪🇬"</span>
                    <span>"العربية"</span>
                </DropdownLinkItem>
            </Dropdown>
            <Dropdown button=move || icon!("mdi/account", "text-4xl") label="Profile Menu Dropdown">
                <li class="grid py-2">
                    <span class="px-2 block text-xs text-gray-500 dark:text-gray-400 t-gray-400">
                        "Signed in as"
                    </span>
                    <span class="text-sm mx-4 justify-self-center font-bold">
                        "John Doe dlskfjsdlkfjsdlkfjlksdjf lksdjlkfjsd"
                    </span>
                </li>
                <DropdownLinkItem href="#">
                    {icon!("mdi/form-textbox-password", "mr-2")} "Change Password"
                </DropdownLinkItem>
                <DropdownButtonItem
                    on_click=move |_| logout.dispatch(Logout {})
                    selected=|| false
                    separator=true
                >
                    {icon!("mdi/logout", "mr-2")}
                    "Logout"
                </DropdownButtonItem>
            </Dropdown>
        </nav>
    }
}

#[component]
pub fn SideNavbar() -> impl IntoView {
    const NAV_CLASS: &str = "side_nav";
    const LINK_CLASS: &str = "side_nav__link";
    const LABEL_CLASS: &str = "side_nav__label";

    let (open, set_open) = create_signal(false);
    view! {
        <style>
            " .side_nav:not(.side_nav__open) .side_nav__label { width: 0; }"
        </style>
        <nav
            class=NAV_CLASS
            class:side_nav__open=open
            on:click=unfocus_on_select
        >
            <button class=LINK_CLASS.to_string() + " max-md:hidden" on:click=move |_| set_open.update(|x| *x = !*x)>
                //class="rotate-90"
                <span class="transition-transform" class:rotate-90=open>
                    {icon!("mdi/chevron-right", "text-3xl")}
                </span>
            </button>
            <A class=LINK_CLASS href="/" exact=true>
                {icon!("mdi/id-card", "text-3xl")}
                <span class="whitespace-nowrap ".to_owned() + LABEL_CLASS>"Student Info"</span>
            </A>
            <A class=LINK_CLASS href="/registration">
                {icon!("mdi/file-document-edit-outline", "text-3xl")}
                <span class=LABEL_CLASS>"Course Registration"</span>
            </A>
            <A class=LINK_CLASS href="/timetable">
                {icon!("mdi/timetable", "text-3xl")}
                <span class=LABEL_CLASS>"Study Timetable"</span>
            </A>
            <A class=LINK_CLASS href="/financial">
                {icon!("mdi/cash-multiple", "text-3xl")}
                <span class=LABEL_CLASS>"Financial Status"</span>
            </A>
            <A class=LINK_CLASS href="/grades">
                {icon!("mdi/trophy-outline", "text-3xl")}
                <span class=LABEL_CLASS>"Grades"</span>
            </A>
        </nav>
    }
}
