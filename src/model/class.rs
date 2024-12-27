use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIs, FromRepr, IntoStaticStr};

#[derive(Hash, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Default, sqlx::Type)]
pub enum WeekParity {
    #[default]
    Both,
    Even,
    Odd,
}

/// The type of class, i.e lecture, lab, tutorial
/// Lecture classes are always weekly and require prof name
/// Lab and tutorial classes can be bi-weekly and require section number
#[derive(Hash, Clone, PartialEq, Eq, Deserialize, Serialize, EnumIs)]
#[serde(tag = "type")]
pub enum Type {
    Lecture { prof: String },
    Lab { sec_no: u8, week_parity: WeekParity },
    Tutorial { sec_no: u8, week_parity: WeekParity },
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Lecture { .. } => write!(f, "Lec"),
            Type::Lab { sec_no, .. } => {
                write!(f, "Lab - group {}", *sec_no as i64)
            }
            Type::Tutorial { sec_no, .. } => {
                write!(f, "Tut - group {}", *sec_no as i64)
            }
        }
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Deserialize, Serialize, Copy, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
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

#[derive(Hash, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Location {
    pub building: Building,
    pub floor: u8,
    pub room: String,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {} Floor, {}",
            self.building,
            match self.floor {
                0 => "Ground".to_owned(),
                f if f % 10 == 1 && f % 11 != 0 => format!("{f}st"),
                f if f % 10 == 2 && f % 12 != 0 => format!("{f}nd"),
                f if f % 10 == 3 && f % 13 != 0 => format!("{f}rd"),
                f => format!("{f}th"),
            },
            self.room
        )
    }
}

#[derive(
    Hash,
    Clone,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    Copy,
    Display,
    FromRepr,
    IntoStaticStr,
    sqlx::Type,
)]
#[sqlx(rename_all = "snake_case")]
pub enum DayOfWeek {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl DayOfWeek {
    pub fn short_name(&self) -> &'static str {
        &Into::<&'static str>::into(self)[..3]
    }
}

// TODO: customise builder for this
#[derive(Hash, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Class {
    pub id: i64,
    /// The type of class, i.e lecture, lab, tutorial
    /// Lecture classes are always weekly and require prof name
    /// Lab and tutorial classes can be bi-weekly and require section number
    pub ctype: Type,
    /// The class ID, e.g "CSEx102"
    pub code: String,
    /// The class name
    pub name: String,
    pub location: Location,
    pub day: DayOfWeek,
    /// inclusive range, 0-indexed
    pub start_period: usize,
    pub end_period: usize,
}

pub struct ClassRecord {
    pub id: i64,
    pub ctype: String,
    pub prof: String,
    pub name: String,
    pub code: String,
    pub building: Building,
    pub floor: i64,
    pub room: String,
    pub day_of_week: DayOfWeek,
    pub start_period: i64,
    pub end_period: i64,
    pub section: u8,
    pub week_parity: WeekParity,
}

/// Since the db has constrains over the fields, a `TryInto` is not needed.
#[allow(clippy::from_over_into)]
impl Into<Class> for ClassRecord {
    fn into(self) -> Class {
        let ctype = match self.ctype.as_str() {
            "lec" => Type::Lecture { prof: self.prof },
            "lab" => Type::Lab {
                sec_no: self.section,
                week_parity: self.week_parity,
            },
            "tut" => Type::Tutorial {
                sec_no: self.section,
                week_parity: self.week_parity,
            },
            _ => unreachable!(),
        };

        Class {
            id: self.id,
            ctype,
            code: self.code,
            name: self.name,
            location: Location {
                building: self.building,
                floor: self.floor as u8,
                room: self.room,
            },
            day: self.day_of_week,
            start_period: self.start_period as usize,
            end_period: self.end_period as usize,
        }
    }
}
