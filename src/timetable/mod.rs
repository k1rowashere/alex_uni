mod grid;
mod list;
mod timetable;

pub use timetable::*;

pub const PERIOD_START_TIME: [&'static str; 12] = [
    "08:30 AM", "09:20 AM", "10:20 AM", "11:10 AM", "12:10 PM", "01:00 PM", "02:00 PM", "02:50 PM",
    "03:50 PM", "04:40 PM", "05:40 PM", "06:30 PM",
];

pub const PERIOD_END_TIME: [&'static str; 12] = [
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
pub enum Building {
    Electricity,
    Mechanics,
    PreparatorySouth,
    PreparatoryNorth,
    Ssp,
}

impl std::fmt::Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Electricity => write!(f, "Electricity Building"),
            Self::Mechanics => write!(f, "Mechanics Building"),
            Self::PreparatorySouth => write!(f, "Preparatory Building South"),
            Self::PreparatoryNorth => write!(f, "Preparatory Building North"),
            Self::Ssp => write!(f, "SSP Building"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ClassLocation {
    building: Building,
    floor: u8,
    room: String,
}

impl std::fmt::Display for ClassLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {} Floor, {}",
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
pub struct Class {
    pub kind: ClassKind,
    pub code: String,
    pub name: String,
    pub prof: Option<String>,
    pub location: ClassLocation,
    pub day_of_week: DayOfWeek,
    /// inclusive range, 0-indexed
    pub period: (usize, usize),
}

#[derive(Clone, PartialEq)]
pub enum ClassOption {
    None,
    Join,
    Some(Class),
}
