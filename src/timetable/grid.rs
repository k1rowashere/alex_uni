use leptos::*;

use super::Class;
use super::*;

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
pub fn TimetableGridLoading() -> impl IntoView {
    view! {
        <table class="timetable_grid skeleton w-full h-[70vh]">
            <thead>
                <tr>
                    <td class="!w-[unset] px-2"/>
                    {(0..12)
                        .map(|_| {
                            view! {
                                <th class="p-1">
                                    <div class="mx-auto rounded-xl w-[2ch] h-4"></div>
                                    <div class="mx-auto rounded-xl w-[7ch] h-4"></div>
                                    <div class="mx-auto rounded-xl w-[8ch] h-4"></div>
                                </th>
                            }
                        })
                        .collect_view()}
                </tr>
            </thead>
            <tbody>
                {(0..6).map(|_| view! {
                    <tr>
                       <th>
                       <div class="rounded-xl w-[3ch] h-4"></div>
                       </th>
                       {(0..12).map(|_| view! {
                          <td class="p-1">
                              <div class="mx-auto rounded-xl w-[8ch] h-4"></div>
                              </td>
                      }).collect_view()}
                    </tr>
                    })
                .collect_view()}
            </tbody>
        </table>
    }
}

#[component]
pub fn TimetableGrid(data: Vec<Class>, flags: TimetableFlags) -> impl IntoView {
    let table_body = timetable_from_classes(data)
        .iter()
        .enumerate()
        .map(|(i, row)| view! { <TimetableGridRow i=i row=row flags=&flags/> })
        .collect_view();

    view! {
        <table class="w-full timetable_grid">
        <thead>
            <tr>
                <td class="!w-[unset]"/>
                {PERIOD_START_TIME
                    .iter()
                    .zip(PERIOD_END_TIME)
                    .enumerate()
                    .map(|(i, (&s, e))| {
                        view! {
                            <th class="p-1">
                                <Show when=move || &flags.time_style.get() != &TimeStyle::Times fallback=|| ()>
                                    <span class="block">{i + 1}</span>
                                </Show>
                                <Show when=move || &flags.time_style.get() != &TimeStyle::Numbers fallback=|| ()>
                                    <span class="text-xs block">{s} {" → "} {e}</span>
                                </Show>
                            </th>
                        }
                    })
                    .collect_view()
                }
            </tr>
        </thead>
        <tbody>{table_body}</tbody>
        </table>
    }
}

#[component]
fn TimetableGridRow<'a>(
    i: usize,
    row: &'a [ClassOption],
    flags: &'a TimetableFlags,
) -> impl IntoView {
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

    view! {
        <tr>
            <th class="text-left px-2">{usize_to_day(i)}</th>
            {row
                .iter()
                .map(|class| {
                    match class {
                        ClassOption::None => view! { <td class="w-[calc(200%/25)]"></td> }.into_view(),
                        ClassOption::Join => view! { <td class="hidden"></td> }.into_view(),
                        ClassOption::Some(c) => {
                            let colspan = c.period.1 - c.period.0 + 1;
                            view! {
                                <TimetableItem
                                    class=c
                                    is_grid=true
                                    colspan=colspan
                                    show_location=flags.show_loc
                                    show_prof=flags.show_prof
                                    show_code=flags.show_code
                                />
                            }
                        }
                    }
                })
                .collect_view()}
        </tr>
    }
}
