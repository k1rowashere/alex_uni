use leptos::*;

use super::grid::TimetableCell;
use super::Class;
use super::*;

fn count_days(table_data: &Vec<Class>) -> [u8; 7] {
    let mut days = [0; 7];
    for class in table_data {
        days[class.day_of_week as usize] += 1;
    }
    days
}

#[component]
pub fn timetable_list_loading() -> impl IntoView {
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
                {[move || {
                    view! {
                        <tr>
                            <th rowspan="2">
                                <div class="mx-auto rounded-xl w-[6ch] h-5"></div>
                            </th>
                            {row}
                        </tr>
                        <tr>{row}</tr>
                    }
                }; 5].collect_view()}
            </tbody>
        </table>
    }
}

#[component]
pub fn timetable_list(
    data: Vec<Class>,
    #[prop(optional, into)] flags: MaybeSignal<TimetableFlags>,
) -> impl IntoView {
    // assumes the list is sorted by day_of_week, and then period
    let flags = store_value(flags);
    let time_style = create_memo(move |_| flags.get_value().get().time_style);
    let show_loc = create_memo(move |_| flags.get_value().get().show_loc);
    let show_prof = create_memo(move |_| flags.get_value().get().show_prof);
    let show_code = create_memo(move |_| flags.get_value().get().show_code);

    let rowspans = count_days(&data);
    let mut prev_day = DayOfWeek::Friday;
    let timetable_list_row = |class: Class| {
        view! {
            <tr>
                {
                    let curr_day = class.day_of_week;
                    if curr_day!= prev_day {
                        prev_day = curr_day;
                        view! {
                            <th rowspan=rowspans[class.day_of_week as usize]>
                                {curr_day.to_string()}
                            </th>
                        }.into_view()
                    } else {
                        ().into_view()
                    }
                }
                <Show
                    when=move || time_style() != TimeStyle::Numbers
                    fallback=|| ()
                >
                    <td class="px-2 text-center">
                        {PERIOD_START_TIME[class.period.0]} {" → "}
                        {PERIOD_END_TIME[class.period.1]}
                    </td>
                </Show>
                <Show
                    when=move || time_style() != TimeStyle::Times
                    fallback=|| ()
                >
                    <td class="px-2 text-center">
                        {class.period.0 + 1}
                        {
                            if class.period.0  != class.period.1  {
                                format!( " → {}", class.period.1 + 1)
                            } else {
                                "".to_string()
                            }
                        }
                    </td>
                </Show>
                <TimetableCell
                    class=class
                    show_location=show_loc
                    show_prof=show_prof
                    show_code=show_code
                />
            </tr>
        }
    };

    view! {
        <table class="timetable_list w-full">
            <thead>
                <th class="w-[calc(100%/8)]">"Day"</th>
                <Show when=move || time_style() != TimeStyle::Numbers fallback=|| ()>
                    <th class="w-[calc(100%/8 * 1.5)]">"Time"</th>
                </Show>
                <Show when=move || time_style() != TimeStyle::Times fallback=|| ()>
                    <th class="w-[calc(100%/8)]">"Period #"</th>
                </Show>
                <th>"Class"</th>
            </thead>
            <tbody>
                {data
                    .into_iter()
                    .map(timetable_list_row)
                    .collect_view()}
            </tbody>
        </table>
    }
}
