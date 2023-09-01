use leptos::*;

mod grid;
mod list;

use crate::timetable::grid::TimetableGrid;
use crate::timetable::list::TimetableList;

const PERIOD_START_TIME: [&'static str; 12] = [
    "08:30 AM", "09:20 AM", "10:20 AM", "11:10 AM", "12:10 PM", "01:00 PM", "02:00 PM", "02:50 PM",
    "03:50 PM", "04:40 PM", "05:40 PM", "06:30 PM",
];

const PERIOD_END_TIME: [&'static str; 12] = [
    "09:20 AM", "10:10 AM", "11:10 AM", "12:00 PM", "01:00 PM", "01:50 PM", "02:50 PM", "03:40 PM",
    "04:40 PM", "05:30 PM", "06:30 PM", "07:20 PM",
];

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ClassKind {
    Lecture,
    Lab,
    Tutorial,
}

impl ClassKind {
    pub fn to_bg_color(self: &Self) -> &'static str {
        match self {
            ClassKind::Lecture => "dark:bg-red-900 bg-red-200",
            ClassKind::Lab => "dark:bg-cyan-800 bg-cyan-200",
            ClassKind::Tutorial => "dark:bg-gray-800 bg-gray-200",
        }
    }
}

impl std::fmt::Display for ClassKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassKind::Lecture => write!(f, "Lec"),
            ClassKind::Lab => write!(f, "Lab"),
            ClassKind::Tutorial => write!(f, "Tut"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ClassLocation {
    building: String,
    floor: u8,
    room: String,
}

impl std::fmt::Display for ClassLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Building, {} Floor, {}",
            self.building,
            match self.floor {
                0 => "Ground".to_owned(),
                1 => "1st".to_owned(),
                2 => "2nd".to_owned(),
                3 => "3rd".to_owned(),
                f => format!("{}th", f),
            },
            self.room
        )
    }
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

// TODO: express biweekly classes and group specific classes
#[derive(
    Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize, derive_builder::Builder,
)]
pub struct __Class {
    kind: ClassKind,
    code: String,
    name: String,
    prof: Option<String>,
    location: ClassLocation,
    day_of_week: DayOfWeek,
    /// inclusive range, 0-indexed
    period: (usize, usize),
}

#[derive(Clone, PartialEq)]
pub enum ClassOption {
    None,
    Join,
    Some(__Class),
}

#[server(GetUserClasses, "/api", "GetJson", "get_user_classes")]
async fn get_user_classes() -> Result<Vec<__Class>, ServerFnError> {
    // TEMP: simulate slow network
    // std::thread::sleep(std::time::Duration::from_secs(1));

    let mut data = vec![
        __Class {
            kind: ClassKind::Lab,
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            prof: None,
            location: ClassLocation {
                building: "Electricity".to_string(),
                floor: 0,
                room: "Lab 7".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (4, 5),
        },
        __Class {
            kind: ClassKind::Tutorial,
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            prof: None,
            location: ClassLocation {
                building: "Electricity".to_string(),
                floor: 7,
                room: "Class 72".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (6, 6),
        },
        __Class {
            kind: ClassKind::Lab,
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            prof: None,
            location: ClassLocation {
                building: "Electricity".to_string(),
                floor: 3,
                room: "Lab Circuits".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (7, 7),
        },
        __Class {
            kind: ClassKind::Lecture,
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            prof: Some("أ.د. مجدي عبد العظيم".to_string()),
            location: ClassLocation {
                building: "Preparatory South".to_string(),
                floor: 0,
                room: "L3".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (8, 9),
        },
        __Class {
            kind: ClassKind::Lecture,
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            prof: Some("أ.د.م. مروان تركي".to_string()),
            location: ClassLocation {
                building: "SSP".to_string(),
                floor: 2,
                room: "C39".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (10, 11),
        },
        __Class {
            kind: ClassKind::Tutorial,
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            prof: None,
            location: ClassLocation {
                building: "Electricity".to_string(),
                floor: 7,
                room: "Class 701".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (2, 3),
        },
        __Class {
            kind: ClassKind::Lecture,
            code: "EMP x19".to_string(),
            name: "Probability and Statistics".to_string(),
            prof: Some("د. ميرفت ميخائيل".to_string()),
            location: ClassLocation {
                building: "Electricity".to_string(),
                floor: 0,
                room: "Class 103".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (5, 7),
        },
        __Class {
            kind: ClassKind::Lab,
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            prof: None,
            location: ClassLocation {
                building: "Electricity".to_string(),
                floor: 5,
                room: "Lab Logic".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (8, 9),
        },
        __Class {
            kind: ClassKind::Lecture,
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            prof: Some("د. عادل الفحار".to_string()),
            location: ClassLocation {
                building: "SSP".to_string(),
                floor: 1,
                room: "C26".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (4, 5),
        },
        __Class {
            kind: ClassKind::Tutorial,
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            prof: None,
            location: ClassLocation {
                building: "SSP".to_string(),
                floor: 2,
                room: "C44".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (6, 6),
        },
        __Class {
            kind: ClassKind::Tutorial,
            code: "EMP x19".to_string(),
            name: "Probability and Statistics".to_string(),
            prof: None,
            location: ClassLocation {
                building: "SSP".to_string(),
                floor: 0,
                room: "C11".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (7, 7),
        },
        __Class {
            kind: ClassKind::Lecture,
            code: "TRN x21".to_string(),
            name: "Technical Writing".to_string(),
            prof: Some("أ.د.م. أحمد التراس".to_string()),
            location: ClassLocation {
                building: "SSP".to_string(),
                floor: 2,
                room: "C40".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (8, 9),
        },
        __Class {
            kind: ClassKind::Tutorial,
            code: "EMP 116".to_string(),
            name: "Differential Equations".to_string(),
            prof: None,
            location: ClassLocation {
                building: "Preparatory South".to_string(),
                floor: 1,
                room: "C8".to_string(),
            },
            day_of_week: DayOfWeek::Wednesday,
            period: (0, 0),
        },
        __Class {
            kind: ClassKind::Lecture,
            code: "EMP 116".to_string(),
            name: "Differential Equations".to_string(),
            prof: Some("أ.د. عمرو عبد الرازاق".to_string()),
            location: ClassLocation {
                building: "Preparatory North".to_string(),
                floor: 0,
                room: "L5".to_string(),
            },
            day_of_week: DayOfWeek::Wednesday,
            period: (3, 5),
        },
    ];

    // TODO: check for conflicts (handle biweekly courses)

    data.sort_by(|a, b| {
        let a = (a.day_of_week as usize, a.period.0);
        let b = (b.day_of_week as usize, b.period.0);
        a.cmp(&b)
    });

    Ok(data)
}

#[component]
pub fn TimetablePage() -> impl IntoView {
    #[derive(Clone)]
    enum View {
        Grid,
        List,
    }
    let table_data = create_resource(|| (), |_| async move { get_user_classes().await });

    let (view, set_view) = create_signal(View::List);

    view! {
        // TODO: add settings to timetable generation
        // - toggles for:
        // - period time visibility
        // - period time format (24h, 12h)
        // - toggle for class info (prof, location)
        // o multiple timetable layouts
        // o list view
        // o grid view (default)
        // - use url to store state
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
                        .map(|classes| {
                            match view() {
                                View::Grid => view!{<TimetableGrid table_data=classes/>},
                                View::List => view!{<TimetableList table_data=classes/>},
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
