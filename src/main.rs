use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use meetcal_web::pages::{
    features::FeaturesPage, home::Home, not_found::NotFound, privacy::PrivacyPage,
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
                    <Route path=path!("/features") view=FeaturesPage />
                    <Route path=path!("/privacy") view=PrivacyPage />
                    <Route path=path!("/terms") view=TermsPage />
                </Routes>
            </main>
        </Router>
    }
}
