#[cfg(feature = "ssr")]
pub mod rem_seats_ws;
mod server_fns;

use std::collections::BTreeSet;

use leptos::*;
use leptos_router::*;
use leptos_use::{use_websocket, UseWebsocketReturn};
use serde::{Deserialize, Serialize};

use crate::and_then;
use crate::class::{Class, Type as ClassType};
use crate::components::accordion::*;
use crate::components::suserr::SusErr;
use crate::timetable::{View, *};
use server_fns::*;

#[derive(
    Serialize,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Deserialize,
    Copy,
    Clone,
    Hash,
    Debug,
)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type), sqlx(transparent))]
pub struct SubjectId(i64);

/// A collection of different choices for a specific subject
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SubjectSelect {
    level: u8,
    name: String,
    code: String,
    choices: Vec<Subject>,
}

/// A container for a class and its associated sections and labs
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Subject {
    id: SubjectId,
    group: u8,
    max_seats: u32,
    lec: Class,
    tut: Option<Class>,
    lab: Option<Class>,
}

type SelectedSubjects = Result<BTreeSet<SubjectId>, ServerFnError>;
type SelectedSubjectsResource = Resource<(), SelectedSubjects>;
type SubjectsResource = Resource<(), Result<Vec<SubjectSelect>, ServerFnError>>;
type RemSeats = Memo<Vec<(SubjectId, u32)>>;

