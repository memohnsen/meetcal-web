use crate::{
    components::{footer::Footer, header::Header},
    utils::api::get_api_response,
};
use leptos::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QualifyingTotal {
    pub qualifying_total: f64,
    pub event_name: String,
    pub gender: String,
    pub age_category: String,
    pub weight_class: String,
}

#[component]
pub fn QualifyingTotals() -> impl IntoView {
    let (filter, set_filter) = signal(String::new());

    let totals = LocalResource::new(move || async move {
        get_api_response::<QualifyingTotal>("/data/qualifying-totals").await
    });

    view! {
        <Header />
        <section class="data-page">
            <p class="data-eyebrow">"Competition data"</p>
            <h1>"Qualifying totals"</h1>
            <p class="data-intro">"Filter qualification totals by event, gender, age category, or weight class."</p>
            <input class="data-filter" placeholder="Search qualifying totals" on:input=move |event| set_filter.set(event_target_value(&event)) />
            {move || totals.with(|response| match response {
                None => view! { <p class="data-status">"Loading qualifying totals…"</p> }.into_any(),
                Some(Err(error)) => view! { <p class="data-status error">{format!("Could not load qualifying totals: {error}")}</p> }.into_any(),
                Some(Ok(rows)) => {
                    let query = filter.get().to_lowercase();
                    let rows = rows.iter().filter(|row| format!("{} {} {} {}", row.event_name, row.gender, row.age_category, row.weight_class).to_lowercase().contains(&query)).map(|row| view! {
                        <tr><td>{row.event_name.clone()}</td><td>{row.gender.clone()}</td><td>{row.age_category.clone()}</td><td>{row.weight_class.clone()}</td><td>{row.qualifying_total}</td></tr>
                    }).collect_view();
                    view! { <DataTable><thead><tr><th>"Event"</th><th>"Gender"</th><th>"Age"</th><th>"Weight class"</th><th>"Total"</th></tr></thead><tbody>{rows}</tbody></DataTable> }.into_any()
                }
            })}
        </section>
        <Footer />
    }
}

#[component]
pub fn DataTable(children: Children) -> impl IntoView {
    view! { <div class="data-table-wrap"><table class="data-table">{children()}</table></div> }
}
