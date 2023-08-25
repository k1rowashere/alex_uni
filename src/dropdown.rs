#![allow(non_snake_case)]
use leptos::{
    ev::{FocusEvent, MouseEvent},
    html::*,
    *,
};
use wasm_bindgen::JsCast;

#[inline]
pub(crate) fn focus_within(e: FocusEvent) -> bool {
    e.current_target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>()
        .matches(":focus-within")
        .unwrap()
}

#[component]
pub(crate) fn Dropdown<F, IV>(
    #[prop(default = String::new(), into)] class: String,
    button: F,
    children: ChildrenFn,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    const DROPDOWN_CLASS: &str = "absolute top-10 right-0 z-50 min-w-[10em] max-w-[20em] \
                                font-normal text-sm overflow-hidden \
                                bg-white dark:bg-gray-800 rounded-md shadow-lg \
                                border border-gray-200 dark:border-gray-700";
    const BUTTON_CLASS: &str = "text-gray-600 hover:text-gray-800 focus:text-gray-800 \
                                dark:text-gray-400 dark:hover:text-white dark:focus:text-white \
                                text-xl rounded-md";

    let (visible, set_visible) = create_signal(false);
    let list_ref = create_node_ref::<Ul>();

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

    view! {
        <div class="relative flex content-center">
            <button class=class class=BUTTON_CLASS on:click=open_menu>
                {button()}
            </button>
            // <Show when=visible fallback= |_| ()>
            <ul
                ref=list_ref
                class=DROPDOWN_CLASS
                class:hidden=move || !visible()
                tabindex=0
                on:focusout=close_menu
                on:click=close_on_select
            >
                {children()}
            </ul>
        // </Show>
        </div>
    }
}

#[component]
pub(crate) fn DropdownButtonItem<S, F>(
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = false)] separator: bool,
    selected: S,
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
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

    view! {
        <li class="m-0">
            <button class=class aria-selected=selected tabindex=0 on:click=on_click>
                {children()}
            </button>
        </li>
    }
}

#[component]
pub(crate) fn DropdownLinkItem(
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

    view! {<ul><a href=href class=class>{children()}</a></ul> }
}
