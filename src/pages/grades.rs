use std::sync::LazyLock;

use actix_identity::Identity;
use actix_web::*;
use error::ErrorInternalServerError;
use http::header::{CacheControl, CacheDirective};
use serde_json::json;
use web::{Data, Query};

use crate::model::term::Season;
use crate::{auth, Database, TEMPLATES};

pub fn compute_cgpa() -> f32 {
    todo!()
}

#[get("parts/calc_item")]
pub async fn calc_item() -> impl Responder {
    static CALC_ITEM: LazyLock<String> = LazyLock::new(|| {
        TEMPLATES
        .render_str(
            r#"
            {% import "components/icon.html" as ico %}
            {% import "components/input.html" as inp %}
            <li class="col-span-full grid grid-cols-subgrid opacity-100 transition-opacity [.htmx-swapping]:opacity-0 [.htmx-added]:opacity-0">
              <button type="button"
                      hx-get="data:text/html,"
                      hx-swap="delete swap:.2s"
                      hx-target="closest li"
                      class="justify-self-center rounded p-1 text-red-700 ring-red-300 hover:text-red-400 disabled:text-slate-500">
                {{ ico::icon(icon="FiX", nomargin=true) }}
              </button>
              {{ inp::input(name=`course`, display_name=`Course`) }} 
              {{ inp::input(class="w-full", name=`credit`, display_name=`Credit Hours`, type=`number`, attrs=`min=0`) }}
              {{ inp::select(name=`grade`, display_name=`Grade`,
                  options=['A+', 'A', 'A-', 'B+', 'B', 'B-', 'C+', 'C', 'C-', 'D+', 'D', 'F'],
                  values=[4.0, 4.0, 3.7, 3.3, 3.0, 2.7, 2.3, 2.0, 1.7, 1.3, 1.0, 0.0]) }}
            </li>
            "#,
            &tera::Context::new(),
        ).expect("Failed to render calc_item")
    });

    web::Html::new(CALC_ITEM.as_str())
        .customize()
        .append_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(u32::MAX),
        ]))
}

// pub async fn get_grades(user_id: i32, db: &Database) -> sqlx::Result<term::TermGrades> {
//     let grades = sqlx::query!(
//         r#"
//             SELECT term_year, term_season,
//             array_agg((code, name, credit, score))
//             FROM
//         "#,
//         user_id
//     )
//     .fetch_all(db)
//     .await?;
//
//     todo!()
// }

pub async fn get_grades(user_id: Query<i32>, db: Data<Database>) -> impl Responder {
    #[derive(Debug, sqlx::Type, serde::Serialize)]
    struct Grade {
        code: String,
        name: String,
        credit: i32,
        score: i32,
    }

    sqlx::query!(
        r#"
            SELECT
              term_year,
              term_season AS "season:Season",
              array_agg((code, name, credit, score)) AS "courses!:Vec<Grade>"
            FROM
              completed as c
              JOIN courses as cr ON c.course_id = cr.id
            WHERE
              student_id = $1
            GROUP BY
              term_year,
              term_season
        "#,
        user_id.0
    )
    .fetch_all(db.as_ref())
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|term_grades| {
        let mut ctx = tera::Context::new();
        let term_grades: Vec<_> = term_grades
            .iter()
            .map(|grade| json!({ "year": grade.term_year, "season": grade.season, "courses": grade.courses }))
            .collect();
        ctx.insert("terms", &term_grades);
        TEMPLATES
            .render("components/grades.html", &ctx)
            .map_err(ErrorInternalServerError)
    })
    .map(web::Html::new)
}

#[get("/grades")]
pub async fn grades(id: Identity, db: Data<Database>) -> impl Responder {
    // TODO: use a middleware for user info
    let user = auth::get_user(&id.id()?, &db)
        .await
        .map_err(ErrorInternalServerError)?;

    // TODO: get grades from db
    //

    let mut ctx = tera::Context::new();
    ctx.insert("user", &user);
    // ctx.insert("grid", &grades);

    TEMPLATES
        .render("grades.html", &ctx)
        .inspect_err(|e| eprintln!("{:?}", e))
        .map_err(ErrorInternalServerError)
        .map(web::Html::new)
}
