#![allow(non_snake_case)]
use leptos::{ev::MouseEvent, *};

use crate::utils::{append_attributes, unfocus_on_select};

#[component]
pub(crate) fn Dropdown<F, IV>(
    #[prop(default = String::new(), into)] class: String,
    button: F,
    #[prop(into, default = String::new())] label: String,
    #[prop(optional, into)] attributes: Option<MaybeSignal<AdditionalAttributes>>,
    children: ChildrenFn,
) -> impl IntoView
where
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    const DROPDOWN_CLASS: &str = "dropdown_menu";
    const BUTTON_CLASS: &str = "dropdown_menu_button";

    view! {
        <div class="relative flex content-center group">
            {append_attributes(
                view! {
                    <button
                        class=class
                        class=BUTTON_CLASS
                        aria-controls=&label
                        aria-label=&label
                    >
                        {button}
                    </button>
                },
                attributes,
            )}
            <ul id=label class=DROPDOWN_CLASS tabindex=0 on:click=unfocus_on_select>
                {children()}
            </ul>
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
                        aria-selected:text-blue-500";
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
                        aria-selected:text-blue-500 dark:aria-selected:text-blue-500";
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
