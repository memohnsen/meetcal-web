use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use meetcal_web::components::{footer::Footer, header::Header};
use meetcal_web::pages::{
    home::{BodySection, HighlightSection, TitleSection},
    privacy::PrivacyPolicy,
    terms::TermsOfUse,
};

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                // TODO: real not found page
                <Routes fallback=|| view! { <h1>"Page not found"</h1> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/privacy") view=PrivacyPage />
                    <Route path=path!("/terms") view=TermsPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Header />
        <TitleSection />
        <BodySection />
        <HighlightSection />
        <Footer />
    }
}

#[component]
fn PrivacyPage() -> impl IntoView {
    view! {
        <Header />
        <PrivacyPolicy />
        <Footer />
    }
}

#[component]
fn TermsPage() -> impl IntoView {
    view! {
        <Header />
        <TermsOfUse />
        <Footer />
    }
}
