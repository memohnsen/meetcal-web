use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        // TODO: add logo here
        <A href="/">
            <button>MeetCal</button>
        </A>
        <A href="/privacy">
            <button>Privacy</button>
        </A>
        <A href="/terms">
            <button>Terms</button>
        </A>
        <A href="/contact">
            <button>Contact Us</button>
        </A>
    }
}
