use leptos::*;

#[component]
pub fn Input(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(default = false)] required: bool,
    #[prop(optional, into)] attributes: Option<MaybeSignal<AdditionalAttributes>>,
) -> impl IntoView {
    let mut input = view! {
        <input
            class="peer p-2 border border-gray-300 dark:border-gray-500 rounded \
                focus:!border-blue-500 outline-none w-full"
            id=&id
            name=&id
            required=required
            placeholder=" "
        />
    };

    if let Some(attributes) = attributes {
        let attributes = attributes.get();
        for (attr_name, attr_value) in attributes.into_iter() {
            let attr_name = attr_name.to_owned();
            let attr_value = attr_value.to_owned();
            input = input.attr(attr_name, move || attr_value.get());
        }
    }

    view! {
        <div class="relative w-full">
            {input}
            <label for=id
                class="input_label cursor-text absolute bottom-2 left-2 \
                    text-gray bg-secondary origin-bottom-left transition-transform"
            >
            {label}
            </label>
        </div>
    }
}
