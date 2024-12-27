use std::{collections::HashMap, iter};

use serde::Serialize;

pub enum Grade {
    APlus,
    A,
    AMinus,
    BPlus,
    B,
    BMinus,
    CPlus,
    C,
    CMinus,
    DPlus,
    D,
    F,
    Withdrawal,
    ForcedWithdrawal,
    MilitaryWithdrawal,
    Incomplete,
    Audit,
    InProgress,
    Absent,
    Excuse,
    Pass,
    Fail,
}

#[derive(sqlx::Type, Debug, Serialize)]
#[sqlx(type_name = "term_season", rename_all = "lowercase")]
pub enum Season {
    Fall,
    Spring,
    Summer,
}

#[derive(Clone, Copy)]
pub struct Score(u8, u8);

pub struct CourseGrade {
    id: u64,
    code: String,
    name: String,
    credit: u8,
    score: Score,
    detailed_score: HashMap<String, Score>,
}

impl<'a> iter::Sum<&'a Score> for Score {
    fn sum<I: Iterator<Item = &'a Score>>(iter: I) -> Self {
        iter.fold(Score(0, 0), |acc, x| Score(acc.0 + x.0, acc.1 + x.1))
    }
}

impl From<(u8, u8)> for Score {
    fn from((score, max): (u8, u8)) -> Self {
        Self(score, max)
    }
}

impl CourseGrade {
    pub fn new(
        id: u64,
        code: String,
        name: String,
        credit: u8,
        detailed_score: Vec<(String, Score)>,
    ) -> Self {
        Self {
            id,
            code,
            name,
            credit,
            score: detailed_score.iter().map(|(_, s)| s).sum(),
            detailed_score: HashMap::from_iter(detailed_score.into_iter()),
        }
    }
}

pub struct TermGrades {
    year: u16,
    season: Season,
    courses: Vec<CourseGrade>,
}

impl TermGrades {
    pub fn new(year: u16, season: Season, courses: Vec<CourseGrade>) -> Self {
        Self {
            year,
            season,
            courses,
        }
    }
}

impl Grade {
    pub fn gpa(&self) -> Option<f32> {
        match self {
            Grade::APlus | Grade::A => Some(4.0),
            Grade::AMinus => Some(3.7),
            Grade::BPlus => Some(3.3),
            Grade::B => Some(3.0),
            Grade::BMinus => Some(2.7),
            Grade::CPlus => Some(2.3),
            Grade::C => Some(2.0),
            Grade::CMinus => Some(1.7),
            Grade::DPlus => Some(1.3),
            Grade::D => Some(1.0),
            Grade::F => Some(0.0),
            Grade::ForcedWithdrawal | Grade::Absent => Some(0.0),
            Grade::Pass
            | Grade::Fail
            | Grade::Withdrawal
            | Grade::MilitaryWithdrawal
            | Grade::Incomplete
            | Grade::Audit
            | Grade::InProgress
            | Grade::Excuse => None,
        }
    }
}

impl From<Grade> for &'static str {
    fn from(val: Grade) -> Self {
        match val {
            Grade::APlus => "A+",
            Grade::A => "A",
            Grade::AMinus => "A-",
            Grade::BPlus => "B+",
            Grade::B => "B",
            Grade::BMinus => "B-",
            Grade::CPlus => "C+",
            Grade::C => "C",
            Grade::CMinus => "C-",
            Grade::DPlus => "D+",
            Grade::D => "D",
            Grade::F => "F",
            Grade::Withdrawal => "W",
            Grade::ForcedWithdrawal => "FW",
            Grade::MilitaryWithdrawal => "MW",
            Grade::Incomplete => "I",
            Grade::Audit => "AU",
            Grade::InProgress => "IP",
            Grade::Absent => "Abs",
            Grade::Excuse => "E",
            Grade::Pass => "P",
            Grade::Fail => "F",
        }
    }
}
