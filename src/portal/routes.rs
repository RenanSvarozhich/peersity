use leptos::{component, view, IntoView};
use leptos_router::Route;
use crate::portal::home_page::HomePage;

#[component(transparent)]
pub fn PortalRoutes() -> impl IntoView {
  view! {
    <Route path="" view=HomePage/>
  }
}