pub mod grid;
mod list;
mod timetable;

pub use crate::class::*;
pub use timetable::*;

pub const PERIOD_START_TIME: [&str; 12] = [
    "08:30 AM", "09:20 AM", "10:20 AM", "11:10 AM", "12:10 PM", "01:00 PM",
    "02:00 PM", "02:50 PM", "03:50 PM", "04:40 PM", "05:40 PM", "06:30 PM",
];

pub const PERIOD_END_TIME: [&str; 12] = [
    "09:20 AM", "10:10 AM", "11:10 AM", "12:00 PM", "01:00 PM", "01:50 PM",
    "02:50 PM", "03:40 PM", "04:40 PM", "05:30 PM", "06:30 PM", "07:20 PM",
];

trait ToBgClass {
    fn to_bg_color(&self) -> &'static str;
}

impl ToBgClass for ClassType {
    fn to_bg_color(&self) -> &'static str {
        match self {
            ClassType::Lecture(_) => "dark:bg-red-900 bg-red-200",
            ClassType::Lab(_, _) => "dark:bg-cyan-800 bg-cyan-200",
            ClassType::Tutorial(_, _) => "dark:bg-gray-800 bg-gray-200",
        }
    }
}
