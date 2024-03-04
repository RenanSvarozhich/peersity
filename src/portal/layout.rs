use leptos::{component, view, Children, IntoView};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <main class="container mx-auto px-4">
            {children()}
        </main>
    }
}