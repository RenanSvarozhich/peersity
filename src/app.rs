use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::portal::layout::Layout;
use crate::learning::routes::LearningRoutes;
use crate::portal::not_found_page::NotFound;
use crate::portal::routes::PortalRoutes;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/peersity.css"/>
        <script src="./node_modules/preline/dist/preline.js"></script>

        <Title text="Peersity"/>

        <Router>
            <Layout>
                <Routes>
                    <LearningRoutes />
                    <PortalRoutes />
                    <Route path="/*any" view=NotFound/>              
                </Routes>
            </Layout>
        </Router>
    }
}