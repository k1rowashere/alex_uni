use crate::icon;
use leptos::*;

#[derive(Clone)]
struct Context(RwSignal<Option<usize>>, &'static str);

#[component]
pub fn accordion(children: Children) -> impl IntoView {
    // TODO: Allow for "interlocking" accordion items

    // provide_context(Context(create_rw_signal::<Option<usize>>(None), id));
    view! {
        <ul class="flex-grow flex flex-col gap-1">
            {children()}
        </ul>
    }
}

#[component]
pub fn accordion_item<F, IV>(head: F, children: Children) -> impl IntoView
where
    F: FnOnce() -> IV + 'static,
    IV: IntoView,
{
    let id = uuid::Uuid::new_v4().to_string();
    // let Context(open_idx, main_id) = expect_context();
    let open = create_rw_signal(false);
    let list = create_node_ref::<html::Div>();
    let get_max_height = move || {
        let mh = match list() {
            Some(list) if open() => list.scroll_height(),
            _ => 0,
        };
        format!("{mh}px")
    };

    view! {
        <li class="p-2 flex flex-col items-center border rounded">
            <button
                type="button"
                class="w-full flex justify-between"
                aria-expanded=move || open().to_string()
                aria-controls=&id
                on:click=move |_| open.update(|o| *o = !*o)
            >
                {head()}
                {icon!("mdi/chevron-down", "transition-transform").class("-rotate-180", open)}
            </button>
            <div
                ref=list
                id=&id
                class="flex flex-col overflow-hidden transition-all"
                class:opacity-0=move || !open()
                class:invisible=move || !open()
                style:max-height=get_max_height
                aria-hidden=move || open().to_string()
            >
                {children()}
            </div>
        </li>
    }
}
