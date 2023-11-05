use leptos::*;

use super::subjects_signal::SubjectsSignal;
use super::{Seats, Subject};
use crate::class::{Class, Type as ClassType};
use crate::icon;

#[component]
pub fn ClassCard(subject: Subject) -> impl IntoView {
    let Subject { id, max_seats, group, lec, tut, lab } = subject;
    let subjects_ctx = expect_context::<SubjectsSignal>();

    let on_click = move |_| subjects_ctx.toggle(id);
    let is_selected = subjects_ctx.is_selected(id);
    let has_collisions = subjects_ctx.has_collisions(id);

    let prof = extract_prof_name(&lec);
    let sec_no = sec_no(&tut, &lab);
    let rem_seats = Memo::new(rem_seats(id, is_selected));

    let format_class_time = |c: Class| {
        view! {
            <div class="grid grid-cols-3">
                <div>
                    {match c.ctype {
                        ClassType::Lecture { .. } => "Lecture",
                        ClassType::Tutorial { .. } => "Tutorial",
                        ClassType::Lab { .. } => "Lab",
                    }}
                </div>
                <div>
                    {icon!("mdi/calendar-today", "mx-1 text-indigo-300 inline-block align-middle")}
                    {c.day.short_name()}
                </div>
                <div>
                    {icon!("mdi/clock-outline", "mx-1 text-indigo-300 inline-block align-middle")}
                    {format_period(c.period)}
                </div>
            </div>
        }
    };

    // "outline-red-500 outline-green-500"
    view! {
        <div class="p-2 m-2 rounded-xl dark:shadow-gray-600 shadow-md \
                    w-1/3 bg-indigo-50 dark:bg-indigo-950 bg-opacity-50 outline"
            class:outline-red-500=has_collisions
            class:outline-green-500=is_selected
            class:outline=move || has_collisions() || is_selected()
        >
            <div class="flex flex-col gap-1">
                <p class="uppercase text-indigo-500 dark:text-indigo-300">
                    {"Group "} {group}
                    {sec_no.map(|&sn| format!(" - Section {}", sn as u8))}
                </p>
                <p class="italic">{prof}</p>
                {format_class_time(lec)}
                {tut.map(format_class_time)}
                {lab.map(format_class_time)}
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

fn format_period((mut st, end): (usize, usize)) -> String {
    st += 1;
    if st == end {
        format!("{st}")
    } else {
        format!("{st} → {end}")
    }
}

fn extract_prof_name(lec: &Class) -> String {
    match lec.ctype {
        ClassType::Lecture { ref prof } => prof.clone(),
        _ => String::new(),
    }
}

fn sec_no<'a>(
    tut: &'a Option<Class>,
    lab: &'a Option<Class>,
) -> Option<&'a crate::timetable::Section> {
    [tut.as_ref(), lab.as_ref()].iter().flatten().find_map(
        |Class { ctype, .. }| match ctype {
            ClassType::Tutorial { sec_no, .. }
            | ClassType::Lab { sec_no, .. } => Some(sec_no),
            _ => None,
        },
    )
}

fn rem_seats(
    id: super::SubjectId,
    is_selected: Memo<bool>,
) -> impl Fn(Option<&u32>) -> u32 {
    let seats_ctx = expect_context::<Seats>();
    move |prev| {
        seats_ctx.with(|s| {
            s.iter()
                .find_map(|&(sid, seats)| (sid == id).then_some(seats))
                .or(prev.cloned())
                .unwrap_or_default()
        }) - if is_selected() { 1 } else { 0 }
    }
}
