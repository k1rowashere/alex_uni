use leptos::*;
#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <h1 class="text-5xl">"Student Info"</h1>
        <PersonalInfo/>
        <ConatactInfo/>
        <ParentInfo/>
    }
}

#[component]
fn PersonalInfo() -> impl IntoView {
    view! {
        <h2>"Personal Info"</h2>
        <table class="gap-4">
            <tbody>
                <tr>
                    <td>"Name (Arabic)"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Name (English)"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Nationality"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Gender"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Religion"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Date of Birth"</td>
                    <td>"VALUE"</td>
                    <td>"Place of Birth"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"National ID/Passport"</td>
                    <td>"VALUE"</td>
                    <td>"Issue Date"</td>
                    <td>"VALUE"</td>
                    <td>"Issue Place"</td>
                    <td>"VALUE"</td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
fn ConatactInfo() -> impl IntoView {
    view! {
        <h2>"Contact Info"</h2>
        <table>
            <tbody>
                <tr>
                    <td>"Phone Number"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Mobile Number"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Email"</td>
                    <td>"VALUE"</td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
fn ParentInfo() -> impl IntoView {
    view! {
        <h2>"Parental/Guardian Info"</h2>
        <table>
            <tbody>
                <tr>
                    <td>"Guardian Name"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Profession"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"City of Residence"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Address"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Phone Number"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Email"</td>
                    <td>"VALUE"</td>
                </tr>
                <tr>
                    <td>"Mobile Number"</td>
                    <td>"VALUE"</td>
                </tr>
            </tbody>
        </table>
    }
}
