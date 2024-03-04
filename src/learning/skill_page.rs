use leptos::{component, create_signal, view, IntoView, SignalUpdate};

#[component]
pub fn SkillPage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1 class="p-6 text-4xl">"Learning"</h1>
        <button class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg" 
            on:click=on_click>
            "Click Me: " {count}
        </button>
    }
}