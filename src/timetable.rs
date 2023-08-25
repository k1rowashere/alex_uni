use leptos::*;

const PERIOD_START_TIME: [&'static str; 12] = [
    "08:30 AM", "09:20 AM", "10:20 AM", "11:10 AM", "12:10 PM", "01:00 PM", "02:00 PM", "02:50 PM",
    "03:50 PM", "04:40 PM", "05:40 PM", "06:30 PM",
];

const PERIOD_END_TIME: [&'static str; 12] = [
    "09:20 AM", "10:10 AM", "11:10 AM", "12:00 PM", "01:00 PM", "01:50 PM", "02:50 PM", "03:40 PM",
    "04:40 PM", "05:30 PM", "06:30 PM", "07:20 PM",
];

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SubjectType {
    Lecture,
    Lab,
    Tutorial,
}

impl std::fmt::Display for SubjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubjectType::Lecture => write!(f, "Lec"),
            SubjectType::Lab => write!(f, "Lab"),
            SubjectType::Tutorial => write!(f, "Tut"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SubjectLocation {
    building: String,
    floor: u8,
    room: String,
}

#[derive(Clone, Hash, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DayOfWeek {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl From<usize> for DayOfWeek {
    fn from(value: usize) -> Self {
        match value {
            0 => DayOfWeek::Saturday,
            1 => DayOfWeek::Sunday,
            2 => DayOfWeek::Monday,
            3 => DayOfWeek::Tuesday,
            4 => DayOfWeek::Wednesday,
            5 => DayOfWeek::Thursday,
            6 => DayOfWeek::Friday,
            _ => unreachable!(),
        }
    }
}
impl ToString for DayOfWeek {
    fn to_string(&self) -> String {
        match self {
            DayOfWeek::Saturday => "Saturday",
            DayOfWeek::Sunday => "Sunday",
            DayOfWeek::Monday => "Monday",
            DayOfWeek::Tuesday => "Tuesday",
            DayOfWeek::Wednesday => "Wednesday",
            DayOfWeek::Thursday => "Thursday",
            DayOfWeek::Friday => "Friday",
        }
        .to_string()
    }
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Subject {
    type_: SubjectType,
    name: String,
    prof: String,
    location: SubjectLocation,
    day_of_week: DayOfWeek,
    /// inclusive range, 0-indexed
    period: (usize, usize),
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SubjectOption {
    None,
    JoinMark,
    Some(Subject),
}

// ----

#[server(GetTimetableSubjects, "api", "GetJson", "GetTimetableSubjects")]
async fn get_timetable_subjects() -> Result<Vec<Subject>, ServerFnError> {
    // TEMP: simulate slow network
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut data = vec![
        Subject {
            type_: SubjectType::Lecture,
            name: "CS 3".to_string(),
            prof: "Dr. John Doe".to_string(),
            location: SubjectLocation {
                building: "E1".to_string(),
                floor: 1,
                room: "101".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (0, 0),
        },
        Subject {
            type_: SubjectType::Lecture,
            name: "CS 2".to_string(),
            prof: "Dr. John Doe".to_string(),
            location: SubjectLocation {
                building: "E1".to_string(),
                floor: 1,
                room: "101".to_string(),
            },
            day_of_week: DayOfWeek::Monday,
            period: (4, 5),
        },
        Subject {
            type_: SubjectType::Lecture,
            name: "CS 1".to_string(),
            prof: "Dr. John Doe".to_string(),
            location: SubjectLocation {
                building: "E1".to_string(),
                floor: 1,
                room: "101".to_string(),
            },
            day_of_week: DayOfWeek::Monday,
            period: (1, 3),
        },
    ];

    // TODO: check for conflicts

    data.sort_by(|a, b| {
        let a = (a.day_of_week as usize, a.period.0);
        let b = (b.day_of_week as usize, b.period.0);
        a.cmp(&b)
    });

    Ok(data)
}

fn usize_to_short_day(n: usize) -> &'static str {
    match n {
        0 => "Sat",
        1 => "Sun",
        2 => "Mon",
        3 => "Tue",
        4 => "Wed",
        5 => "Thu",
        6 => "Fri",
        _ => unreachable!(),
    }
}

fn timetable_from_subjects(subjects: Vec<Subject>) -> [[SubjectOption; 12]; 6] {
    let mut timetable: [[SubjectOption; 12]; 6] =
        std::array::from_fn(|_| std::array::from_fn(|_| SubjectOption::None));
    for s in subjects {
        timetable[s.day_of_week as usize][s.period.0] = SubjectOption::Some(s.clone());
        for i in s.period.0 + 1..=s.period.1 {
            timetable[s.day_of_week as usize][i] = SubjectOption::JoinMark;
        }
    }
    timetable
}

#[component]
fn TimetableGridView(table_data: Vec<Subject>) -> impl IntoView {
    let table_body = timetable_from_subjects(table_data)
        .iter()
        .enumerate()
        .map(|(i, row)| {
            // TODO: Remove unwrap
            view! {
                <tr>
                    <th>{usize_to_short_day(i)}</th>
                    {
                        row
                        .iter()
                        .map(|s| {
                            match s {
                                SubjectOption::None => view! { <><td/></> },
                                SubjectOption::JoinMark => leptos::Fragment::new([].to_vec()),
                                SubjectOption::Some(s) => {
                                    let colspan = s.period.1 - s.period.0 + 1;
                                    view! { <><td colspan=colspan>{s.name.clone()}</td></> }
                                }
                            }
                        })
                        .collect_view()
                    }
                </tr>
            }
        })
        .collect_view();

    view! {
        <table class="w-full max-w-7xl mx-auto">
            <thead>
                <tr>
                    <td class="w-[unset] h-[unset]"></td>
                    {
                        PERIOD_START_TIME
                        .iter()
                        .enumerate()
                        .map(|(i, &t)| {
                            view! {
                                <th>
                                    <span class="block">{i + 1}</span>
                                    <span>{t}</span>
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
fn TimetableListView(table_data: Vec<Subject>) -> impl IntoView {
    // assumes the list is sorted by day_of_week, and then period

    let mut prev_day = DayOfWeek::Friday;
    let mut day_header = |s: &Subject| {
        if s.day_of_week != prev_day {
            prev_day = s.day_of_week;
            // count number of subjects in the same day
            // shortcircuits when the day changes (since it's sorted)
            let rowspan = table_data
                .iter()
                .take_while(|s| s.day_of_week == prev_day)
                .count();
            view! {
                <>
                    <th rowspan=rowspan>{s.day_of_week.to_string()}</th>
                </>
            }
        } else {
            leptos::Fragment::new([].to_vec())
        }
    };

    let table_body = table_data
        .iter()
        .map(|s| {
            view! {
                <tr>
                    {day_header(s)}
                    <td>
                        {format!(
                            "{} → {}",
                            PERIOD_START_TIME[s.period.0],
                            PERIOD_END_TIME[s.period.1],
                        )}
                    </td>
                    <td>
                        {format!(
                                "[{}]\n{}",
                                s.type_,
                                s.name,

                        )}
                    </td>
                </tr>
            }
        })
        .collect_view();

    view! {
        <table class="w-full max-w-7xl mx-auto">
            <thead>
                <th>"Day"</th>
                <th>"Time"</th>
                <th>"Subject"</th>
            </thead>
            <tbody>{table_body}</tbody>
        </table>
    }
}

#[component]
pub fn TimetablePage() -> impl IntoView {
    #[derive(Clone)]
    enum View {
        Grid,
        List,
    }

    let table_data = create_resource(|| (), |_| async move { get_timetable_subjects().await });

    let (view, set_view) = create_signal(View::Grid);

    view! {
        // TODO: add settings to timetable generation
        // - toggles for:
        // - period time visibility
        // - period time format (24h, 12h)
        // - toggle for subject info (prof, location)
        // o multiple timetable layouts
        // o list view
        // o grid view (default)
        <h1 class="text-5xl">
            timetable
        </h1>
        <div class="w-auto overflow-x-auto pt-7">
        // TODO: add fancy loading
            <Suspense fallback=move || view! { <p>"loading"</p> }>
                {move || {
                    table_data
                        .read()
                        // TODO: Remove unwrap
                        .map(|res| res.unwrap())
                        .map(|subjects| {
                            match view() {
                                View::Grid => view!{<TimetableGridView table_data=subjects/>},
                                View::List => view!{<TimetableListView table_data=subjects/>},
                            }
                        })
                }}
            </Suspense>
        </div>
        // mode switcher
        <div class="flex justify-center">
            <button
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                on:click=move |_| set_view(View::Grid)
            >
                "Grid"
            </button>
            <button
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                on:click=move |_| set_view(View::List)
            >
                "List"
            </button>
        </div>
        // TODO: store settings
        // TODO: form for settings
    }
}
