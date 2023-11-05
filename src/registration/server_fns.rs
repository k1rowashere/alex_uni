use std::collections::BTreeSet;

use super::{SubjectChoices, SubjectId};
use leptos::*;

#[cfg(feature = "ssr")]
use super::{rem_seats_ws::RemSeatsMsg, Subject};

#[cfg(feature = "ssr")]
#[cached::proc_macro::cached(time = 1000, time_refresh, result)]
pub(super) async fn subject_by_id(
    s: SubjectId,
) -> sqlx::Result<Option<Subject>> {
    use crate::class::*;

    let req = expect_context::<actix_web::HttpRequest>();
    let pool = req
        .app_data::<sqlx::Pool<sqlx::Sqlite>>()
        .expect("DB ctx not found");

    let (group, max_seats) = {
        let query = sqlx::query!(
            r#"
                SELECT 
                    ts.group_no,
                    ts.max_seats
                FROM term_subjects AS ts 
                WHERE ts.id = ?
            "#,
            s
        )
        .fetch_one(pool)
        .await?;

        (query.group_no as u8, query.max_seats as u32)
    };

    let Some(lec): Option<Class> = sqlx::query_as!(
        crate::class::db::ClassRow,
        r#"
            SELECT cv.* FROM classes_view AS cv 
            INNER JOIN term_subjects AS ts ON cv.id = ts.lec_id
            WHERE ts.id = ?
        "#,
        s
    )
    .fetch_optional(pool)
    .await?
    .map(|c| c.into()) else {
        return Ok(None);
    };

    let tut: Option<Class> = sqlx::query_as!(
        crate::class::db::ClassRow,
        r#"
            SELECT cv.* FROM classes_view AS cv 
            INNER JOIN term_subjects AS ts ON cv.id = ts.tut_id
            WHERE ts.id = ?
        "#,
        s
    )
    .fetch_optional(pool)
    .await?
    .map(|c| c.into());

    let lab: Option<Class> = sqlx::query_as!(
        crate::class::db::ClassRow,
        r#"
            SELECT cv.* FROM classes_view AS cv 
            INNER JOIN term_subjects AS ts ON cv.id = ts.lab_id
            WHERE ts.id = ?
        "#,
        s
    )
    .fetch_optional(pool)
    .await?
    .map(|c| c.into());

    // guaranteed to be valid by the db
    assert!(lec.ctype.is_lecture());
    assert!(tut.is_none() || tut.as_ref().unwrap().ctype.is_tutorial());
    assert!(lab.is_none() || lab.as_ref().unwrap().ctype.is_lab());

    Ok(Some(Subject {
        id: s,
        max_seats,
        group,
        lec,
        tut,
        lab,
    }))
}

/// Returns the remaining seats for the given subjects
/// if `None` given, returns the remaining seats for all subjects
#[cfg(feature = "ssr")]
pub async fn get_rem_seats(
    subjects: &[SubjectId],
    pool: sqlx::SqlitePool,
) -> Result<RemSeatsMsg, ServerFnError> {
    let query = if !subjects.is_empty() {
        let query_str = format!(
            r#"
                SELECT ts.id, 
                    (ts.max_seats - COUNT(tsub.term_subject_id)) as rem_seats
                FROM term_subjects AS ts
                LEFT JOIN term_subscribers AS tsub 
                    ON ts.id = tsub.term_subject_id
                WHERE ts.id IN (?{})
                GROUP BY ts.id
            "#,
            ", ?".repeat(subjects.len() - 1)
        );
        let mut query = sqlx::query_as(&query_str);
        for s in subjects {
            query = query.bind(s);
        }
        query.fetch_all(&pool).await
    } else {
        sqlx::query_as(
            r#"
                SELECT ts.id, 
                    (ts.max_seats - COUNT(tsub.term_subject_id)) as rem_seats
                FROM term_subjects AS ts
                LEFT JOIN term_subscribers AS tsub 
                    ON ts.id = tsub.term_subject_id
                GROUP BY ts.id
            "#,
        )
        .fetch_all(&pool)
        .await
    };

    Ok(RemSeatsMsg(query?))
}

