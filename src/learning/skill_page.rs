use leptos::{component, create_signal, view, For, IntoView};

#[component]
pub fn SkillPage() -> impl IntoView {
    view! {
        <h1 class="text-4xl dark:text-white">Skills</h1>
        <SkillList/>
    }
}

#[component]
fn SkillList() -> impl IntoView {
    let initial_skills = vec![
        "Rust",
        "Java",
        "C#",
        "Python",
        "Javascript",
        "Lua",
    ];

    let (skills, _) = create_signal(initial_skills);

    view! {
        <div>
            <ul>
                <For
                    each=skills
                    key=|skill| skill.to_string()
                    children=move |skill| {
                        view! {
                            <li>
                                {skill}
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}