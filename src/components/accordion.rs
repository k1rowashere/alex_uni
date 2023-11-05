use crate::icon;
use leptos::*;
use uuid::Uuid;

#[derive(Clone)]
struct Context(RwSignal<Option<Uuid>>);

#[component]
pub fn Accordion(children: Children) -> impl IntoView {
    provide_context(Context(create_rw_signal(None)));

    view! {
        <ul class="flex-grow flex flex-col gap-2">
            {children()}
        </ul>
    }
}

#[component]
pub fn AccordionItem<F, IV>(
    head: F,
    #[prop(default = true)] interlocking: bool,
    #[prop(optional)] start_open: bool,
    children: Children,
    #[prop(optional, into)] class: String,
) -> impl IntoView
where
    F: FnOnce() -> IV + 'static,
    IV: IntoView,
{
    let id = Uuid::new_v4();
    let id_str = id.to_string();

    let (open, on_click): (Signal<_>, Box<dyn FnMut(_)>) = {
        if interlocking {
            let Context(open_id) = use_context()
            .expect("`AccordionItem` must be a child of `Accordion`, if `interlocking` is true");

            // if start_open {
            //     open_id.set_untracked(Some(id));
            // }
            let open = move || open_id.with(|i| *i == Some(id));

            let on_click = move |_| {
                open_id.update(|i| {
                    *i = if *i == Some(id) { None } else { Some(id) }
                });
            };

            (open.into_signal(), Box::new(on_click))
        } else {
            let (open, set_open) = create_signal(start_open);
            let on_click = move |_| set_open.update(|o| *o = !*o);

            (open.into_signal(), Box::new(on_click))
        }
    };

    let list = NodeRef::<html::Div>::new();
    let get_max_height = move || {
        let mh = match list() {
            Some(list) if open() => list.scroll_height(),
            _ => 0,
        };
        format!("{mh}px")
    };

    view! {
        <li class=class + " p-2 flex flex-col items-center border rounded">
            <button
                type="button"
                class="w-full flex justify-between items-center"
                aria-expanded=move || open().to_string()
                aria-controls=&id_str
                on:click=on_click
            >
                {head()}
                {icon!("mdi/chevron-down", "transition-transform").class("-rotate-180", open)}
            </button>
            <div
                ref=list
                id=&id_str
                class="transition-[opacity,_max-height] overflow-hidden w-full"
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
