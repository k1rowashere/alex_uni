use leptos::*;

use strum_macros::{Display, EnumString};
use wasm_bindgen::JsCast;

use super::Class;
use crate::icon;
use crate::timetable::grid::{TimetableGrid, TimetableGridLoading};
use crate::timetable::list::{TimetableList, TimetableListLoading};
use crate::utils::is_checked;

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

#[derive(Clone)]
pub struct TimetableFlags {
    pub view: Memo<View>,            // Table style
    pub time_style: Memo<TimeStyle>, // Period times/numbers
    pub show_loc: Memo<bool>,        // Location visibilty
    pub show_prof: Memo<bool>,       // Prof name visibilty
    pub show_code: Memo<bool>,       // Subject code visibilty
}

#[server]
async fn get_user_classes() -> Result<Vec<Class>, ServerFnError> {
    use super::{Building, ClassKind, ClassLocation, DayOfWeek};
    // TEMP:
    let mut data = vec![
        Class {
            kind: ClassKind::Lab,
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 0,
                room: "Lab 7".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (4, 5),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 7,
                room: "Class 72".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (6, 6),
        },
        Class {
            kind: ClassKind::Lab,
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 3,
                room: "Lab Circuits".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (7, 7),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            prof: Some("أ.د. مجدي عبد العظيم".to_string()),
            location: ClassLocation {
                building: Building::PreparatorySouth,
                floor: 0,
                room: "L3".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (8, 9),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            prof: Some("أ.د.م. مروان تركي".to_string()),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 2,
                room: "C39".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (10, 11),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 7,
                room: "Class 701".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (2, 3),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "EMP x19".to_string(),
            name: "Probability and Statistics".to_string(),
            prof: Some("د. ميرفت ميخائيل".to_string()),
            location: ClassLocation {
                building: Building::Electricity,
                floor: 0,
                room: "Class 103".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (5, 7),
        },
        Class {
            kind: ClassKind::Lab,
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 5,
                room: "Lab Logic".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (8, 9),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            prof: Some("د. عادل الفحار".to_string()),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 1,
                room: "C26".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (4, 5),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Ssp,
                floor: 2,
                room: "C44".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (6, 6),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "EMP x19".to_string(),
            name: "Probability and Statistics".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Ssp,
                floor: 0,
                room: "C11".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (7, 7),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "TRN x21".to_string(),
            name: "Technical Writing".to_string(),
            prof: Some("أ.د.م. أحمد التراس".to_string()),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 2,
                room: "C40".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (8, 9),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "EMP 116".to_string(),
            name: "Differential Equations".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::PreparatorySouth,
                floor: 1,
                room: "C8".to_string(),
            },
            day_of_week: DayOfWeek::Wednesday,
            period: (0, 0),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "EMP 116".to_string(),
            name: "Differential Equations".to_string(),
            prof: Some("أ.د. عمرو عبد الرازاق".to_string()),
            location: ClassLocation {
                building: Building::PreparatoryNorth,
                floor: 0,
                room: "L5".to_string(),
            },
            day_of_week: DayOfWeek::Wednesday,
            period: (3, 5),
        },
    ];

    // TODO: check for conflicts (handle biweekly courses)

    data.sort_by(|a, b| {
        let a = (a.day_of_week as usize, a.period.0);
        let b = (b.day_of_week as usize, b.period.0);
        a.cmp(&b)
    });

    Ok(data)
}

#[component]
pub fn TimetablePage() -> impl IntoView {
    let table_data = create_resource(|| (), |_| async move { get_user_classes().await });
    let (timetable_settings, flags) = timetable_settings();
    let flags = store_value(flags);
    let (closed, set_closed) = create_signal(true);

    // FIXME: fix tailwind not grabbing dynamic styles
    // class="z-0 opacity-0 rotate-45"
    // TODO: annotate the settings menu
    view! {
        <div class="relative flex justify-between">
            <h1 class="text-4xl">"Timetable"</h1>
            <button
                class="transition-transform p-1 text-3xl"
                class:rotate-45=closed
                on:click=move |_| set_closed.update(|prev| *prev = !*prev)
            >
                {icon!("mdi/cog")}
            </button>
            <div
                class="flex max-md:flex-col absolute top-12 p-5 \
                    w-full gap-3 justify-center bg-secondary rounded shadow-lg \
                    transition-opacity"
                class:opacity-0=closed
                class:z-0=closed
            >
                {timetable_settings.into_view()}
            </div>
        </div>
        <div class="w-auto overflow-x-auto pt-7">
            <Suspense fallback=move || match flags.get_value().view.get() {
                View::List => TimetableListLoading().into_view(),
                View::Grid => TimetableGridLoading().into_view(),
            }>
                {move || {
                    table_data()
                        .map(|res| res.unwrap())
                        .map(|classes| {
                            match flags.get_value().view.get() {
                                View::List =>  view! { <TimetableList data=classes flags=flags.get_value()/> },
                                View::Grid =>  view! { <TimetableGrid data=classes flags=flags.get_value()/> },
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn Checkbox<T>(
    id: &'static str,
    getter: Memo<bool>,
    setter: SignalSetter<Option<bool>>,
    children: T,
) -> impl IntoView
where
    T: IntoView,
{
    view! {
        <input
            type="checkbox"
            id=id
            checked=getter
            on:change=move |e| setter(is_checked(e))
        />
        <label for=id>{children}</label>
    }
}

fn on_radio_change<'a, T>(setter: SignalSetter<Option<T>>, name: &str)
where
    T: std::str::FromStr + std::default::Default,
{
    let val = document()
        .query_selector(&format! {"[name={name}]:checked"})
        .unwrap()
        .and_then(|el| Some(el.unchecked_into::<web_sys::HtmlInputElement>().value()));
    setter(val.map(|v| T::from_str(v.as_str()).unwrap_or_default()))
}

fn timetable_settings() -> (impl IntoView, TimetableFlags) {
    use crate::utils::create_query_signal;

    let (view, set_view) = create_query_signal::<View>("view");
    let (time_style, set_time_style) = create_query_signal::<TimeStyle>("time_style");
    let (show_loc, set_show_loc) = create_query_signal::<bool>("show_loc");
    let (show_prof, set_show_prof) = create_query_signal::<bool>("show_prof");
    let (show_code, set_show_code) = create_query_signal::<bool>("show_code");

    // TODO: save params
    let flags = TimetableFlags {
        view: create_memo(move |_| view().unwrap_or_default()),
        time_style: create_memo(move |_| time_style().unwrap_or_default()),
        show_loc: create_memo(move |_| show_loc().unwrap_or(true)),
        show_prof: create_memo(move |_| show_prof().unwrap_or(true)),
        show_code: create_memo(move |_| show_code().unwrap_or(true)),
    };

    let on_view_change = move |_| on_radio_change(set_view, "view");
    let on_time_style_change = move |_| on_radio_change(set_time_style, "time_style");

    let view = view! {
        <div class="grid grid-cols-[min-content,_1fr] gap-x-2 content-start">
            <Checkbox id="location" getter=flags.show_loc setter=set_show_loc>
                "Display class location"
            </Checkbox>
            <Checkbox id="prof" getter=flags.show_prof setter=set_show_prof>
                "Display class professor"
            </Checkbox>
            <Checkbox id="code" getter=flags.show_code setter=set_show_code>
                "Display class code"
            </Checkbox>
        </div>
        <div class="grid grid-cols-[min-content,_1fr] gap-x-2 content-start">
            <input
                type="radio"
                name="view"
                id="grid"
                value="grid"
                checked=move || matches!(flags.view.get(), View::Grid)
                on:change=on_view_change
            />
            <label for="grid">"Grid"</label>
            <input
                type="radio"
                name="view"
                id="list"
                value="list"
                checked=move || matches!(flags.view.get(), View::List)
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
                checked=move || matches!(flags.time_style.get(), TimeStyle::Times)
                on:change=on_time_style_change
            />
            <label for="times">"Display class times only"</label>
            <input
                type="radio"
                name="time_style"
                id="numbers"
                value="numbers"
                checked=move || matches!(flags.time_style.get(), TimeStyle::Numbers)
                on:change=on_time_style_change
            />
            <label for="numbers">"Display class period # only"</label>
            <input
                type="radio"
                name="time_style"
                id="both"
                value="both"
                checked=move || matches!(flags.time_style.get(), TimeStyle::Both)
                on:change=on_time_style_change
            />
            <label for="both">"Display both"</label>
        </div>
    };

    (view, flags)
}

#[component]
pub fn TimetableItem<'a>(
    class: &'a Class,
    #[prop(default = 1)] colspan: usize,
    #[prop(default = false)] is_grid: bool,
    #[prop(default = true.into(), into)] show_prof: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] show_location: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] show_code: MaybeSignal<bool>,
) -> impl IntoView {
    let class = store_value(class.clone());
    let style = if is_grid {
        "block"
    } else {
        "before:content-['_-_']"
    };

    view! {
        <td colspan=colspan class=format!("p-1 {}", class().kind.to_bg_color())>
            <span class="text-xs">{format!("[{}] ", class().kind)}</span>
            <Show when=show_code fallback=|| ()>
                <span class="text-xs">{class().code}</span>
            </Show>
            <span class=format!("font-bold {}", style)>{class().name}</span>
            <Show when=show_prof fallback=|| ()>
                {class().prof
                    .map(|prof| {
                        view! { <span class=format!("text-xs font-thin {}", style)>{prof}</span> }
                    })}

            </Show>
            <Show when=show_location fallback=|| ()>
                <span class="text-xs block">{class().location.to_string()}</span>
            </Show>
        </td>
    }
}
