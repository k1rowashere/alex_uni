use crate::utils::Unzip;
use leptos::*;

use super::Class;
use super::*;

#[derive(Clone, PartialEq)]
enum TimetableCell {
    None,
    Join,
    Some(Class),
}

type GridSignal = (
    [[Signal<TimetableCell>; 12]; 6],
    [[WriteSignal<TimetableCell>; 12]; 6],
);

fn create_grid_signal(classes: Vec<Class>) -> GridSignal {
    let timetable = grid_from_classes(classes);
    timetable
        .map(|row| {
            row.map(|cell| {
                let (r, w) = create_signal(cell);
                (r.into_signal(), w)
            })
            .unzip()
        })
        .unzip()
}

fn grid_from_classes(classes: Vec<Class>) -> [[TimetableCell; 12]; 6] {
    use std::array::from_fn;
    use TimetableCell as Cell;
    let mut timetable = from_fn(|_| from_fn(|_| Cell::None));
    for s in classes {
        let row = s.day_of_week as usize;
        let st = s.period.0;
        let end = s.period.1;
        timetable[row][st] = Cell::Some(s);
        for i in st + 1..=end {
            timetable[row][i] = Cell::Join;
        }
    }
    timetable
}

#[component]
pub fn timetable_grid_loading() -> impl IntoView {
    view! {
        <table class="timetable_grid skeleton w-full h-[70vh]">
            <thead>
                <tr>
                    <td class="!w-[unset] px-2"></td>
                    {[|| view! {
                        <th class="p-1">
                            <div class="mx-auto rounded-xl w-[2ch] h-4"></div>
                            <div class="mx-auto rounded-xl w-[7ch] h-4"></div>
                            <div class="mx-auto rounded-xl w-[8ch] h-4"></div>
                        </th>
                    }; 12] .collect_view()}
                </tr>
            </thead>
            <tbody>
                {[|| view! {
                    <tr>
                        <th>
                            <div class="rounded-xl w-[3ch] h-4"></div>
                        </th>
                        {[|| view! {
                            <td class="p-1">
                                <div class="mx-auto rounded-xl w-[8ch] h-4"></div>
                            </td>
                        }; 12].collect_view()}
                    </tr>
                }; 6].collect_view()}
            </tbody>
        </table>
    }
}

#[component]
pub fn timetable_grid(
    #[prop(into)] data: MaybeSignal<Vec<Class>>,
    #[prop(optional, into)] flags: MaybeSignal<TimetableFlags>,
) -> impl IntoView {
    let flags = store_value(flags);
    let time_style = create_memo(move |_| flags().get().time_style);
    let show_location = create_memo(move |_| flags().get().show_loc);
    let show_prof = create_memo(move |_| flags().get().show_prof);
    let show_code = create_memo(move |_| flags().get().show_code);

    let (grid, set_grid) = create_grid_signal(data.get_untracked());

    // this effect is responsible for updating the grid upon change in data
    // PERF: This might not be the most optimal way, (try derived signals?)
    create_effect(move |prev: Option<[[TimetableCell; 12]; 6]>| {
        let curr = grid_from_classes(data());
        if let Some(prev) = prev {
            for ((prev, curr), set) in prev.iter().zip(&curr).zip(set_grid) {
                for ((prev, curr), set) in prev.iter().zip(curr).zip(set) {
                    if prev != curr {
                        set(curr.clone());
                    }
                }
            }
        }
        curr
    });

    let head = PERIOD_START_TIME
        .into_iter()
        .zip(PERIOD_END_TIME)
        .enumerate();

    view! {
        <table class="w-full timetable_grid">
            <thead>
                <td class="!w-[unset]"></td>
                {head
                    .map(|(i, (s, e))| {
                        view! {
                            <th class="p-1">
                                <Show when=move || time_style() != TimeStyle::Times fallback=|| ()>
                                    <span class="block">{i + 1}</span>
                                </Show>
                                <Show
                                    when=move || time_style() != TimeStyle::Numbers
                                    fallback=|| ()
                                >
                                    <span class="text-xs block">{s} {" → "} {e}</span>
                                </Show>
                            </th>
                        }
                    })
                    .collect_view()}
            </thead>
            <tbody>
                {grid
                    .into_iter()
                    .enumerate()
                    .map(|(i, row)| {
                        view! {
                            <TimetableGridRow
                                i=i
                                row=row
                                show_location=show_location
                                show_prof=show_prof
                                show_code=show_code
                            />
                        }
                    })
                    .collect_view()}
            </tbody>
        </table>
    }
}

#[component]
fn timetable_grid_row(
    i: usize,
    #[prop(into)] row: [Signal<TimetableCell>; 12],
    #[prop(default = true.into(), into)] show_prof: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] show_location: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] show_code: MaybeSignal<bool>,
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
            {
                row.map(|cell|
                    view!{
                        <TimetableItemWrapped
                            cell=cell
                            show_location=show_location
                            show_prof=show_prof
                            show_code=show_code
                        />
                    }
                    )
            }
        </tr>
    }
}

#[component]
fn timetable_item_wrapped(
    #[prop(into)] cell: MaybeSignal<TimetableCell>,
    #[prop(default = true.into(), into)] show_prof: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] show_location: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] show_code: MaybeSignal<bool>,
) -> impl IntoView {
    use TimetableCell as Cell;
    move || match cell() {
        Cell::None => view! { <td class="w-[calc(200%/25)]"></td> }.into_view(),
        Cell::Join => ().into_view(),
        Cell::Some(class) => {
            let colspan = class.period.1 - class.period.0 + 1;
            view! {
                <TimetableCell
                    class=class
                    is_grid=true
                    colspan=colspan
                    show_location=show_location
                    show_prof=show_prof
                    show_code=show_code
                />
            }
        }
    }
}

#[component]
pub fn timetable_cell(
    class: Class,
    #[prop(default = 1)] colspan: usize,
    #[prop(default = false)] is_grid: bool,
    #[prop(default = true.into(), into)] show_prof: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] show_location: MaybeSignal<bool>,
    #[prop(default = true.into(), into)] show_code: MaybeSignal<bool>,
) -> impl IntoView {
    let style = if is_grid {
        "block"
    } else {
        "before:content-['_-_']"
    };

    view! {
        <td colspan=colspan class=format!("p-1 {}", class.ctype.to_bg_color())>
            <span class="text-xs">{format!("[{}] ", class.ctype)}</span>
            <Show when=show_code fallback=|| ()>
                <span class="text-xs">{&class.code}</span>
            </Show>
            <span class=format!("font-bold {}", style)>{&class.name}</span>
            <Show when=show_prof fallback=|| ()>
                {if let ClassType::Lecture(prof) = &class.ctype {
                    view! { <span class=format!("text-xs font-thin {}", style)>{prof}</span> }
                        .into_view()
                } else {
                    ().into_view()
                }}
            </Show>
            <Show when=show_location fallback=|| ()>
                <span class="text-xs block">{class.location.to_string()}</span>
            </Show>
        </td>
    }
}
