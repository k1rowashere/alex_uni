use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::class::{Class, ClassType};
use crate::components::accordion::*;
use crate::timetable::{grid::TimetableGrid, TimeStyle, TimetableFlags};

macro_rules_attribute::derive_alias! {
    #[derive(common!)] = #[derive(Serialize, Deserialize, Clone, PartialEq, Eq)];
}

#[cfg(feature = "ssr")]
fn server_error(str: &str) -> ServerFnError {
    ServerFnError::ServerError(str.to_string())
}

#[cfg(feature = "ssr")]
const DB_CTX_ERR: &str = "Database context not found";

const TIMETABLE_FLAGS: TimetableFlags = TimetableFlags {
    time_style: TimeStyle::Numbers,
    show_loc: false,
    show_prof: false,
    show_code: true,
    view: crate::timetable::View::Grid,
};

/// A collection of different choices for a specific subject
#[derive(common!)]
pub struct SubjectChoices {
    level: u8,
    name: String,
    code: String,
    choices: Vec<Subject>,
}

/// A container for a class and its associated sections and labs
#[derive(common!)]
pub struct Subject {
    subs: u32,
    lec: Class,
    tut: Option<Class>,
    lab: Option<Class>,
}

#[server]
async fn get_std_class_list() -> Result<Vec<SubjectChoices>, ServerFnError> {
    use crate::class::db::ClassRow;
    use cached::proc_macro::cached;
    use futures::TryStreamExt;
    use futures::{stream, StreamExt};
    use sqlx::types::Json;

    // Structs used to query + deserialize from DB
    #[derive(common!, Hash, sqlx::Type)]
    #[sqlx(transparent)]
    struct SubjectId(i64);

    #[derive(common!, Hash)]
    pub struct SubjectChoicesId {
        level: u8,
        name: String,
        code: String,
        choices: Json<Vec<SubjectId>>,
    }

    #[cached(time = 5, result)]
    async fn subject_by_id(s: SubjectId) -> sqlx::Result<Subject> {
        let req = expect_context::<actix_web::HttpRequest>();
        let pool = req
            .app_data::<sqlx::Pool<sqlx::Sqlite>>()
            .expect(DB_CTX_ERR);

        let mut subjects = sqlx::query_as!(
            ClassRow,
            r#"
                SELECT cv.*
                FROM classes_view AS cv 
                INNER JOIN term_subjects AS ts 
                    ON cv.id IN (ts.lec_id, ts.lab_id, ts.tut_id)
                WHERE ts.id = ?
                ORDER BY CASE
                    WHEN cv.ctype = 'lec' THEN 1
                    WHEN cv.ctype = 'tut' THEN 2
                    WHEN cv.ctype = 'lab' THEN 3
                END ASC
            "#,
            s
        )
        .fetch_all(pool)
        .await?;

        let count = sqlx::query_scalar!(
            r#"SELECT count(*) as count FROM subjects WHERE id = ?"#,
            s
        )
        .fetch_one(pool)
        .await?;

        let len = subjects.len();
        let err = |e| sqlx::Error::Decode(Box::new(e));

        let lab = subjects.pop();
        let tut = subjects.pop();
        let lec = subjects.pop();

        match len {
            1 => Ok(Subject {
                subs: count as u32,
                lec: lec.unwrap().try_into().map_err(err)?,
                tut: None,
                lab: None,
            }),
            3 => Ok(Subject {
                subs: count as u32,
                lec: lec.unwrap().try_into().map_err(err)?,
                tut: Some(tut.unwrap().try_into().map_err(err)?),
                lab: Some(lab.unwrap().try_into().map_err(err)?),
            }),
            _ => Err(sqlx::Error::RowNotFound),
        }
    }

    async fn subject_choices_map(
        s: SubjectChoicesId,
    ) -> sqlx::Result<SubjectChoices> {
        Ok(SubjectChoices {
            level: s.level,
            name: s.name,
            code: s.code,
            choices: stream::iter(s.choices.to_vec())
                .map(subject_by_id)
                .buffered(4)
                .try_collect::<Vec<_>>()
                .await?,
        })
    }

    let req = expect_context::<actix_web::HttpRequest>();
    let pool = req
        .app_data::<sqlx::Pool<sqlx::Sqlite>>()
        .ok_or(server_error(DB_CTX_ERR))?;

    // Get Student ID from JWT cookie
    let student_id = 0;

    let subjects = sqlx::query_file_as!(
        SubjectChoicesId,
        "sql/select_subjects_choices.sql",
        student_id
    )
    .fetch_all(pool)
    .await?;

    let subjects = stream::iter(subjects)
        .map(subject_choices_map)
        .buffer_unordered(4)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(subjects)
}

