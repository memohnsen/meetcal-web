use crate::{
    components::{footer::Footer, header::Header},
    utils::api::get_api_response,
};
use leptos::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Ranking {
    meet: String,
    ranking: f64,
    name: String,
    weight_class: String,
    total: f64,
    percent_a: f64,
    gender: String,
    age_category: String,
}

#[component]
pub fn Rankings() -> impl IntoView {
    let (filter, set_filter) = signal(String::new());
    let rankings =
        LocalResource::new(|| async { get_api_response::<Ranking>("/data/intl-rankings").await });
    view! { <Header /> <section class="data-page"><p class="data-eyebrow">"Competition data"</p><h1>"International rankings"</h1><p class="data-intro">"Search current international rankings by athlete, meet, division, or weight class."</p><input class="data-filter" placeholder="Search rankings" on:input=move |event| set_filter.set(event_target_value(&event)) />
        {move || rankings.with(|response| match response { None => view!{<p class="data-status">"Loading rankings…"</p>}.into_any(), Some(Err(error)) => view!{<p class="data-status error">{format!("Could not load rankings: {error}")}</p>}.into_any(), Some(Ok(rows)) => { let query = filter.get().to_lowercase(); let rows = rows.iter().filter(|row| format!("{} {} {} {} {}", row.name, row.meet, row.gender, row.age_category, row.weight_class).to_lowercase().contains(&query)).map(|row| view!{<tr><td>{row.ranking}</td><td>{row.name.clone()}</td><td>{row.meet.clone()}</td><td>{row.gender.clone()}</td><td>{row.age_category.clone()}</td><td>{row.weight_class.clone()}</td><td>{row.total}</td><td>{row.percent_a}</td></tr>}).collect_view(); view!{<div class="data-table-wrap"><table class="data-table"><thead><tr><th>"Rank"</th><th>"Athlete"</th><th>"Meet"</th><th>"Gender"</th><th>"Age"</th><th>"Weight class"</th><th>"Total"</th><th>"Percent A"</th></tr></thead><tbody>{rows}</tbody></table></div>}.into_any() }})}
    </section><Footer /> }
}
