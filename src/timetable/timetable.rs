use leptos::*;

use strum_macros::{Display, EnumString};

use super::Class;

use crate::components::checkbox::Checkbox;
use crate::icon;
use crate::timetable::grid::{TimetableGrid, TimetableGridLoading};
use crate::timetable::list::{TimetableList, TimetableListLoading};
use crate::utils::get_radio_value;

#[derive(PartialEq, Clone, Default, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum View {
    #[default]
    Grid,
    List,
}

#[derive(PartialEq, Clone, Default, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TimeStyle {
    Times,
    Numbers,
    #[default]
    Both,
}

#[derive(PartialEq, Clone, Default)]
pub struct TimetableFlags {
    pub view: View,
    pub time_style: TimeStyle,
    pub show_loc: bool,
    pub show_prof: bool,
    pub show_code: bool,
}

#[server]
async fn get_user_classes() -> Result<Vec<Class>, ServerFnError> {
    // TODO: check for conflicts (handle biweekly courses)
    // TEMP:
    let mut data: Vec<_> = crate::class::data().into();

    data.sort_by(|a, b| {
        let a = (a.day_of_week as usize, a.period.0);
        let b = (b.day_of_week as usize, b.period.0);
        a.cmp(&b)
    });

    Ok(data)
}

// TODO: handle bi-weekly classes
#[component]
pub fn timetable_page() -> impl IntoView {
    let table_data =
        create_resource(|| (), |_| async move { get_user_classes().await });
    let (timetable_settings, flags) = timetable_settings_inner();
    let view = create_memo(move |_| flags().view);

    // FIXME: fix tailwind not grabbing dynamic styles
    // class="z-0 opacity-0 rotate-45"
    // TODO: annotate the settings menu
    view! {
        <div class="relative flex justify-between">
            <h1 class="text-4xl">"Timetable"</h1>
            <TimetableSettings>
                {timetable_settings}
            </TimetableSettings>
        </div>
        <div class="w-auto overflow-x-auto pt-7">
            <Suspense fallback=move || match view() {
                View::List => TimetableListLoading().into_view(),
                View::Grid => TimetableGridLoading().into_view(),
            }>
                {move || {
                    table_data()
                        .map(|res| res.unwrap())
                        .map(|classes| {
                            match view() {
                                View::List => view! { <TimetableList data=classes flags=flags/> },
                                View::Grid => view! { <TimetableGrid data=classes flags=flags/> },
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn timetable_settings(children: Children) -> impl IntoView {
    let (settings_closed, set_settings_closed) = create_signal(true);
    let settings_menu = create_node_ref::<html::Div>();
    let settings_wrapper = create_node_ref::<html::Div>();
    let on_settings_unfocus = move |_| {
        if !settings_wrapper()
            .unwrap()
            .matches(":focus-within")
            .unwrap()
        {
            set_settings_closed(true);
        }
    };
    let on_settings_click = move |_| {
        set_settings_closed.update(|prev| *prev = !*prev);
        settings_menu().unwrap().focus().unwrap()
    };

    view! {
        <div
            node_ref=settings_wrapper
            class="contents"
            on:focusout=on_settings_unfocus
        >
            <button
                id="settings_button"
                class="transition-transform p-1 text-3xl"
                class:rotate-45=settings_closed
                on:click=on_settings_click
            >
                {icon!("mdi/cog")}
            </button>
            <div
                node_ref=settings_menu
                class="z-50 flex max-md:flex-col absolute top-12 p-5 \
                    w-full gap-3 justify-center bg-secondary rounded shadow-lg \
                    transition-all"
                class:opacity-0=settings_closed
                class:invisible=settings_closed
                tabindex="0"
            >
                {children()}
            </div>
        </div>
    }
}

fn timetable_settings_inner() -> (impl IntoView, Signal<TimetableFlags>) {
    use crate::utils::create_query_signal as query;

    let (view, set_view) = query::<View>("view");
    let (time_style, set_time_style) = query::<TimeStyle>("time_style");
    let (show_loc, set_show_loc) = query::<bool>("show_loc");
    let (show_prof, set_show_prof) = query::<bool>("show_prof");
    let (show_code, set_show_code) = query::<bool>("show_code");

    // TODO: save params
    let flags = create_memo(move |_| TimetableFlags {
        view: view().unwrap_or_default(),
        time_style: time_style().unwrap_or_default(),
        show_loc: show_loc().unwrap_or(true),
        show_prof: show_prof().unwrap_or(true),
        show_code: show_code().unwrap_or(true),
    });

    let view = create_memo(move |_| flags().view);
    let time_style = create_memo(move |_| flags().time_style);
    let show_loc = create_memo(move |_| flags().show_loc);
    let show_prof = create_memo(move |_| flags().show_prof);
    let show_code = create_memo(move |_| flags().show_code);

    let on_view_change = move |_| {
        set_view(
            get_radio_value("view").and_then(|v| v.as_str().try_into().ok()),
        )
    };
    let on_time_style_change = move |_| {
        set_time_style(
            get_radio_value("time_style")
                .and_then(|v| v.as_str().try_into().ok()),
        )
    };

    let view = view! {
        <div class="grid grid-cols-[min-content,_1fr] gap-x-2 content-start">
            <Checkbox id="location" getter=show_loc setter=set_show_loc>
                "Display class location"
            </Checkbox>
            <Checkbox id="prof" getter=show_prof setter=set_show_prof>
                "Display class professor"
            </Checkbox>
            <Checkbox id="code" getter=show_code setter=set_show_code>
                "Display class code"
            </Checkbox>
        </div>
        <div class="grid grid-cols-[min-content,_1fr] gap-x-2 content-start">
            <input
                type="radio"
                name="view"
                id="grid"
                value="grid"
                checked=move || matches!(view(), View::Grid)
                on:change=on_view_change
            />
            <label for="grid">"Grid"</label>
            <input
                type="radio"
                name="view"
                id="list"
                value="list"
                checked=move || matches!(view(), View::List)
                on:change=on_view_change
            />
            <label for="list">"List"</label>
        </div>
        <div class="grid grid-cols-[min-content,_1fr] gap-x-2">
            <input
                type="radio"
                name="time_style"
                id="times"
                value="times"
                checked=move || matches!(time_style(), TimeStyle::Times)
                on:change=on_time_style_change
            />
            <label for="times">"Display class times only"</label>
            <input
                type="radio"
                name="time_style"
                id="numbers"
                value="numbers"
                checked=move || matches!(time_style(), TimeStyle::Numbers)
                on:change=on_time_style_change
            />
            <label for="numbers">"Display class period # only"</label>
            <input
                type="radio"
                name="time_style"
                id="both"
                value="both"
                checked=move || matches!(time_style(), TimeStyle::Both)
                on:change=on_time_style_change
            />
            <label for="both">"Display both"</label>
        </div>
    };

    (view, flags.into())
}
