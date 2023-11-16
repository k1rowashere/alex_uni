use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIs, FromRepr, IntoStaticStr};

#[derive(Hash, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[cfg_attr(
    feature = "ssr",
    derive(sqlx::Type),
    sqlx(rename_all = "snake_case")
)]
pub enum WeekParity {
    #[default]
    Both,
    Even,
    Odd,
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[repr(i64)]
pub enum Section {
    #[default]
    One = 1,
    Two,
}

/// The type of class, i.e lecture, lab, tutorial
/// Lecture classes are always weekly and require prof name
/// Lab and tutorial classes can be bi-weekly and require section number
#[derive(Hash, Clone, PartialEq, Eq, Deserialize, Serialize, EnumIs)]
pub enum Type {
    Lecture {
        prof: String,
    },
    Lab {
        sec_no: Section,
        week_parity: WeekParity,
    },
    Tutorial {
        sec_no: Section,
        week_parity: WeekParity,
    },
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

#[derive(Hash, Clone, PartialEq, Eq, Deserialize, Serialize, Copy)]
#[cfg_attr(
    feature = "ssr",
    derive(sqlx::Type),
    sqlx(rename_all = "snake_case")
)]
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
    building: Building,
    floor: u8,
    room: String,
}

impl std::fmt::Display for Location {
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
)]
#[cfg_attr(
    feature = "ssr",
    derive(sqlx::Type),
    sqlx(rename_all = "snake_case")
)]
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

#[derive(Hash, Clone, PartialEq, Eq, Deserialize, Serialize, Copy, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type), sqlx(transparent))]
pub struct ClassId(i64);

impl From<i64> for ClassId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

// TODO: customise builder for this
#[derive(Hash, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Class {
    pub id: ClassId,
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
    pub period: (usize, usize),
}

impl std::fmt::Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("id", &self.id.0)
            .field("ctype", &self.ctype.to_string())
            .field("code", &self.code)
            .field("name", &self.name)
            .field("location", &self.location.to_string())
            .field("day", &self.day.to_string())
            .field("period", &self.period)
            .finish()
    }
}

#[cfg(feature = "ssr")]
pub mod db {
    use super::*;
    pub struct ClassRow {
        pub id: i64,
        pub ctype: String,
        pub prof: String,
        pub name: String,
        pub code: String,
        pub building: Building,
        pub floor: i64,
        pub room: String,
        pub day_of_week: DayOfWeek,
        pub period_start: i64,
        pub period_end: i64,
        pub section: Section,
        pub week_parity: WeekParity,
    }

    /// Since the db has constrains over the fields, a `TryInto` is not needed.
    #[allow(clippy::from_over_into)]
    impl Into<Class> for ClassRow {
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
                id: ClassId(self.id),
                ctype,
                code: self.code,
                name: self.name,
                location: Location {
                    building: self.building,
                    floor: self.floor as u8,
                    room: self.room,
                },
                day: self.day_of_week,
                period: (self.period_start as usize, self.period_end as usize),
            }
        }
    }
}
