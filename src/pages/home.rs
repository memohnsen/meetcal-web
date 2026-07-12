use crate::components::feature::FeatureSection;
use leptos::prelude::*;

#[component]
pub fn TitleSection() -> impl IntoView {
    view! {
        <h1>"Your Competition Schedule, Simplified"</h1>
        <h4>"MeetCal transforms complex Olympic weightlifting competition schedules into an intuitive, easy-to-use mobile experience."</h4>

        <button>"Download for iOS"</button>
        <button>"Download for Android"</button>
    }
}

#[component]
pub fn BodySection() -> impl IntoView {
    view! {
        <h2>"Everything you need to stay on track"</h2>

        <FeatureSection title="Simple Schedule View" subtitle="Access your competition schedule instantly with our clean, intuitive interface." />

        <FeatureSection title="Save to Calendar" subtitle="Access your competition schedule instantly with our clean, intuitive interface." />

        <FeatureSection title="Track Your Team" subtitle="Keep tabs on all your athletes and teams in one place." />

        <FeatureSection title="Smart Filtering" subtitle="Filter by weight class, platform, or club to find exactly what you need." />

        <FeatureSection title="Cross-Platform" subtitle="Available on both iOS and Android" />
    }
}

#[component]
pub fn HighlightSection() -> impl IntoView {
    view! {
        <h2>"Throw Out the PDFs"</h2>
        <p>"Experience MeetCal's intuitive interface designed for athletes, coaches, and spectators."</p>
    }
}
