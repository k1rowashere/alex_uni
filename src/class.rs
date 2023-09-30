use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

derive_alias! {
    #[derive(Common!)] = #[derive(Hash, Clone, PartialEq, Eq, Deserialize, Serialize)];
}

#[derive(Common!)]
pub enum WeekParity {
    None,
    Even,
    Odd,
}

#[derive(Common!, Copy)]
pub enum Section {
    One = 1,
    Two,
}

/// The type of class, i.e lecture, lab, tutorial
/// Lecture classes are always weekly and require prof name
/// Lab and tutorial classes can be bi-weekly and require section number
#[derive(Common!, IntoStaticStr)]
pub enum ClassType {
    Lecture(String),
    Lab(Section, WeekParity),
    Tutorial(Section, WeekParity),
}

impl std::fmt::Display for ClassType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassType::Lecture(_) => write!(f, "Lec"),
            ClassType::Lab(sec, _) => write!(f, "Lab - group {}", *sec as u8),
            ClassType::Tutorial(sec, _) => {
                write!(f, "Tut - group {}", *sec as u8)
            }
        }
    }
}

#[derive(Common!, Copy, EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
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

#[derive(
    Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize, Builder,
)]
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

#[derive(Common!, Copy, Display, EnumIter, FromRepr)]
pub enum DayOfWeek {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(Common!, Copy, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type), sqlx(transparent))]
pub struct ClassId(i64);

impl From<i64> for ClassId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

// TODO: customise builder for this
#[derive(Common!, Builder)]
pub struct Class {
    #[builder(setter(into), default)]
    pub id: ClassId,
    /// The type of class, i.e lecture, lab, tutorial
    /// Lecture classes are always weekly and require prof name
    /// Lab and tutorial classes can be bi-weekly and require section number
    pub ctype: ClassType,
    /// The class ID, e.g "CSEx102"
    #[builder(setter(into))]
    pub code: String,
    /// The class name
    #[builder(setter(into))]
    pub name: String,
    pub location: ClassLocation,
    pub day_of_week: DayOfWeek,
    /// inclusive range, 0-indexed
    #[builder(setter(custom))]
    pub period: (usize, usize),
}

impl ClassBuilder {
    pub fn period(&mut self, start: usize, end: usize) -> &mut Self {
        self.period = Some((start, end));
        self
    }
}

#[cfg(feature = "ssr")]
pub mod db {
    use super::Class;

    #[derive(sqlx::FromRow, Clone)]
    pub struct ClassRow {
        pub id: i64,
        pub ctype: String,
        pub prof: Option<String>,
        pub name: String,
        pub code: String,
        pub building: String,
        pub floor: i64,
        pub room: String,
        pub day_of_week: i64,
        pub period_start: i64,
        pub period_end: i64,
        pub section: Option<i64>,
        pub week_parity: Option<i64>,
    }

    #[derive(Debug)]
    pub enum ConversionError {
        InvalidValue(&'static str, String),
        MissingValue(&'static str),
    }

    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::InvalidValue(field, val) => {
                    write!(f, "Invalid value: '{val}' for '{field}'")
                }
                Self::MissingValue(field) => {
                    write!(f, "Missing value for {field}")
                }
            }
        }
    }

    impl std::error::Error for ConversionError {}

    impl TryInto<Class> for ClassRow {
        type Error = ConversionError;
        fn try_into(self) -> Result<Class, Self::Error> {
            use crate::class::*;
            let invalid = Self::Error::InvalidValue;
            let missing = Self::Error::MissingValue;

            let section = match self.section {
                Some(1) => Some(Section::One),
                Some(2) => Some(Section::Two),
                None => None,
                Some(v) => return Err(invalid("section", v.to_string())),
            };

            let parity = match self.week_parity {
                Some(0) => WeekParity::Even,
                Some(1) => WeekParity::Odd,
                None => WeekParity::None,
                Some(a) => return Err(invalid("week_parity", a.to_string())),
            };

            let ctype = match self.ctype.as_str() {
                "lec" => ClassType::Lecture(self.prof.ok_or(missing("prof"))?),
                "lab" => ClassType::Lab(
                    section.ok_or(missing("section_no_lab"))?,
                    parity,
                ),
                "tut" => ClassType::Tutorial(
                    section.ok_or(missing("section_no_tut"))?,
                    parity,
                ),
                v => return Err(invalid("ctype", v.to_string())),
            };

            Ok(Class {
                id: ClassId(self.id),
                ctype,
                code: self.code,
                name: self.name,
                location: ClassLocation {
                    building: {
                        let bld = self.building;
                        bld.as_str()
                            .try_into()
                            .map_err(|_| invalid("building", bld))?
                    },
                    floor: self.floor as u8,
                    room: self.room,
                },
                day_of_week: {
                    let dow = self.day_of_week;
                    DayOfWeek::from_repr(dow as usize)
                        .ok_or(invalid("day_of_week", dow.to_string()))?
                },
                period: (self.period_start as usize, self.period_end as usize),
            })
        }
    }
}

