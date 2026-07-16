use crate::{
    components::{footer::Footer, header::Header},
    utils::api::get_api_response_with_query,
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
struct SearchQuery {
    query: String,
    start_date: String,
    end_date: String,
}

#[derive(Deserialize)]
struct SearchResponse {
    matched_name: Option<String>,
    suggestions: Vec<String>,
    results: Vec<LiftingResult>,
}

#[derive(Deserialize)]
struct LiftingResult {
    federation: String,
    meet: String,
    date: String,
    name: String,
    age: String,
    body_weight: f64,
    snatch_best: f64,
    cj_best: f64,
    total: f64,
    adaptive: bool,
}

#[component]
pub fn Results() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (start_date, set_start_date) = signal(String::new());
    let (end_date, set_end_date) = signal(String::new());
    let (request, set_request) = signal(None::<SearchQuery>);

    let results = LocalResource::new(move || {
        let request = request.get();
        async move {
            match request {
                Some(query) => get_api_response_with_query::<SearchResponse, _>("/search", &query)
                    .await
                    .map_err(|error| error.to_string()),
                None => Ok(SearchResponse {
                    matched_name: None,
                    suggestions: Vec::new(),
                    results: Vec::new(),
                }),
            }
        }
    });

    view! {
        <Header />
        <section class="data-page">
            <p class="data-eyebrow">"Competition data"</p>
            <h1>"Results"</h1>
            <p class="data-intro">"Search an athlete’s competition history within a date range."</p>
            <form class="result-search" on:submit=move |event| {
                event.prevent_default();
                let athlete = name.get().trim().to_owned();
                let start = start_date.get();
                let end = end_date.get();
                if !athlete.is_empty() && !start.is_empty() && !end.is_empty() {
                    set_request.set(Some(SearchQuery { query: athlete, start_date: start, end_date: end }));
                }
            }>
                <input class="data-filter" placeholder="Athlete name" on:input=move |event| set_name.set(event_target_value(&event)) />
                <input class="data-filter" type="date" aria-label="Start date" on:input=move |event| set_start_date.set(event_target_value(&event)) />
                <input class="data-filter" type="date" aria-label="End date" on:input=move |event| set_end_date.set(event_target_value(&event)) />
                <button class="data-search-button" type="submit">"Search"</button>
            </form>
            {move || results.with(|response| match response {
                None => view! { <p class="data-status">"Loading results…"</p> }.into_any(),
                Some(Err(error)) => view! { <p class="data-status error">{format!("Could not load results: {error}")}</p> }.into_any(),
                Some(Ok(response)) if request.get().is_none() => view! { <p class="data-status">"Enter an athlete name and date range to search."</p> }.into_any(),
                Some(Ok(response)) => {
                    let suggestions = (!response.suggestions.is_empty()).then(|| view! {
                        <p class="data-status">"Try: " {response.suggestions.join(", ")}</p>
                    });
                    let rows = response.results.iter().map(|row| view! {
                        <tr><td>{row.date.clone()}</td><td>{row.name.clone()}</td><td>{row.meet.clone()}</td><td>{row.federation.clone()}</td><td>{row.age.clone()}</td><td>{row.body_weight}</td><td>{row.snatch_best}</td><td>{row.cj_best}</td><td>{row.total}</td><td>{if row.adaptive { "Yes" } else { "No" }}</td></tr>
                    }).collect_view();
                    let heading = response.matched_name.as_ref().map(|name| view! { <p class="data-status">"Results for " <strong>{name.clone()}</strong></p> });
                    view! { {heading} {suggestions} <div class="data-table-wrap"><table class="data-table"><thead><tr><th>"Date"</th><th>"Athlete"</th><th>"Meet"</th><th>"Federation"</th><th>"Division"</th><th>"Bodyweight"</th><th>"Snatch"</th><th>"Clean & jerk"</th><th>"Total"</th><th>"Adaptive"</th></tr></thead><tbody>{rows}</tbody></table></div> }.into_any()
                }
            })}
        </section>
        <Footer />
    }
}