#[server]
pub async fn register_subjects(
    #[server(default)] new: BTreeSet<SubjectId>,
) -> Result<(), ServerFnError> {
    // TODO: Collision detection
    //       Deduping
    //       (preferably on DB):
    //       Completion check
    //       Pre-requirements check
    //       Credit hrs total check
    //       Remaining seats check

    use actix_broker::{Broker, SystemBroker};

    let req = expect_context::<actix_web::HttpRequest>();
    let pool = req
        .app_data::<sqlx::Pool<sqlx::Sqlite>>()
        .expect("DB ctx not found");

    // TODO: move this to a middleware
    let student_id = if let Some(uid) = crate::login::user_id_from_jwt(&req) {
        uid
    } else {
        expect_context::<leptos_actix::ResponseOptions>()
            .set_status(actix_web::http::StatusCode::UNAUTHORIZED);
        return Ok(());
    };

    let diff: Vec<_> = {
        let prev = sqlx::query_scalar!(
            r#"
            SELECT tsub.term_subject_id AS "id: SubjectId"
            FROM term_subscribers as tsub
            WHERE tsub.student_id = ?
        "#,
            student_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .collect();

        new.symmetric_difference(&prev).copied().collect()
    };

    if diff.is_empty() {
        return Ok(());
    }

    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM term_subscribers WHERE student_id = ?")
        .bind(student_id)
        .execute(&mut *tx)
        .await?;

    if !new.is_empty() {
        // Insert into term_subscribers
        // using string formatting because sqlx doesn't support variable length bind params
        let query_str = format!(
            r#"
            INSERT INTO term_subscribers (student_id, term_subject_id) 
            SELECT ?, sid.* 
            FROM (VALUES (?){}) AS sid
            "#,
            ", (?)".repeat(new.len() - 1)
        );

        let mut query = sqlx::query(&query_str);
        query = query.bind(student_id);
        for s in &new {
            query = query.bind(s);
        }

        query.execute(&mut *tx).await?;
    }

    tx.commit().await?;

    // Broadcast `rem_seats` changes to ws actors
    Broker::<SystemBroker>::issue_async(
        get_rem_seats(&diff, pool.clone()).await?,
    );
    Ok(())
}

#[server(encoding = "GetJson")]
pub async fn get_subbed_subjects() -> Result<BTreeSet<SubjectId>, ServerFnError>
{
    use crate::login::user_id_from_jwt;

    let res = expect_context::<leptos_actix::ResponseOptions>();
    let req = expect_context::<actix_web::HttpRequest>();
    let pool = req
        .app_data::<sqlx::Pool<sqlx::Sqlite>>()
        .ok_or(ServerFnError::ServerError("No DB context provided".into()))?;

    let Some(student_id) = user_id_from_jwt(&req) else {
        res.set_status(actix_web::http::StatusCode::UNAUTHORIZED);
        return Ok(BTreeSet::new());
    };

    let query = sqlx::query_scalar!(
        r#"
            SELECT ts.id AS "id: SubjectId" 
            FROM term_subjects as ts
            INNER JOIN term_subscribers as tsub
            ON ts.id = tsub.term_subject_id
            WHERE tsub.student_id = ?;
            "#,
        student_id
    )
    .fetch_all(pool)
    .await?;

    Ok(BTreeSet::from_iter(query))
}

#[server(encoding = "GetJson")]
pub async fn get_registerable_subjects(
) -> Result<Vec<SubjectChoices>, ServerFnError> {
    use futures::{stream, StreamExt, TryStreamExt};

    let req = expect_context::<actix_web::HttpRequest>();
    let pool = req
        .app_data::<sqlx::SqlitePool>()
        .expect("DB ctx not found");

    let student_id = crate::login::user_id_from_jwt(&req);
    // TODO: check if registration is active for student_id

    let subjects_by_id = sqlx::query!(
        r#"
            SELECT s.level AS "level: u8",
                   s.name,
                   s.code,
                   json_group_array(ts.id) AS "choices!: sqlx::types::Json<Vec<SubjectId>>"
            FROM subjects AS s
            INNER JOIN term_subjects AS ts ON ts.subject_id = s.id
            LEFT JOIN completed AS c ON s.id = c.subject_id AND c.student_id = ?1
            WHERE c.student_id IS NULL
              AND NOT EXISTS (
                SELECT value
                FROM json_each(s.pre_req)
                WHERE NOT EXISTS (
                  SELECT *
                  FROM completed AS c2
                  WHERE c2.student_id = ?1
                    AND c2.subject_id = value
                )
              )
            GROUP BY s.id
            ORDER By s.level, s.name;
        "#, student_id)
        .fetch_all(pool)
        .await?;

    let subjects = stream::iter(subjects_by_id)
        .map(|s| async move {
            let choices = stream::iter(s.choices.iter())
                .map(|&s| subject_by_id(s))
                .buffer_unordered(4)
                .try_filter_map(|s| async move { Ok(s) })
                .try_collect()
                .await?;
            Ok(SubjectChoices {
                level: s.level,
                name: s.name,
                code: s.code,
                choices,
            }) as Result<_, sqlx::Error>
        })
        .buffer_unordered(4)
        .try_collect()
        .await?;

    Ok(subjects)
}

#[cfg(test)]
mod test {}
