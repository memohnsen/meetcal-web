use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use meetcal_web::pages::{
    comp_data::{
        data_home::CompData, qual_totals::QualifyingTotals, rankings::Rankings, records::Records,
        results::Results, standards::Standards,
    },
    home::Home,
    not_found::NotFound,
    privacy::PrivacyPage,
    terms::TermsPage,
};

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/comp-data") view=CompData />
                    <Route path=path!("/qualifying-totals") view=QualifyingTotals />
                    <Route path=path!("/standards") view=Standards />
                    <Route path=path!("/records") view=Records />
                    <Route path=path!("/results") view=Results />
                    <Route path=path!("/rankings") view=Rankings />
                    <Route path=path!("/privacy") view=PrivacyPage />
                    <Route path=path!("/terms") view=TermsPage />
                </Routes>
            </main>
        </Router>
    }
}
