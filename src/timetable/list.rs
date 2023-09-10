use leptos::*;

use super::{Class, DayOfWeek, PERIOD_END_TIME, PERIOD_START_TIME};

fn count_days(table_data: &Vec<Class>) -> [u8; 7] {
    let mut days = [0; 7];
    for class in table_data {
        days[class.day_of_week as usize] += 1;
    }
    days
}

#[component]
fn TimetableListDayHead<'a>(
    curr_day: DayOfWeek,
    prev_day: &'a mut DayOfWeek,
    rowspan: u8,
) -> impl IntoView {
    let prev_day_ = *prev_day;
    *prev_day = curr_day;
    view! {
        <Show when=move || curr_day != prev_day_ fallback=move || ()>
            <th rowspan=rowspan>{curr_day.to_string()}</th>
        </Show>
    }
}

#[component]
pub fn TimetableListLoading() -> impl IntoView {
    let row = || {
        view! {
            <td class="p-1 center">
                <div class="mx-auto rounded-xl w-[10ch] h-5"></div>
            </td>
            <td class="p-1">
                <div class="rounded-xl w-[6ch] h-4 inline-block mb-0.5"></div>
                <div class="rounded-xl w-[20ch] h-5 inline-block"></div>
                <div class="rounded-xl w-[10ch] h-4 block"></div>
            </td>
        }
    };

    view! {
        <table class="timetable_list skeleton w-full">
            <thead>
                <tr>
                    <th class="w-1/6 p-1">
                        <div class="mx-auto rounded-xl w-[3ch] h-5"></div>
                    </th>
                    <th class="w-1/6 p-1">
                        <div class="mx-auto rounded-xl w-[4ch] h-5"></div>
                    </th>
                    <th class="w-2/3 p-1">
                        <div class="mx-auto rounded-xl w-[5ch] h-5"></div>
                    </th>
                </tr>
            </thead>
            <tbody>
                {(0..5)
                    .map(|_| {
                        view! {
                            <tr>
                                <th rowspan="2">
                                    <div class="mx-auto rounded-xl w-[6ch] h-5"></div>
                                </th>
                                {row}
                            </tr>
                            <tr>
                                {row}
                            </tr>
                        }
                    })
                    .collect_view()
                }
            </tbody>
        </table>
    }
}

#[component]
pub fn TimetableList(data: Vec<Class>) -> impl IntoView {
    // assumes the list is sorted by day_of_week, and then period
    let rowspans = count_days(&data);
    let mut prev_day = DayOfWeek::Friday;

    view! {
        <table class="timetable_list w-full">
            <thead>
                <th class="w-1/6">"Day"</th>
                <th class="w-1/6">"Time"</th>
                <th class="w-2/3">"Class"</th>
            </thead>
            <tbody>
                {data
                    .iter()
                    .map(|c| {
                        view! {
                            <tr>
                                <TimetableListDayHead
                                    curr_day=c.day_of_week
                                    prev_day=&mut prev_day
                                    rowspan=rowspans[c.day_of_week as usize]
                                />
                                <td class="px-2 text-center">
                                    {PERIOD_START_TIME[c.period.0]} {" → "}
                                    {PERIOD_END_TIME[c.period.1]}
                                </td>
                                <TimetableListItem c=c/>
                            </tr>
                        }
                    })
                    .collect_view()}
            </tbody>
        </table>
    }
}

#[component]
fn TimetableListItem<'a>(c: &'a Class) -> impl IntoView {
    view! {
        <td class="p-1" class=c.kind.to_bg_color()>
            <span class="text-xs">{format!("[{}] ", c.kind)}</span>
            <span class="text-xs after:content-['_-_']">{&c.code}</span>
            <span class="font-bold after:content-['_-_']">{&c.name}</span>
            {c.prof.as_ref().map(|p| view! { <span class="text-sm font-thin">{p}</span> })}
            <br/>
            <span class="text-xs">{c.location.to_string()}</span>
        </td>
    }
}
