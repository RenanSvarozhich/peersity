use leptos::{component, create_signal, view, IntoView, SignalUpdate};

#[component]
pub fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1 class="p-6 text-4xl">"Home"</h1>
        <button class="rounded-lg" on:click=on_click>
            "Click Me: " {count}
        </button>
    }
}