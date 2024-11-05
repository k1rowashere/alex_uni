use leptos::*;

#[component]
pub fn input(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(default = false)] required: bool,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <div class="relative w-full">
            <input
                class="peer p-2 border border-gray-300 dark:border-gray-500 rounded focus:!border-blue-500 outline-none w-full"
                id=&id
                name=&id
                required=required
                placeholder=" "
                {..attrs}
            />
            <label
                for=id
                class="input_label cursor-text absolute bottom-2 left-2 text-gray bg-secondary origin-bottom-left transition-transform"
            >
                {label}
            </label>
        </div>
    }
}