#[inline]
fn map_subject_ids_to_classes(
    selected_subjects: SelectedSubjectsResource,
    subjects: SubjectsResource,
) -> Vec<Class> {
    if let Some(Ok(selected)) = selected_subjects() {
        if let Some(Ok(all)) = subjects() {
            all.iter()
                .flat_map(|s| s.choices.iter())
                .filter(|s| selected.contains(&s.id))
                .flat_map(|s| {
                    [Some(s.lec.clone()), s.tut.clone(), s.lab.clone()]
                })
                .flatten()
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

#[component]
pub fn registration_page() -> impl IntoView {
    let subjects: SubjectsResource =
        create_resource(|| (), |_| get_registerable_subjects());
    let selected_subjects: SelectedSubjectsResource = create_resource(
        || (),
        |_| async {
            Ok(BTreeSet::from_iter(
                // TODO: handle error (show modal or smth)
                get_subbed_subjects().await?.into_iter(),
            ))
        },
    );
    let ss_loading = selected_subjects.loading();
    provide_context(selected_subjects);
    // Maps selected_subjects -> selected_classs
    // to display in timetable
    // TODO: Collision checking
    let selected_classes =
        move || map_subject_ids_to_classes(selected_subjects, subjects);

    // Websocket for streaming remaining places live
    let rem_seats_msg: RemSeats = create_memo({
        let UseWebsocketReturn { message, .. } = use_websocket("/ws/rem_seats");
        move |_| {
            message()
                .and_then(|msg| serde_json::from_str(msg.as_str()).ok())
                .unwrap_or_default()
        }
    });
    provide_context(rem_seats_msg);

    let (get_tab_idx, set_tab_idx) = {
        let (get, set) = create_query_signal::<usize>("page");
        let getter = Memo::new(move |_| get().unwrap_or_default());
        let setter = SignalSetter::map(move |idx| set(Some(idx)));

        (getter, setter)
    };

    let save_action = create_action(move |_: &RegisterSubjects| async move {
        if let Some(Ok(ss)) = selected_subjects() {
            register_subjects(ss).await?;
        }
        Ok(())
    })
    .using_server_fn::<RegisterSubjects>();

    // TODO: Make scrollable overflow
    //       Hide extra data in a dropdown?
    //       Add a filter bar (by group, section, ...)
    view! {
        <h1 class="text-4xl">"Class Registration"</h1>
        <ActionForm
            action=save_action
            // on:submit=move |e| {
            //     e.prevent_default();
            //     save_action.dispatch(());
            // }
        >
        <SusErr>
            {and_then!(move |subjects| {
                let tabs: BTreeSet<_> = subjects
                    .iter()
                    .map(|c| (c.level as usize, format!("Level {}", c.level)))
                    .collect();
                let start_tab = tabs.first().map(|(i, _)| *i).unwrap_or_default();
                view! {
                    <LevelTabbar tabs start_tab get_tab_idx set_tab_idx/>
                    <div class="rounded-b-lg p-4 bg-secondary shadow-lg">
                        <div class="pb-4 flex flex-row items-stretch gap-1">
                            <ClassAccordion curr_level=get_tab_idx subjects/>
                            <SideMenu/>
                        </div>
                        <div>
                            <button
                                type="button"
                                class="btn-secondary"
                                on:click=move |_| selected_subjects.refetch()
                            >
                                "Discard"
                            </button>
                            <button type="submit" class="btn-primary" disabled=ss_loading>
                                "Save"
                            </button>
                        </div>
                        <TimetableGrid data=selected_classes.into_signal()
                            flags=TimetableFlags {
                                time_style: TimeStyle::Numbers,
                                show_loc: false,
                                show_prof: false,
                                show_code: true,
                                view: View::Grid,
                            }
                        />
                    </div>
                }
            })}
        </SusErr>
        </ActionForm>
    }
}

#[component]
fn class_accordion_head(name: String, code: String) -> impl IntoView {
    // TODO: add selected group number + sections
    view! { <div class="font-bold">{format!("[{code}] {name}")}</div> }
}

#[component]
fn class_accordion<'a>(
    #[prop(into)] curr_level: Signal<usize>,
    subjects: &'a [SubjectSelect],
) -> impl IntoView {
    let subjects = store_value(subjects.to_owned());

    view! {
        <Accordion>
            {move || {
                subjects.get_value()
                    .into_iter()
                    .filter(|c| c.level == curr_level() as u8)
                    .map(move |s| {
                        view! {
                            <AccordionItem head=|| {
                                view! { <ClassAccordionHead name=s.name code=s.code/> }
                            }>
                                {s.choices
                                    .into_iter()
                                    .map(|subject| view! { <Class subject/> })
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
fn class(subject: Subject) -> impl IntoView {
    let Subject {
        id,
        max_seats,
        group,
        lec,
        tut,
        lab,
    } = subject;

    let selected_subjects = expect_context::<SelectedSubjectsResource>();

    let rem_seats = create_memo(move |prev| {
        expect_context::<RemSeats>()
            .get()
            .iter()
            .find_map(|&(sid, seats)| (sid == id).then_some(seats))
            .or(prev.cloned())
            .unwrap_or_default()
    });

    let prof = match lec.ctype {
        ClassType::Lecture { prof } => prof,
        _ => String::new(),
    };

    let sec_no = {
        let sec_no_tut = tut.and_then(|t| match t.ctype {
            ClassType::Tutorial { sec_no, .. } => Some(sec_no),
            _ => None,
        });

        let sec_no_lab = lab.and_then(|t| match t.ctype {
            ClassType::Lab { sec_no, .. } => Some(sec_no),
            _ => None,
        });

        sec_no_tut.or(sec_no_lab)
    };

    let on_click = move |_| {
        selected_subjects.update(|ss| {
            if let Some(Ok(ss)) = ss {
                if !ss.insert(id) {
                    ss.remove(&id);
                }
            };
        })
    };
    let is_selected = move || {
        if let Some(Ok(ss)) = selected_subjects() {
            ss.contains(&id)
        } else {
            false
        }
    };

    // TODO:
    // - form stuff (class selection)
    // - highlight if selected

    // "bg-red-500"
    view! {
        <div class="flex [&:not(:first-child)]:top-separator items-center content-center gap-2">
            <div class="w-full flex flex-col gap-2">
                <div>
                    {format!("Group {group}")}
                    {sec_no.map_or(
                        String::new(),
                        |sec_no| format!(" - Section {}", sec_no as u8)
                    )}
                </div>
                <div>
                    <span>"[Lec]"</span>
                    <span class="text-xs font-thin">{prof}</span>
                    <span>{lec.day_of_week.to_string()}</span>
                    <span>
                        {lec.period.0 + 1}
                        {if lec.period.1 == lec.period.0 {
                            "".to_string()
                        } else {
                            format!(" → {}", lec.period.1 + 1)
                        }}
                    </span>
                </div>
                <button
                    type="button"
                    class=move || if is_selected() { "btn-danger" } else { "btn-primary" }
                    on:click=on_click
                >
                    {move || if is_selected() { "Remove" } else { "Add" }}
                    <span class="text-xs font-thin">
                        {move || format!(" ({} / {})", rem_seats(), max_seats)}
                    </span>
                </button>
            </div>
        </div>
    }
}

#[component]
fn level_tabbar(
    #[prop(into)] tabs: MaybeSignal<BTreeSet<(usize, String)>>,
    #[prop(into)] get_tab_idx: Signal<usize>,
    #[prop(into)] set_tab_idx: SignalSetter<usize>,
    #[prop(optional)] start_tab: usize,
) -> impl IntoView {
    create_effect(move |_| set_tab_idx(start_tab));
    view! {
        <div class="flex flex-row gap-1 rounded-t-lg bg-tertiary h-[calc(1em_+_1rem)]">
            <For
                each=tabs
                key=|(i, _)| *i
                let:item
            >
                <button
                    type="button"
                    role="tab"
                    class="link flex-grow p-2 text-center flex rounded-t-lg aria-selected:bg-secondary \
                        max-w-[20%] aria-selected:font-bold"
                    aria-selected=move || (get_tab_idx() == item.0).to_string()
                    aria-controls=move || format!("tab-{}", item.0)
                    on:click=move |_| set_tab_idx(item.0)
                >
                    {item.1}
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
            <button type="button" class="btn-primary">"Apply Preset"</button>
        </div>
    }
}
