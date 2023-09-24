use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumIter};

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
#[derive(Common!, AsRefStr)]
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
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
#[builder(setter(into))]
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

#[derive(Common!, Copy, Display, EnumIter)]
pub enum DayOfWeek {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

// TODO: customise builder for this
#[derive(Common!, Builder)]
pub struct Class {
    #[builder(setter(into, strip_option), default)]
    pub group: Option<u8>,
    /// The type of class, i.e lecture, lab, tutorial
    /// Lecture classes are always weekly and require prof name
    /// Lab and tutorial classes can be bi-weekly and require section number
    pub kind: ClassType,
    /// The class ID, e.g "CSEx102"
    #[builder(setter(into))]
    pub code: String,
    /// The class name
    #[builder(setter(into))]
    pub name: String,
    pub location: ClassLocation,
    pub day_of_week: DayOfWeek,
    /// inclusive range, 0-indexed
    pub period: (usize, usize),
}

impl Default for Class {
    fn default() -> Self {
        Self {
            group: None,
            kind: ClassType::Lecture("###########".into()),
            code: "### ####".into(),
            name: "###############".into(),
            location: ClassLocation {
                building: Building::PreparatoryNorth,
                floor: 0,
                room: "### ##".into(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (0, 0),
        }
    }
}

// TEMP:
pub fn data() -> [Class; 14] {
    use crate::class::{Section as S, WeekParity as P};
    [
        Class {
            group: Some(0),
            kind: ClassType::Lab(S::One, P::None),
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            location: ClassLocation {
                building: Building::Electricity,
                floor: 0,
                room: "Lab 7".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (4, 5),
        },
        Class {
            group: Some(0),
            kind: ClassType::Tutorial(S::One, P::None),
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            location: ClassLocation {
                building: Building::Electricity,
                floor: 7,
                room: "Class 72".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (6, 6),
        },
        Class {
            group: Some(0),
            kind: ClassType::Lab(S::One, P::None),
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            location: ClassLocation {
                building: Building::Electricity,
                floor: 3,
                room: "Lab Circuits".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (7, 7),
        },
        Class {
            group: Some(0),
            kind: ClassType::Lecture("أ.د. مجدي عبد العظيم".to_string()),
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            location: ClassLocation {
                building: Building::PreparatorySouth,
                floor: 0,
                room: "L3".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (8, 9),
        },
        Class {
            group: Some(0),
            kind: ClassType::Lecture("أ.د.م. مروان تركي".to_string()),
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 2,
                room: "C39".to_string(),
            },
            day_of_week: DayOfWeek::Saturday,
            period: (10, 11),
        },
        Class {
            group: Some(0),
            kind: ClassType::Tutorial(S::One, P::None),
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            location: ClassLocation {
                building: Building::Electricity,
                floor: 7,
                room: "Class 701".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (2, 3),
        },
        Class {
            group: Some(0),
            kind: ClassType::Lecture("د. ميرفت ميخائيل".to_string()),
            code: "EMP x19".to_string(),
            name: "Probability and Statistics".to_string(),
            location: ClassLocation {
                building: Building::Electricity,
                floor: 0,
                room: "Class 103".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (5, 7),
        },
        Class {
            group: Some(0),
            kind: ClassType::Lab(S::One, P::None),
            code: "CSE 136".to_string(),
            name: "Digital Logic Circuits I".to_string(),
            location: ClassLocation {
                building: Building::Electricity,
                floor: 5,
                room: "Lab Logic".to_string(),
            },
            day_of_week: DayOfWeek::Sunday,
            period: (8, 9),
        },
        Class {
            group: Some(0),
            kind: ClassType::Lecture("د. عادل الفحار".to_string()),
            code: "EEC 116".to_string(),
            name: "Analysis of Electrical Circuits".to_string(),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 1,
                room: "C26".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (4, 5),
        },
        Class {
            group: Some(0),
            kind: ClassType::Tutorial(S::One, P::None),
            code: "CSE 127".to_string(),
            name: "Data Structures I".to_string(),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 2,
                room: "C44".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (6, 6),
        },
        Class {
            group: Some(0),
            kind: ClassType::Tutorial(S::One, P::None),
            code: "EMP x19".to_string(),
            name: "Probability and Statistics".to_string(),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 0,
                room: "C11".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (7, 7),
        },
        Class {
            group: Some(0),
            kind: ClassType::Lecture("أ.د.م. أحمد التراس".to_string()),
            code: "TRN x21".to_string(),
            name: "Technical Writing".to_string(),
            location: ClassLocation {
                building: Building::Ssp,
                floor: 2,
                room: "C40".to_string(),
            },
            day_of_week: DayOfWeek::Tuesday,
            period: (8, 9),
        },
        Class {
            group: Some(0),
            kind: ClassType::Tutorial(S::One, P::None),
            code: "EMP 116".to_string(),
            name: "Differential Equations".to_string(),
            location: ClassLocation {
                building: Building::PreparatorySouth,
                floor: 1,
                room: "C8".to_string(),
            },
            day_of_week: DayOfWeek::Wednesday,
            period: (0, 0),
        },
        Class {
            group: Some(0),
            kind: ClassType::Lecture("أ.د. عمرو عبد الرازاق".to_string()),
            code: "EMP 116".to_string(),
            name: "Differential Equations".to_string(),
            location: ClassLocation {
                building: Building::PreparatoryNorth,
                floor: 0,
                room: "L5".to_string(),
            },
            day_of_week: DayOfWeek::Wednesday,
            period: (3, 5),
        },
    ]
}
