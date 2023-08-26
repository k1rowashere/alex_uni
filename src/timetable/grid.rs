use crate::timetable::{Class, ClassOption, PERIOD_END_TIME, PERIOD_START_TIME};
use leptos::*;

fn timetable_from_classes(classes: Vec<Class>) -> [[ClassOption; 12]; 6] {
    let mut timetable: [[ClassOption; 12]; 6] =
        std::array::from_fn(|_| std::array::from_fn(|_| ClassOption::None));
    for s in classes {
        timetable[s.day_of_week as usize][s.period.0] = ClassOption::Some(s.clone());
        for i in s.period.0 + 1..=s.period.1 {
            timetable[s.day_of_week as usize][i] = ClassOption::Join;
        }
    }
    timetable
}

#[component]
pub fn TimetableGrid(table_data: Vec<Class>) -> impl IntoView {
    let table_body = timetable_from_classes(table_data)
        .iter()
        .enumerate()
        .map(|(i, row)| view! { <TimetableGridRow i=i row=row/> })
        .collect_view();

    view! {
        <table class="w-full timetable_grid">
            <TimetableGridHead/>
            <tbody>{table_body}</tbody>
        </table>
    }
}

#[component]
fn TimetableGridHead() -> impl IntoView {
    view! {
        <thead>
            <tr>
                <td style="width: unset;"/>
                {PERIOD_START_TIME
                    .iter()
                    .zip(PERIOD_END_TIME)
                    .enumerate()
                    .map(|(i, (&s, e))| {
                        view! {
                            <th>
                                <span class="block">{i + 1}</span>
                                <span class="text-xs block">{s}</span>
                                <span class="text-xs">{" → "}{e}</span>
                            </th>
                        }
                    })
                    .collect_view()
                }
            </tr>
        </thead>
    }
}

#[component]
fn TimetableGridRow<'a>(i: usize, row: &'a [ClassOption]) -> impl IntoView {
    let usize_to_day = |i| match i {
        0 => "Sat",
        1 => "Sun",
        2 => "Mon",
        3 => "Tue",
        4 => "Wed",
        5 => "Thu",
        6 => "Fri",
        _ => unreachable!(),
    };

    // let calc_key = |c| match c {
    //     ClassOption::None => 0,
    //     ClassOption::Join => 1,
    //     ClassOption::Some(c) => 2 * c.period.1.pow(2) * c.period.0.pow(3),
    // };

    // TODO: this is prolly gonna use signals in the future

    view! {
        <tr>
            <th class="text-left px-2">{usize_to_day(i)}</th>
            {row.iter().map(|class| view! { <TimetableGridItem class=class/> }).collect_view()}
        </tr>
    }
}

#[component]
fn TimetableGridItem<'a>(class: &'a ClassOption) -> impl IntoView {
    match class {
        ClassOption::None => view! { <td class="w-[calc(2/25*100%)]"/> },
        ClassOption::Join => view! { <td class="hidden"/> },
        ClassOption::Some(c) => {
            let colspan = c.period.1 - c.period.0 + 1;
            let class = "p-1 ".to_string() + c.kind.to_bg_color();
            view! {
                <td colspan=colspan class=class>
                    <span class="text-xs">{format!("[{}] ", c.kind)}</span>
                    <span class="text-xs after:content-['_-_']">{&c.code}</span>
                    <br/>
                    <span class="font-bold">{&c.name}</span>
                    <br/>
                    {c.prof.as_ref().map(|p| view! { <span class="text-sm font-thin block">{p}</span> })}
                    <span class="text-xs">{c.location.to_string()}</span>
                </td>
            }
        }
    }
}
