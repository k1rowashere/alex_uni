use leptos::*;

use super::grid::TimetableCell;
use super::Class;
use super::*;

fn count_days(table_data: &[Class]) -> [u8; 7] {
    let mut days = [0; 7];
    for class in table_data {
        days[class.day as usize] += 1;
    }
    days
}

fn create_read_slice<T, O>(
    signal: impl SignalWith<Value = T> + 'static,
    getter: impl Fn(&T) -> O + Copy + 'static,
) -> Memo<O>
where
    O: Clone + Copy + PartialEq,
{
    Memo::new(move |_| signal.with(getter))
}

/// Assumes list is sorted by day of week and period
#[component]
pub fn TimetableList(
    data: Vec<Class>,
    #[prop(optional, into)] flags: MaybeSignal<TimetableFlags>,
) -> impl IntoView {
    let time_style = create_read_slice(flags, |f| f.time_style);
    let show_loc = create_read_slice(flags, |f| f.show_loc);
    let show_prof = create_read_slice(flags, |f| f.show_prof);
    let show_code = create_read_slice(flags, |f| f.show_code);

    let rowspans = count_days(&data);
    let mut prev_day = DayOfWeek::Friday;
    let timetable_list_row = |class: Class| {
        view! {
            <tr>
                {
                    let curr_day = class.day;
                    if curr_day != prev_day {
                        prev_day = curr_day;
                        view! {
                            <th rowspan=rowspans[class.day as usize]>
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
                    class=&class
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
