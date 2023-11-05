use leptos::*;
use serde::{Deserialize, Serialize};
// TODO:
// - [ ] Add a grades page
//     - [ ] Add a resource
// - [ ] Cards for each term
// - [ ] Server
//     - [ ] Db stuff
//     - [ ] Org into struct
//     - [ ] Handle repeated subjects

#[derive(Serialize, Deserialize)]
pub struct Grades {}

#[server]
async fn get_std_grades() -> Result<Grades, ServerFnError> {
    todo!()
}

#[component]
pub fn GradesPage() -> impl IntoView {
    view! {
        <div>
            "Grades"
        </div>
    }
}
