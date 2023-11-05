pub mod grid;
mod list;

use leptos::*;
use strum_macros::{Display, EnumString};

pub use crate::class::{Class, *};
use crate::components::checkbox::Checkbox;
use crate::components::suserr::TransErr;
use crate::icon;
pub use crate::timetable::grid::TimetableGrid;
pub use crate::timetable::list::TimetableList;
use crate::utils::get_radio_value;

pub const PERIOD_START_TIME: [&str; 12] = [
    "08:30 AM", "09:20 AM", "10:20 AM", "11:10 AM", "12:10 PM", "01:00 PM",
    "02:00 PM", "02:50 PM", "03:50 PM", "04:40 PM", "05:40 PM", "06:30 PM",
];

pub const PERIOD_END_TIME: [&str; 12] = [
    "09:20 AM", "10:10 AM", "11:10 AM", "12:00 PM", "01:00 PM", "01:50 PM",
    "02:50 PM", "03:40 PM", "04:40 PM", "05:30 PM", "06:30 PM", "07:20 PM",
];

#[derive(Display, EnumString, PartialEq, Copy, Clone, Default)]
#[strum(serialize_all = "snake_case")]
pub enum View {
    #[default]
    Grid,
    List,
}

#[derive(PartialEq, Copy, Clone, Default, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TimeStyle {
    Times,
    Numbers,
    #[default]
    Both,
}

#[derive(PartialEq, Copy, Clone, Default)]
pub struct TimetableFlags {
    pub view: View,
    pub time_style: TimeStyle,
    pub show_loc: bool,
    pub show_prof: bool,
    pub show_code: bool,
}

#[server(encoding = "GetJson")]
pub async fn get_std_classes() -> Result<Vec<Class>, ServerFnError> {
    use crate::class::db::ClassRow;
    use crate::login::user_id_from_jwt;

    let res = expect_context::<leptos_actix::ResponseOptions>();
    let req = expect_context::<actix_web::HttpRequest>();
    let pool = req
        .app_data::<sqlx::Pool<sqlx::Sqlite>>()
        .ok_or(ServerFnError::ServerError("No DB context provided".into()))?;

    let Some(student_id) = user_id_from_jwt(&req) else {
        res.set_status(actix_web::http::StatusCode::UNAUTHORIZED);
        return Ok(vec![]);
    };

    let classes_db = sqlx::query_as!(
        ClassRow,
        r#"
            SELECT 
                classes_view.*
            FROM classes_view
            INNER JOIN term_subjects as ts
            INNER JOIN term_subscribers as tsub
                ON ts.id = tsub.term_subject_id
            WHERE tsub.student_id = ?;
        "#,
        student_id
    )
    .fetch_all(pool)
    .await?;

    let mut classes: Vec<Class> = classes_db
        .into_iter()
        .filter_map(|c| c.try_into().ok())
        .collect();

    // TODO: check for conflicts (handle biweekly courses)

    classes.sort_by(|a, b| {
        let a = (a.day as usize, a.period.0);
        let b = (b.day as usize, b.period.0);
        a.cmp(&b)
    });

    Ok(classes)
}

// TODO: handle bi-weekly classes
#[component]
pub fn TimetablePage() -> impl IntoView {
    let table_data =
        create_resource(|| (), |_| async move { get_std_classes().await });
    let (timetable_settings, flags) = timetable_settings_inner();
    let view = Memo::new(move |_| flags().view);

    // PERF: Investigate `template!{}`
    // FIXME: fix tailwind not grabbing dynamic styles
    // class="z-0 opacity-0 rotate-45"
    // TODO: annotate the settings menu
    view! {
        <div class="relative flex justify-between">
            <h1 class="text-4xl">"Timetable"</h1>
            <TimetableSettings>{timetable_settings}</TimetableSettings>
        </div>
        <div class="w-auto overflow-x-auto pt-7">
            <TransErr resource=table_data let:classes>
                {match view() {
                    View::List => view! { <TimetableList data=classes.to_owned() flags=flags/> },
                    View::Grid => view! { <TimetableGrid data=classes.to_owned() flags=flags/> },
                }}
            </TransErr>
        </div>
    }
}

#[component]
fn TimetableSettings(children: Children) -> impl IntoView {
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
    let flags = Memo::new(move |_| TimetableFlags {
        view: view().unwrap_or_default(),
        time_style: time_style().unwrap_or_default(),
        show_loc: show_loc().unwrap_or(true),
        show_prof: show_prof().unwrap_or(true),
        show_code: show_code().unwrap_or(true),
    });

    let view = Memo::new(move |_| flags().view);
    let time_style = Memo::new(move |_| flags().time_style);
    let show_loc = Memo::new(move |_| flags().show_loc);
    let show_prof = Memo::new(move |_| flags().show_prof);
    let show_code = Memo::new(move |_| flags().show_code);

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
