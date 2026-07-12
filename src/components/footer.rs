use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <h2>"Ready to transform your competition experience?"</h2>
        <button>"Download Now"</button>

        // TODO: fix hardcoded year
        <p>"© 2026 MeetCal LLC. All rights reserved."</p>
        <div>
            <button>"Privacy Policy"</button>
            <button>"Terms of Use"</button>
        </div>
    }
}
