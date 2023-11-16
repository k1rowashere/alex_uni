mod class_card;
#[cfg(feature = "ssr")]
pub mod rem_seats_ws;
mod server_fns;
mod subjects_signal;

use std::collections::BTreeSet;

use leptos::*;
use leptos_router::*;
use leptos_use::{use_websocket, UseWebsocketReturn};
use serde::{Deserialize, Serialize};

use crate::class::Class;
use crate::components::accordion::*;
use crate::registration::class_card::ClassCard;
use crate::timetable::{View, *};
use subjects_signal::SubjectsSignal;

#[rustfmt::skip]
#[derive(Serialize, PartialOrd, Ord, PartialEq, Eq, Deserialize, Copy, Clone, Hash, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type), sqlx(transparent))]
pub struct SubjectId(i64);

/// A collection of different choices for a specific subject
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct SubjectChoices {
    level: u8,
    name: String,
    code: String,
    choices: Vec<Subject>,
}

/// A container for a class and its associated sections and labs
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Subject {
    id: SubjectId,
    group: u8,
    max_seats: u32,
    lec: Class,
    tut: Option<Class>,
    lab: Option<Class>,
}

pub type SelectedSubjects = Result<BTreeSet<SubjectId>, ServerFnError>;
pub type SelectedSubjectsResource = Resource<(), SelectedSubjects>;
pub type AllSubjectsResource =
    Resource<(), Result<Vec<SubjectChoices>, ServerFnError>>;
type Seats = Signal<Vec<(SubjectId, u32)>>;
type TabRwSignal = (Memo<usize>, SignalSetter<usize>);

#[component]
pub fn RegistrationPage() -> impl IntoView {
    // let all_subjects = Resource::new(|| (), |_| get_registerable_subjects());
    // let selected_subjects = Resource::new(|| (), |_| get_subbed_subjects());
    // TODO: handle errors with a modal or smth..
    // Should only run once
    // should subjects_signal handle resources?
    // let subjects_signal = SubjectsSignal::new(selected_subjects, all_subjects);
    // provide_context(subjects_signal);

    let tab_idx = {
        let (get, set) = create_query_signal::<usize>("page");
        (
            Memo::new(move |_| get().unwrap_or_default()),
            SignalSetter::map(move |idx| set(Some(idx))),
        )
    };

    let rem_seats_ws = {
        let UseWebsocketReturn { message, .. } = use_websocket("/ws/rem_seats");
        move || {
            message()
                .and_then(|msg| serde_json::from_str(&msg).ok())
                .unwrap_or_default()
        }
    };

    provide_context(rem_seats_ws.into_signal() as Seats);

    // TODO: Make scrollable overflow
    //       Hide extra data in a dropdown?
    //       Add a filter bar (by group, section, ...)
    //       Add and/or subtract from the rem_seats when selected
    view! {
        <subjects_signal::CxtProvider let:subjects>
            {move ||
                subjects
                .choices()
                .with_value(|sc| {
                    let tabs: BTreeSet<_> = sc
                        .iter()
                        .map(|c| c.level as usize)
                        .collect();
                    let start_tab = tabs.first().cloned().unwrap_or_default();
                    view! { <TabSelector tabs start_tab selector=tab_idx/> }
            })}
            <div class="rounded-b-lg p-4 bg-secondary shadow-lg">
                <div class="flex flex-row items-stretch gap-2">
                    <ClassAccordion curr_level=tab_idx.0 subjects/>
                    <SideMenu/>
                </div>
                // status + action bar
                <div class="w-full py-2 flex gap-2 justify-end">
                    // TODO: add status bar (collisions, selected credit hours...)
                    <button
                        type="button"
                        class="btn-primary-outline max-w-[1/6]"
                        disabled=subjects.saved()
                        on:click=move |_| subjects.discard()
                    >
                        "Discard"
                    </button>
                    <button
                        type="submit"
                        class="btn-primary max-w-[1/6]"
                        disabled=subjects.saved()
                        on:click=move |_| subjects.save()
                    >
                        "Save"
                    </button>
                </div>
                <TimetableGrid
                    data=subjects.classes()
                    flags=TimetableFlags {
                        time_style: TimeStyle::Numbers,
                        show_loc: false,
                        show_prof: false,
                        show_code: true,
                        view: View::Grid,
                    }
                />
            </div>
        </subjects_signal::CxtProvider>
    }
}

#[component]
fn ClassAccordion(
    #[prop(into)] curr_level: Signal<usize>,
    subjects: SubjectsSignal,
) -> impl IntoView {
    // TODO: fix start_open
    fn row((_i, s): (usize, SubjectChoices)) -> leptos::View {
        view! {
            <AccordionItem
                class="[&:has([data-selected])]:border-indigo-300 \
                    [&:has([data-invalid])]:!border-red-300 \
                    bg-gray-50 dark:bg-slate-900"
                inner_class="grid px-0.5 grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-2"
                head=move || view! { <span class="font-bold">{"["}{s.code}{"] "}{s.name}</span> }
            >
                {s.choices
                    .into_iter()
                        .map(|subject| view! { <ClassCard subject/> })
                        .collect_view()
                }
            </AccordionItem>
        }
    }

    view! {
        <Accordion>
            {move || subjects.choices().with_value(|s| {
                s.iter()
                    .filter(|c| c.level == curr_level() as u8)
                    .cloned()
                    .enumerate()
                    .map(row)
                .collect_view()
            })}
        </Accordion>
    }
}

#[component]
fn TabSelector(
    #[prop(into)] tabs: MaybeSignal<BTreeSet<usize>>,
    selector: TabRwSignal,
    #[prop(optional)] start_tab: usize,
) -> impl IntoView {
    let (get, set) = selector;
    create_render_effect(move |_| set(start_tab));
    view! {
        <div class="flex flex-row gap-1 rounded-t-lg bg-tertiary">
            <h1 class="m-2 font-bold text-xl opacity-50">"Registration"</h1>
            <For each=tabs key=|i| *i let:item>
                <button
                    type="button"
                    role="tab"
                    class="link flex-grow p-2 text-center flex rounded-t-lg aria-selected:bg-secondary \
                        max-w-[20%] aria-selected:font-bold"
                    aria-selected=move || (get() == item).to_string()
                    aria-controls=move || format!("tab-{}", item)
                    on:click=move |_| set(item)
                >
                    {"Level "}{item}
                </button>
            </For>
        </div>
    }
}

#[component]
fn SideMenu() -> impl IntoView {
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
