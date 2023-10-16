#![allow(clippy::needless_lifetimes)]
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::and_then;

const LABEL_CLASS: &str = "dot_grid font-bold";
const GRID_CLASS: &str =
    "ml-2 grid md:grid-cols-4 max-md:grid-cols-2 content-center";

#[derive(Serialize, Deserialize)]
pub struct ProfileInfo {
    student_info: StudentInfo,
    guardian_info: GuardianInfo,
    qualification: QualInfo,
}

#[derive(Serialize, Deserialize)]
struct StudentInfo {
    name_en: String,
    name_ar: String,
    program_name: String,
    gender: Option<String>,
    birth_date: Option<String>,
    birth_place: Option<String>,
    nationality: String,
    national_id: Option<String>,
    city: Option<String>,
    contact_info: ContactInfo,
}

#[derive(Serialize, Deserialize)]
struct GuardianInfo {
    name: Option<String>,
    occupation: Option<String>,
    contact_info: ContactInfo,
}

#[derive(Serialize, Deserialize)]
struct QualInfo {
    school: Option<String>,
    qualification: Option<String>,
    graduation_year: Option<i64>,
    score: Option<i64>,
    percent: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct ContactInfo {
    phone_no: Option<String>,
    mobile_no: Option<String>,
    email: Option<String>,
    address: Option<String>,
}

#[server(,,"GetJson")]
async fn get_std_perosonal_info() -> Result<ProfileInfo, ServerFnError> {
    let req = expect_context::<actix_web::HttpRequest>();
    let pool = req
        .app_data::<sqlx::Pool<sqlx::Sqlite>>()
        .expect("Expected SqlitePool");

    let student_id = if let Some(sid) = crate::login::user_id_from_jwt(&req) {
        sid
    } else {
        expect_context::<leptos_actix::ResponseOptions>()
            .set_status(actix_web::http::StatusCode::UNAUTHORIZED);
        return Err(ServerFnError::ServerError("Auth Error".into()));
    };

    // TODO: Encrypt using user

    let q = sqlx::query!(
        r#"
        SELECT
           sp.*,
           p.name as program_name,
           p.code as program_code
        FROM student_profile AS sp
        INNER JOIN users AS u ON sp.id = u.profile_id
        INNER JOIN programs AS p ON sp.program_id = p.id
        WHERE u.id = ?
        "#,
        student_id
    )
    .fetch_one(pool)
    .await?;

    let p = ProfileInfo {
        student_info: StudentInfo {
            name_en: q.name_en,
            name_ar: q.name_ar,
            program_name: q.program_name,
            gender: q.gender,
            birth_date: q.birth_date.map(|d| d.to_string()),
            birth_place: q.birth_place,
            nationality: q.nationality,
            national_id: q.national_id,
            city: q.city,
            contact_info: ContactInfo {
                phone_no: q.phone_no,
                mobile_no: q.mobile_no,
                email: q.email,
                address: q.address,
            },
        },
        qualification: QualInfo {
            school: q.prev_school,
            qualification: q.prev_qualification,
            graduation_year: q.prev_graduation_year,
            score: q.prev_score,
            percent: q.prev_percent,
        },
        guardian_info: GuardianInfo {
            name: q.guardian_name,
            occupation: q.guardian_occupation,
            contact_info: ContactInfo {
                phone_no: q.guardian_phone_no,
                mobile_no: q.guardian_mobile_no,
                email: q.guardian_email,
                address: q.guardian_address,
            },
        },
    };

    Ok(p)
}

#[component]
pub fn profile_page() -> impl IntoView {
    let profile = create_resource(|| (), |_| get_std_perosonal_info());
    view! {
        <h1 class="text-4xl mb-7">"Student Profile"</h1>
        <div class="flex flex-col gap-4">
            <Suspense fallback=|| ()>
                <ErrorBoundary fallback=|_| ()>
                    {and_then!(|profile| {
                        view! {
                            <Personal info=&profile.student_info/>
                            <section>
                                <h2 class="text-2xl">"Contact Info"</h2>
                                <div class=GRID_CLASS>
                                    <Contact info=&profile.student_info.contact_info/>
                                </div>
                            </section>
                            <Parent info=&profile.guardian_info/>
                            <Education info=&profile.qualification/>
                        }
                    })}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}

#[component]
fn personal<'a>(info: &'a StudentInfo) -> impl IntoView {
    view! {
        <section>
            <h2 class="text-2xl">"Personal Info"</h2>
            <div class=GRID_CLASS>
                <span class=LABEL_CLASS>"Name (Arabic):"</span>
                <span>{&info.name_ar}</span>
                <span class=LABEL_CLASS>"Name (English):"</span>
                <span>{&info.name_en}</span>
                <span class=LABEL_CLASS>"Nationality:"</span>
                <span>{&info.nationality}</span>
                <span class=LABEL_CLASS>"Gender:"</span>
                <span>{info.gender.as_ref()}</span>
                <span class="col-span-2"/>
                <span class=LABEL_CLASS>"Date of Birth:"</span>
                <span>{info.birth_date.as_ref()}</span>
                <span class=LABEL_CLASS>"Place of Birth:"</span>
                <span>{info.birth_place.as_ref()}</span>
                // TODO: select between national id or passport depending on the student
                <span class=LABEL_CLASS>"National ID:"</span>
                <span>{info.national_id.as_ref()}</span>
            </div>
        </section>
    }
}

#[component]
fn contact<'a>(info: &'a ContactInfo) -> impl IntoView {
    view! {
        <span class=LABEL_CLASS>"Phone Number"</span>
        <span>{info.phone_no.as_ref()}</span>
        <span class=LABEL_CLASS>"Mobile Number"</span>
        <span>{info.mobile_no.as_ref()}</span>
        <span class=LABEL_CLASS>"Email"</span>
        <span>{info.email.as_ref()}</span>
        <span class=LABEL_CLASS>"Address"</span>
        <span>{info.address.as_ref()}</span>
    }
}

#[component]
fn parent<'a>(info: &'a GuardianInfo) -> impl IntoView {
    view! {
        <section>
            <h2 class="text-2xl">"Parent / Guardian Info"</h2>
            <div class=GRID_CLASS>
                <span class=LABEL_CLASS>"Name"</span>
                <span>{info.name.as_ref()}</span>
                <span class=LABEL_CLASS>"Occupation"</span>
                <span>{info.occupation.as_ref()}</span>
                <span class="col-span-2"/>
                <Contact info=&info.contact_info/>
            </div>
        </section>
    }
}

#[component]
fn education<'a>(info: &'a QualInfo) -> impl IntoView {
    view! {
        <section>
            <h2 class="text-2xl">"Prior Education"</h2>
            <div class=GRID_CLASS>
                <span class=LABEL_CLASS>"Degree"</span>
                <span>{info.qualification.as_ref()}</span>
                <span class=LABEL_CLASS>"School"</span>
                <span>{info.school.as_ref()}</span>
                <span class=LABEL_CLASS>"Graduation year"</span>
                <span>{info.graduation_year}</span>
                <span class=LABEL_CLASS>"Final Score"</span>
                <span>{info.score}</span>
                <span class=LABEL_CLASS>"Percentage"</span>
                <span>{info.percent.map(|p| format!("{p:.2}%"))}</span>
                // <span class=LABEL_CLASS>"Seating №"</span>
                // <span>"VALUE"</span>
                // <span class=LABEL_CLASS>"Application Acceptance №"</span>
                // <span>"VALUE"</span>
                // <span class=LABEL_CLASS>"Acceptance Date"</span>
                // <span>"VALUE"</span>
            </div>
        </section>
    }
}