#[component]
fn class_accordion(
    #[prop(into)] curr_tab: Signal<usize>,
    class_list: Vec<SubjectChoices>,
) -> impl IntoView {
    let class_list = store_value(class_list);
    view! {
        <Accordion id="subjects_accordion">
            {move || {
                class_list()
                    .into_iter()
                    .filter(|c| c.level == curr_tab() as u8)
                    .enumerate()
                    .map(move |(i, classes)| {
                        view! {
                            <AccordionItem
                                head=|| view! {
                                    <ClassAccordionHead name=classes.name code=classes.code/>
                        #[cached]        }
                                id=i
                            >
                                {classes
                                    .choices
                                    .into_iter()
                                    .map(|class| view! { <Class class/> })
                                    .collect_view()}
                            </AccordionItem>
                        }
                    })
                    .collect_view()
            }}
        </Accordion>
    }
}

#[component]
fn class_accordion_head(name: String, code: String) -> impl IntoView {
    // TODO: add selected group number + sections
    view! { <div class="font-bold">{format!("[{code}] {name}")}</div> }
}

#[component]
fn class(class: Subject) -> impl IntoView {
    let lec = class.lec;
    // let [tut1, tut2] = class.tuts;
    // let [lab1, lab2] = class.labs;

    view! {
        <div class="flex [&:not(:first-child)]:top-separator items-center content-center gap-2">
            <div class="w-full flex flex-col gap-2">
                <div>
                    // <div>{format!("Group {}", lec.group)}</div>
                    <span>"[Lecture]"</span>
                    {if let ClassType::Lecture(prof) = &lec.ctype {
                        view! { <span class="text-xs font-thin">{prof}</span> }.into_view()
                    } else {
                        ().into_view()
                    }}
                    <span class="text-xs">{lec.location.to_string()}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn registration_page() -> impl IntoView {
    let class_list = create_resource(|| (), |_| get_std_class_list());

    // provide_context(class_map);

    let (selected_classes, set_selected_classes) = create_signal(Vec::new());
    let _ = set_selected_classes;

    let (tab_idx, set_tab_idx) = create_query_signal::<usize>("page");
    let set_tab_idx = SignalSetter::map(move |idx| set_tab_idx(Some(idx)));
    let get_tab_idx = create_memo(move |_| tab_idx().unwrap_or_default());

    // TODO: Make scrollable overflow
    //       Hide extra data in a dropdown?
    //       Add a filter bar (by group, section, ...)
    //       Add deselect all button
    view! {
        <h1 class="text-4xl">"Class Registration"</h1>
        <Suspense fallback=move || view! { <div>"loading..."</div> }>
            <ErrorBoundary fallback=move |_| view!{<div>"server error"</div>}>
            {
                move || class_list.and_then(|class_list| {
                    let tabs: Vec<_> = class_list.iter().map(|c| format!("Level {}", c.level)).collect();
                    view!{<Tabbar tabs get_tab_idx set_tab_idx/>}
                })
            }
            </ErrorBoundary>
        </Suspense>
        <div class="rounded-b bg-secondary shadow-lg">
            <div class="p-4 flex flex-row items-stretch gap-1">
                <Suspense fallback=move || view! { <AccordionSkeleton count=6/> }>
                    <ErrorBoundary fallback=|_| {
                        view! { <div>"Server error"</div> }
                    }>
                        {move || {
                            class_list
                                .and_then(|class_list| {
                                    view! {
                                        <ClassAccordion
                                            curr_tab=get_tab_idx
                                            class_list=class_list.clone()
                                        />
                                    }
                                })
                        }}
                    </ErrorBoundary>
                </Suspense>
            <SideMenu/>
            </div>
            <TimetableGrid data=selected_classes flags=TIMETABLE_FLAGS/>
        </div>
    }
}

#[component]
fn tabbar(
    #[prop(into)] tabs: MaybeSignal<Vec<String>>,
    #[prop(into)] get_tab_idx: Signal<usize>,
    #[prop(into)] set_tab_idx: SignalSetter<usize>,
) -> impl IntoView {
    view! {
        <div class="flex flex-row gap-1 rounded-t bg-gray-500">
            <For
                each=move || tabs().into_iter().enumerate()
                key=|(i, _)| *i
                let:item
            >
                <button
                    role="tab"
                    class="link flex-grow p-2 text-center flex rounded-t-lg aria-selected:bg-secondary \
                        max-w-[20%] aria-selected:font-bold"
                    aria-selected=move || (get_tab_idx() == item.0).to_string()
                    aria-controls=move || format!("tab-{}", item.0)
                    on:click=move |_| set_tab_idx(item.0)
                >
                    {item.1.to_string()}
                </button>
            </For>
        </div>
    }
}

#[component]
fn side_menu() -> impl IntoView {
    view! {
        <div class="p-2 w-[min-content] flex flex-wrap items-center content-center justify-between gap-4 border rounded">
            <div class="flex flex-wrap gap-4">
                <span class="font-bold">"Apply a Preset Schedule:"</span>
                <div>
                    <select id="group_select" aria-label="group">
                        <option value="" selected disabled>
                            "Group…"
                        </option>
                        {(1..=4)
                            .map(|i| view! { <option value=i>{format!("Group {}", i)}</option> })
                            .collect_view()}
                    </select>
                    <select id="section_select" aria-label="section">
                        <option value="" selected disabled>
                            "Section…"
                        </option>
                        <option value=1>"Section 1"</option>
                        <option value=2>"Section 2"</option>
                    </select>
                </div>
            </div>
            <button class="btn-primary">"Apply Preset"</button>
        </div>
    }
}
