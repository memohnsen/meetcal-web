use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        // TODO: add logo here
        <button>MeetCal</button>
        <button>Privacy</button>
        <button>Terms</button>
        <button>Contact Us</button>
    }
}
