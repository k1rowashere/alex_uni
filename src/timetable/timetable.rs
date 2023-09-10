use leptos::*;

use super::Class;

use crate::timetable::grid::{TimetableGrid, TimetableGridLoading};
use crate::timetable::list::{TimetableList, TimetableListLoading};

#[server]
async fn get_user_classes() -> Result<Vec<Class>, ServerFnError> {
    use super::{Building, ClassKind, ClassLocation, DayOfWeek};
    // TEMP:
    let mut data = vec![
        Class {
            kind: ClassKind::Lab,
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 0,
                room: "Lab 7".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (4, 5),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 7,
                room: "Class 72".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (6, 6),
        },
        Class {
            kind: ClassKind::Lab,
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 3,
                room: "Lab Circuits".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (7, 7),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            prof: Some("أ.د. مجدي عبد العظيم".to_string()),
            location: ClassLocation {
                building: Building::PreparatorySouth,
                floor: 0,
                room: "L3".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (8, 9),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            prof: Some("أ.د.م. مروان تركي".to_string()),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 2,
                room: "C39".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (10, 11),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 7,
                room: "Class 701".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (2, 3),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "EMP x19".to_string(),
            name: "Probability and Statistics".to_string(),
            prof: Some("د. ميرفت ميخائيل".to_string()),
            location: ClassLocation {
                building: Building::Electricity,
                floor: 0,
                room: "Class 103".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (5, 7),
        },
        Class {
            kind: ClassKind::Lab,
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Electricity,
                floor: 5,
                room: "Lab Logic".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (8, 9),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            prof: Some("د. عادل الفحار".to_string()),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 1,
                room: "C26".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (4, 5),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Ssp,
                floor: 2,
                room: "C44".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (6, 6),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "EMP x19".to_string(),
            name: "Probability and Statistics".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::Ssp,
                floor: 0,
                room: "C11".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (7, 7),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "TRN x21".to_string(),
            name: "Technical Writing".to_string(),
            prof: Some("أ.د.م. أحمد التراس".to_string()),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 2,
                room: "C40".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (8, 9),
        },
        Class {
            kind: ClassKind::Tutorial,
            code: "EMP 116".to_string(),
            name: "Differential Equations".to_string(),
            prof: None,
            location: ClassLocation {
                building: Building::PreparatorySouth,
                floor: 1,
                room: "C8".to_string(),
            },
            day_of_week: DayOfWeek::Wednesday,
            period: (0, 0),
        },
        Class {
            kind: ClassKind::Lecture,
            code: "EMP 116".to_string(),
            name: "Differential Equations".to_string(),
            prof: Some("أ.د. عمرو عبد الرازاق".to_string()),
            location: ClassLocation {
                building: Building::PreparatoryNorth,
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

#[derive(PartialEq, Clone, Default)]
enum View {
    #[default]
    Grid,
    List,
}

impl ToString for View {
    fn to_string(&self) -> String {
        match self {
            View::Grid => "grid",
            View::List => "list",
        }
        .to_string()
    }
}

impl std::str::FromStr for View {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "grid" => Ok(View::Grid),
            "list" => Ok(View::List),
            _ => Err(()),
        }
    }
}

#[component]
fn Checkbox(getter: Memo<Option<bool>>, setter: SignalSetter<Option<bool>>) -> impl IntoView {
    let node_ref = create_node_ref::<html::Input>();
    view! {
        <input
            _ref=node_ref
            type="checkbox"
            checked=move ||matches!(getter(), None | Some(true))
            on:change=move |_| setter(node_ref.get().map(|n| n.checked()))
        />
    }
}
#[component]
pub fn TimetablePage() -> impl IntoView {
    use crate::utils::create_query_signal;

    let table_data = create_resource(|| (), |_| async move { get_user_classes().await });
    let (view, set_view) = create_query_signal::<View>("view");
    let (times_vis, set_times_vis) = create_query_signal::<bool>("times");
    let (loc_vis, set_loc_vis) = create_query_signal::<bool>("loc");
    let (prof_vis, set_prof_vis) = create_query_signal::<bool>("prof");
    let (per_no_vis, set_per_no_vis) = create_query_signal::<bool>("per_no");

    // TODO: save params, add period num to list view
    let radio_ref = create_node_ref::<html::Input>();
    let on_view_change = move |_| {
        set_view(if radio_ref.get().unwrap().checked() {
            Some(View::Grid)
        } else {
            Some(View::List)
        })
    };

    view! {
        <h1 class="text-4xl">"Timetable"</h1>
        <div class="flex justify-center">
            <Checkbox getter=times_vis setter=set_times_vis/>
            <Checkbox getter=loc_vis setter=set_loc_vis/>
            <Checkbox getter=prof_vis setter=set_prof_vis/>
            <Checkbox getter=per_no_vis setter=set_per_no_vis/>
            <input
                _ref=radio_ref
                type="radio"
                name="view"
                value="grid"
                checked=matches!(view(), None | Some(View::Grid))
                on:change=on_view_change
            />
            <input
                type="radio"
                name="view"
                value="list"
                checked=matches!(view(), Some(View::List))
                on:change=on_view_change
            />
        </div>
        <div class="w-auto overflow-x-auto pt-7">
            <Suspense
                fallback=move || match view() {
                    Some(View::List) => TimetableListLoading().into_view(),
                    Some(View::Grid) | None => TimetableGridLoading().into_view(),
                }
            >
                {move || {
                    table_data()
                        .map(|res| res.unwrap())
                        .map(|classes| {
                            match view() {
                                Some(View::List) => view! { <TimetableList data=classes/> },
                                Some(View::Grid) | None => view! { <TimetableGrid data=classes/> },
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}
