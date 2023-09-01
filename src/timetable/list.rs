use leptos::*;

use super::{DayOfWeek, __Class as Class, PERIOD_END_TIME, PERIOD_START_TIME};

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
pub fn TimetableList(table_data: Vec<Class>) -> impl IntoView {
    // assumes the list is sorted by day_of_week, and then period
    let rowspans = count_days(&table_data);
    let mut prev_day = DayOfWeek::Friday;

    let table_body = table_data
        .iter()
        .map(|c| {
            view! {
                <tr>
                    <TimetableListDayHead
                        curr_day=c.day_of_week
                        prev_day=&mut prev_day
                        rowspan=rowspans[c.day_of_week as usize]
                    />
                    <td class="w-min px-2 text-center">
                        {PERIOD_START_TIME[c.period.0]}
                        <br/>
                        {" → "}{PERIOD_END_TIME[c.period.1]}
                    </td>
                    <TimetableListItem c=c/>
                </tr>
            }
        })
        .collect_view();

    view! {
        <table class="timetable_list w-full max-w-7xl mx-auto">
            <thead>
                <th>"Day"</th>
                <th>"Time"</th>
                <th>"Class"</th>
            </thead>
            <tbody>{table_body}</tbody>
        </table>
    }
}

#[component]
fn TimetableListItem<'a>(c: &'a Class) -> impl IntoView {
    let class = "p-1 ".to_string() + c.kind.to_bg_color();
    view! {
        <td class=class>
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
