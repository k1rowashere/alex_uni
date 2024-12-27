use actix_identity::Identity;
use actix_web::*;
use error::ErrorInternalServerError;
use web::Data;

use crate::{auth, model::class::*, Database, TEMPLATES};

#[get("/schedule/classes")]
pub async fn class_schedule(id: Identity, db: Data<Database>) -> impl Responder {
    // TODO: use a middleware for user info
    let user = auth::get_user(&id.id()?, &db)
        .await
        .map_err(ErrorInternalServerError)?;

    // TODO: get classes from db

    let classes = [
        Class {
            id: 1,
            ctype: Type::Tutorial {
                sec_no: 1,
                week_parity: WeekParity::Even,
            },
            code: "CSE 376".to_string(),
            name: "Analog Communication".to_string(),
            location: Location {
                building: Building::Electricity,
                floor: 4,
                room: "C402".to_string(),
            },
            day: DayOfWeek::Saturday,
            start_period: 0,
            end_period: 1,
        },
        Class {
            id: 1,
            ctype: Type::Tutorial {
                sec_no: 1,
                week_parity: WeekParity::Odd,
            },
            code: "CSE 376".to_string(),
            name: "Database Systems".to_string(),
            location: Location {
                building: Building::Electricity,
                floor: 4,
                room: "C402".to_string(),
            },
            day: DayOfWeek::Saturday,
            start_period: 0,
            end_period: 1,
        },
        Class {
            id: 2,
            ctype: Type::Tutorial {
                sec_no: 1,
                week_parity: WeekParity::Odd,
            },
            code: "CSE 326".to_string(),
            name: "Analysis and Design of Algorithms".to_string(),
            location: Location {
                building: Building::Electricity,
                floor: 4,
                room: "C402".to_string(),
            },
            day: DayOfWeek::Saturday,
            start_period: 2,
            end_period: 3,
        },
        Class {
            id: 4,
            ctype: Type::Lecture {
                prof: "عمرو المصري".to_string(),
            },
            code: "CSE 326".to_string(),
            name: "Analysis and Design of Algorithms".to_string(),
            location: Location {
                building: Building::Mechanics,
                floor: 4,
                room: "H401".to_string(),
            },
            day: DayOfWeek::Saturday,
            start_period: 5,
            end_period: 7,
        },
        Class {
            id: 5,
            ctype: Type::Lecture {
                prof: "محمد عبد الحكم".to_string(),
            },
            code: "BUS 342".to_string(),
            name: "Entrepreneurship".to_string(),
            location: Location {
                building: Building::Ssp,
                floor: 1,
                room: "C103".to_string(),
            },
            day: DayOfWeek::Saturday,
            start_period: 8,
            end_period: 9,
        },
        Class {
            id: 6,
            ctype: Type::Lab {
                sec_no: 1,
                week_parity: WeekParity::Both,
            },
            code: "CSE 326".to_string(),
            name: "Analysis and Design of Algorithms".to_string(),
            location: Location {
                building: Building::Electricity,
                floor: 0,
                room: "Lab 7".to_string(),
            },
            day: DayOfWeek::Sunday,
            start_period: 4,
            end_period: 5,
        },
        Class {
            id: 7,
            ctype: Type::Lecture {
                prof: "أ.د.م.يسري طه".to_string(),
            },
            code: "CSE 376".to_string(),
            name: "Database Systems".to_string(),
            location: Location {
                building: Building::Ssp,
                floor: 1,
                room: "C102".to_string(),
            },
            day: DayOfWeek::Sunday,
            start_period: 10,
            end_period: 11,
        },
        Class {
            id: 8,
            ctype: Type::Lecture {
                prof: "أ.د.م.مروان تركي".to_string(),
            },
            code: "CSE 359".to_string(),
            name: "Introduction to Machine Learning".to_string(),
            location: Location {
                building: Building::Mechanics,
                floor: 3,
                room: "H301".to_string(),
            },
            day: DayOfWeek::Tuesday,
            start_period: 0,
            end_period: 1,
        },
        Class {
            id: 9,
            ctype: Type::Lab {
                sec_no: 1,
                week_parity: WeekParity::Both,
            },
            code: "CSE 376".to_string(),
            name: "Database Systems".to_string(),
            location: Location {
                building: Building::PreparatoryNorth,
                floor: 3,
                room: "Lab 4".to_string(),
            },
            day: DayOfWeek::Wednesday,
            start_period: 2,
            end_period: 3,
        },
        Class {
            id: 10,
            ctype: Type::Lecture {
                prof: "د.سامية حافظ".to_string(),
            },
            code: "CSE 366".to_string(),
            name: "Operating Systems".to_string(),
            location: Location {
                building: Building::Mechanics,
                floor: 5,
                room: "H501".to_string(),
            },
            day: DayOfWeek::Wednesday,
            start_period: 4,
            end_period: 5,
        },
        Class {
            id: 11,
            ctype: Type::Lab {
                sec_no: 1,
                week_parity: WeekParity::Both,
            },
            code: "CSE 366".to_string(),
            name: "Operating Systems".to_string(),
            location: Location {
                building: Building::Electricity,
                floor: 0,
                room: "Lab 7".to_string(),
            },
            day: DayOfWeek::Wednesday,
            start_period: 6,
            end_period: 7,
        },
        Class {
            id: 12,
            ctype: Type::Tutorial {
                sec_no: 1,
                week_parity: WeekParity::Both,
            },
            code: "CSE 359".to_string(),
            name: "Introduction to Machine Learning".to_string(),
            location: Location {
                building: Building::PreparatorySouth,
                floor: 3,
                room: "C305".to_string(),
            },
            day: DayOfWeek::Thursday,
            start_period: 6,
            end_period: 7,
        },
        Class {
            id: 13,
            ctype: Type::Lab {
                sec_no: 1,
                week_parity: WeekParity::Both,
            },
            code: "CSE 359".to_string(),
            name: "Introduction to Machine Learning".to_string(),
            location: Location {
                building: Building::PreparatorySouth,
                floor: 3,
                room: "C306".to_string(),
            },
            day: DayOfWeek::Thursday,
            start_period: 8,
            end_period: 9,
        },
        Class {
            id: 14,
            ctype: Type::Lecture {
                prof: "د.محمد النقيب, د.نسمة عبد المجيد, أ.د.أحمد بيومي".to_string(),
            },
            code: "HUM x51".to_string(),
            name: "Issues of Energy, Water and Climate Change".to_string(),
            location: Location {
                building: Building::Ssp,
                floor: 2,
                room: "C201".to_string(),
            },
            day: DayOfWeek::Thursday,
            start_period: 10,
            end_period: 11,
        },
    ];

    let mut classes_grouped: [_; 6] = std::array::from_fn(|_| Vec::new());
    classes
        .chunk_by(|a, b| {
            (a.day, a.start_period, b.end_period) == (b.day, b.start_period, b.end_period)
        })
        .for_each(|grp| classes_grouped[grp[0].day as usize].push(grp));

    let mut ctx = tera::Context::new();
    ctx.insert("user", &user);
    ctx.insert("grid", &classes_grouped);
    ctx.insert("time_style", &"12h");

    TEMPLATES
        .render("class_schedule.html", &ctx)
        .inspect_err(|e| eprintln!("{:?}", e))
        .map_err(error::ErrorInternalServerError)
        .map(web::Html::new)
}
