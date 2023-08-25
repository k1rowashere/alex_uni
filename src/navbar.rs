use leptos::ev::MouseEvent;
use leptos::*;
use leptos_router::*;

use wasm_bindgen::JsCast;

use crate::dropdown::*;
use crate::icon;
use crate::theme::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    const ICON: &str = "mr-2 flex content-center";
    view! { cx,
        <nav class="sticky top-0 z-50 bg-inherit w-screen px-5 py-2 rounded-b flex font-semibold gap-2">
            <img class="w-8 aspect-[64/83] my-auto" src="assets/alex_logo_min.webp"/>
            <a class="font-extrabold text-2xl flex-grow my-auto">
                Alexandria University
            </a>
            <ThemeDropdown/>
            <Dropdown button=move |cx| icon!(cx, "mdi/web", "text-2xl")>
                <DropdownLinkItem href="#">
                    <span class=ICON>"🇺🇸"</span>
                          E
                </DropdownLinkItem>
                <DropdownLinkItem href="#">
                    <span class=ICON>"🇪🇬"</span>

                </DropdownLinkItem>
            </Dropdown>
            <Dropdown button=move |cx| icon!(cx, "mdi/account", "text-4xl")>
                <li class="grid py-2">
                    <span class="px-2 block text-xs text-gray-500 dark:text-gray-400 t-gray-400">
                        "Signed in as"
                    </span>
                    <span class="text-sm mx-4 justify-self-center font-bold">
                        "John Doe dlskfjsdlkfjsdlkfjlksdjf lksdjlkfjsd"
                    </span>
                </li>
                <DropdownLinkItem href="#" separator=true>
                    {icon!(cx, "mdi/card-account-details-outline", "mr-2")}
                    "Profile"
                </DropdownLinkItem>
                <DropdownLinkItem href="#">
                    {icon!(cx, "mdi/form-textbox-password", "mr-2")}
                    "Change Password"
                </DropdownLinkItem>
                <DropdownLinkItem href="#" separator=true>
                    {icon!(cx, "mdi/logout", "mr-2")}
                    "Logout"
                </DropdownLinkItem>
            </Dropdown>
        </nav>
    }
}

#[component]
pub fn VerticalNavbar(cx: Scope) -> impl IntoView {
    const LINK_CLASS: &str = "h-12 flex items-center overflow-hidden rounded-l pl-1 \
                        hover:bg-gray-200 focus:bg-gray-200 dark:hover:bg-gray-600 dark:focus:bg-gray-600 focus:outline-none \
                        max-md:rounded max-md:p-2 max-sm:justify-center";

    const ACTIVE_CLASS: &str = "text-blue-800 dark:text-blue-400";

    const LABEL_CLASS: &str = "ml-2 vert_nav__label max-sm:hidden";

    // TODO: why do wasm_bindgen be like that
    let unfocus = move |e: MouseEvent| {
        let el = e
            .target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlElement>()
            .closest("button, a")
            .unwrap();
        if let Some(el) = el {
            el.unchecked_into::<web_sys::HtmlElement>().blur().unwrap()
        }
    };

    let (open, set_open) = create_signal(cx, false);
    view! { cx,
        <nav
            class="md:sticky md:py-5 md:pl-1 md:content-start md:top-[var(--nav-offset)] md:max-h-[calc(100vh-var(--nav-offset))] md:border-r-2 \
                max-md:fixed max-md:bottom-0 max-md:py-3 max-md:grid-cols-4 max-md:justify-between max-md:border-t-2 max-md:w-screen \
                gap-6 border-opacity-25 vert_nav grid overflow-y-auto border-gray-200 dark:border-gray-600 bg-inherit"
            class:vert_nav__open=open
            on:click=unfocus
        >
            <button class=LINK_CLASS.to_string() + " max-md:hidden" on:click=move |_| set_open.update(|x| *x = !*x)>
                //class="rotate-90"
                <span class="transition-all" class:rotate-90=open>
                    {icon!(cx, "mdi/chevron-right", "text-3xl")}
                </span>
            </button>
            <A class=LINK_CLASS href="/registration" active_class=ACTIVE_CLASS>
                {icon!(cx, "mdi/file-document-edit-outline", "text-3xl")}
                <span class=LABEL_CLASS>"Course Registration"</span>
            </A>
            <A class=LINK_CLASS href="/timetable" active_class=ACTIVE_CLASS>
                {icon!(cx, "mdi/timetable", "text-3xl")}
                <span class=LABEL_CLASS>"Study Timetable"</span>
            </A>
            <A class=LINK_CLASS href="/financial" active_class=ACTIVE_CLASS>
                {icon!(cx, "mdi/cash-multiple", "text-3xl")}
                <span class=LABEL_CLASS>"Financial Status"</span>
            </A>
            <A class=LINK_CLASS href="/grades" active_class=ACTIVE_CLASS>
                {icon!(cx, "mdi/trophy-outline", "text-3xl")}
                <span class=LABEL_CLASS>"Grades"</span>
            </A>
        </nav>
    }
}
