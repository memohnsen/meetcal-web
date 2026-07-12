use js_sys::Date;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
    let current_date = Date::new_0();

    let formatted_date = format!("{}", current_date.get_full_year(),);

    view! {
        <h2>"Ready to transform your competition experience?"</h2>
        <a href="https://apps.apple.com/us/app/meetcal/id6741133286">
            <button>"Download Now"</button>
        </a>

        <p>"© " {formatted_date} " MeetCal LLC. All rights reserved."</p>
        <div>
            <A href="/privacy">
                <button>Privacy Policy</button>
            </A>
            <A href="/terms">
                <button>Terms of Use</button>
            </A>
        </div>
    }
}
