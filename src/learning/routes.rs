use leptos::{component, view, IntoView};
use leptos_router::Route;
use crate::learning::skill_page::SkillPage;

#[component(transparent)]
pub fn LearningRoutes() -> impl IntoView {
  view! {
    <Route path="/learning" view=SkillPage />
  }
}