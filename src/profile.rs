use leptos::*;
const LABEL_CLASS: &str = "dot_grid font-bold";
const GRID_CLASS: &str = "ml-2 grid md:grid-cols-4 max-md:grid-cols-2";

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <h1 class="text-4xl mb-7">"Student Profile"</h1>
        <div class="flex flex-col gap-4">
            <Personal/>
            <section>
                <h2 class="text-2xl">"Contact Info"</h2>
                <div class=GRID_CLASS>
                    <Contact/>
                </div>
            </section>
            <Parent/>
            <Education/>
        </div>
    }
}

#[component]
fn Personal() -> impl IntoView {
    view! {
        <section>
            <h2 class="text-2xl">"Personal Info"</h2>
            <div class=GRID_CLASS>
                <span class=LABEL_CLASS>"Name (Arabic):"</span>
                <span>"اختبار اختبار اختبار اختبار"</span>
                <span class=LABEL_CLASS>"Name (English):"</span>
                <span>"Test Test Test Test "</span>
                <span class=LABEL_CLASS>"Nationality:"</span>
                <span>"Egyptian"</span>
                <span class=LABEL_CLASS>"Gender:"</span>
                <span>"Attack Heli"</span>
                <span class=LABEL_CLASS>"Religion:"</span>
                <span>"religon"</span>
                <span class="col-span-2"/>
                <span class=LABEL_CLASS>"Date of Birth:"</span>
                <span>"11/09/2001"</span>
                <span class=LABEL_CLASS>"Place of Birth:"</span>
                <span>"Alexandria"</span>
                // TODO: select between national id or passport depending on the student
                <span class=LABEL_CLASS>"National ID:"</span>
                <span>"0123456789"</span>
                <span class=LABEL_CLASS>"Issue Date:"</span>
                <span>"00/00/0000"</span>
                <span class=LABEL_CLASS>"Issue Place:"</span>
                <span>"Cairo"</span>
            </div>
        </section>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <span class=LABEL_CLASS>"Phone Number"</span>
        <span>"VALUE"</span>
        <span class=LABEL_CLASS>"Mobile Number"</span>
        <span>"VALUE"</span>
        <span class=LABEL_CLASS>"Email"</span>
        <span>"VALUE"</span>
        <span class=LABEL_CLASS>"Address"</span>
        <span>"VALUE"</span>
    }
}

#[component]
fn Parent() -> impl IntoView {
    view! {
        <section>
            <h2 class="text-2xl">"Parent / Guardian Info"</h2>
            <div class=GRID_CLASS>
                <span class=LABEL_CLASS>"Name"</span>
                <span>"VALUE"</span>
                <span class=LABEL_CLASS>"Profession"</span>
                <span>"VALUE"</span>
                <span class=LABEL_CLASS>"City of Residence"</span>
                <span>"VALUE"</span>
                <span class="col-span-2"/>
                <Contact/>
            </div>
        </section>
    }
}

#[component]
fn Education() -> impl IntoView {
    view! {
        <section>
            <h2 class="text-2xl">"Prior Education"</h2>
            <div class=GRID_CLASS>
                <span class=LABEL_CLASS>"Degree"</span>
                <span>"VALUE"</span>
                <span class=LABEL_CLASS>"School"</span>
                <span>"VALUE"</span>
                <span class=LABEL_CLASS>"Graduation year"</span>
                <span>"VALUE"</span>
                <span class=LABEL_CLASS>"Final Score"</span>
                <span>"VALUE"</span>
                <span class=LABEL_CLASS>"Percentage"</span>
                <span>"VALUE"</span>
                <span class=LABEL_CLASS>"Seating №"</span>
                <span>"VALUE"</span>
                <span class=LABEL_CLASS>"Application Acceptance №"</span>
                <span>"VALUE"</span>
                <span class=LABEL_CLASS>"Acceptance Date"</span>
                <span>"VALUE"</span>
            </div>
        </section>
    }
}
