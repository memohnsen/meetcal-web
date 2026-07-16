use crate::{
    components::{footer::Footer, header::Header},
    utils::api::get_api_response,
};
use leptos::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Standard {
    age_category: String,
    gender: String,
    standard_a: f64,
    standard_b: f64,
    weight_class: String,
}

#[component]
pub fn Standards() -> impl IntoView {
    let (filter, set_filter) = signal(String::new());
    let standards =
        LocalResource::new(|| async { get_api_response::<Standard>("/data/standards").await });
    view! { <Header /> <section class="data-page"><p class="data-eyebrow">"Competition data"</p><h1>"Standards"</h1><p class="data-intro">"A and B standards by gender, age category, and weight class."</p><input class="data-filter" placeholder="Search standards" on:input=move |event| set_filter.set(event_target_value(&event)) />
        {move || standards.with(|response| match response { None => view!{<p class="data-status">"Loading standards…"</p>}.into_any(), Some(Err(error)) => view!{<p class="data-status error">{format!("Could not load standards: {error}")}</p>}.into_any(), Some(Ok(rows)) => { let query = filter.get().to_lowercase(); let rows = rows.iter().filter(|row| format!("{} {} {}", row.gender, row.age_category, row.weight_class).to_lowercase().contains(&query)).map(|row| view!{<tr><td>{row.gender.clone()}</td><td>{row.age_category.clone()}</td><td>{row.weight_class.clone()}</td><td>{row.standard_a}</td><td>{row.standard_b}</td></tr>}).collect_view(); view!{<div class="data-table-wrap"><table class="data-table"><thead><tr><th>"Gender"</th><th>"Age"</th><th>"Weight class"</th><th>"A"</th><th>"B"</th></tr></thead><tbody>{rows}</tbody></table></div>}.into_any() }})}
    </section><Footer /> }
}