// TEMP:
// pub fn data() -> [Class; 14] {
//     use crate::class::{Section as S, WeekParity as P};
//     [
//         Class {
//             ctype: ClassType::Lab(S::One, P::None),
//             code: "CSE 127".to_string(),
//             name: "Data Structures I".to_string(),
//             location: ClassLocation {
//                 building: Building::Electricity,
//                 floor: 0,
//                 room: "Lab 7".to_string(),
//             },
//             day_of_week: DayOfWeek::Saturday,
//             period: (4, 5),
//         },
//         Class {
//             ctype: ClassType::Tutorial(S::One, P::None),
//             code: "CSE 136".to_string(),
//             name: "Digital Logic Circuits I".to_string(),
//             location: ClassLocation {
//                 building: Building::Electricity,
//                 floor: 7,
//                 room: "Class 72".to_string(),
//             },
//             day_of_week: DayOfWeek::Saturday,
//             period: (6, 6),
//         },
//         Class {
//             ctype: ClassType::Lab(S::One, P::None),
//             code: "EEC 116".to_string(),
//             name: "Analysis of Electrical Circuits".to_string(),
//             location: ClassLocation {
//                 building: Building::Electricity,
//                 floor: 3,
//                 room: "Lab Circuits".to_string(),
//             },
//             day_of_week: DayOfWeek::Saturday,
//             period: (7, 7),
//         },
//         Class {
//             ctype: ClassType::Lecture("أ.د. مجدي عبد العظيم".to_string()),
//             code: "CSE 136".to_string(),
//             name: "Digital Logic Circuits I".to_string(),
//             location: ClassLocation {
//                 building: Building::PreparatorySouth,
//                 floor: 0,
//                 room: "L3".to_string(),
//             },
//             day_of_week: DayOfWeek::Saturday,
//             period: (8, 9),
//         },
//         Class {
//             ctype: ClassType::Lecture("أ.د.م. مروان تركي".to_string()),
//             code: "CSE 127".to_string(),
//             name: "Data Structures I".to_string(),
//             location: ClassLocation {
//                 building: Building::Ssp,
//                 floor: 2,
//                 room: "C39".to_string(),
//             },
//             day_of_week: DayOfWeek::Saturday,
//             period: (10, 11),
//         },
//         Class {
//             ctype: ClassType::Tutorial(S::One, P::None),
//             code: "EEC 116".to_string(),
//             name: "Analysis of Electrical Circuits".to_string(),
//             location: ClassLocation {
//                 building: Building::Electricity,
//                 floor: 7,
//                 room: "Class 701".to_string(),
//             },
//             day_of_week: DayOfWeek::Sunday,
//             period: (2, 3),
//         },
//         Class {
//             ctype: ClassType::Lecture("د. ميرفت ميخائيل".to_string()),
//             code: "EMP x19".to_string(),
//             name: "Probability and Statistics".to_string(),
//             location: ClassLocation {
//                 building: Building::Electricity,
//                 floor: 0,
//                 room: "Class 103".to_string(),
//             },
//             day_of_week: DayOfWeek::Sunday,
//             period: (5, 7),
//         },
//         Class {
//             ctype: ClassType::Lab(S::One, P::None),
//             code: "CSE 136".to_string(),
//             name: "Digital Logic Circuits I".to_string(),
//             location: ClassLocation {
//                 building: Building::Electricity,
//                 floor: 5,
//                 room: "Lab Logic".to_string(),
//             },
//             day_of_week: DayOfWeek::Sunday,
//             period: (8, 9),
//         },
//         Class {
//             ctype: ClassType::Lecture("د. عادل الفحار".to_string()),
//             code: "EEC 116".to_string(),
//             name: "Analysis of Electrical Circuits".to_string(),
//             location: ClassLocation {
//                 building: Building::Ssp,
//                 floor: 1,
//                 room: "C26".to_string(),
//             },
//             day_of_week: DayOfWeek::Tuesday,
//             period: (4, 5),
//         },
//         Class {
//             ctype: ClassType::Tutorial(S::One, P::None),
//             code: "CSE 127".to_string(),
//             name: "Data Structures I".to_string(),
//             location: ClassLocation {
//                 building: Building::Ssp,
//                 floor: 2,
//                 room: "C44".to_string(),
//             },
//             day_of_week: DayOfWeek::Tuesday,
//             period: (6, 6),
//         },
//         Class {
//             ctype: ClassType::Tutorial(S::One, P::None),
//             code: "EMP x19".to_string(),
//             name: "Probability and Statistics".to_string(),
//             location: ClassLocation {
//                 building: Building::Ssp,
//                 floor: 0,
//                 room: "C11".to_string(),
//             },
//             day_of_week: DayOfWeek::Tuesday,
//             period: (7, 7),
//         },
//         Class {
//             ctype: ClassType::Lecture("أ.د.م. أحمد التراس".to_string()),
//             code: "TRN x21".to_string(),
//             name: "Technical Writing".to_string(),
//             location: ClassLocation {
//                 building: Building::Ssp,
//                 floor: 2,
//                 room: "C40".to_string(),
//             },
//             day_of_week: DayOfWeek::Tuesday,
//             period: (8, 9),
//         },
//         Class {
//             ctype: ClassType::Tutorial(S::One, P::None),
//             code: "EMP 116".to_string(),
//             name: "Differential Equations".to_string(),
//             location: ClassLocation {
//                 building: Building::PreparatorySouth,
//                 floor: 1,
//                 room: "C8".to_string(),
//             },
//             day_of_week: DayOfWeek::Wednesday,
//             period: (0, 0),
//         },
//         Class {
//             ctype: ClassType::Lecture("أ.د. عمرو عبد الرازاق".to_string()),
//             code: "EMP 116".to_string(),
//             name: "Differential Equations".to_string(),
//             location: ClassLocation {
//                 building: Building::PreparatoryNorth,
//                 floor: 0,
//                 room: "L5".to_string(),
//             },
//             day_of_week: DayOfWeek::Wednesday,
//             period: (3, 5),
//         },
//     ]
// }
