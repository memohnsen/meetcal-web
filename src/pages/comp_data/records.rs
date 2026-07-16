use crate::{
    components::{footer::Footer, header::Header},
    utils::api::get_api_response,
};
use leptos::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    age_category: String,
    gender: String,
    weight_class: String,
    record_type: String,
    snatch_record: f64,
    cj_record: f64,
    total_record: f64,
}

#[component]
pub fn Records() -> impl IntoView {
    let (filter, set_filter) = signal(String::new());
    let records =
        LocalResource::new(|| async { get_api_response::<Record>("/data/records").await });

    view! {
        <Header />
        <section class="data-page">
            <p class="data-eyebrow">"Competition data"</p>
            <h1>"Records"</h1>
            <p class="data-intro">"Filter record performances by organization, division, and weight class."</p>
            <input
                class="data-filter"
                placeholder="Search records"
                on:input=move |event| set_filter.set(event_target_value(&event))
            />

            {move || records.with(|response| match response {
                None => view! {
                    <p class="data-status">"Loading records…"</p>
                }
                .into_any(),
                Some(Err(error)) => view! {
                    <p class="data-status error">
                        {format!("Could not load records: {error}")}
                    </p>
                }
                .into_any(),
                Some(Ok(records)) => {
                    let query = filter.get().to_lowercase();
                    let rows = records
                        .iter()
                        .filter(|record| {
                            format!(
                                "{} {} {} {}",
                                record.record_type,
                                record.gender,
                                record.age_category,
                                record.weight_class,
                            )
                            .to_lowercase()
                            .contains(&query)
                        })
                        .map(|record| view! {
                            <tr>
                                <td>{record.record_type.clone()}</td>
                                <td>{record.gender.clone()}</td>
                                <td>{record.age_category.clone()}</td>
                                <td>{record.weight_class.clone()}</td>
                                <td>{record.snatch_record}</td>
                                <td>{record.cj_record}</td>
                                <td>{record.total_record}</td>
                            </tr>
                        })
                        .collect_view();

                    view! {
                        <div class="data-table-wrap">
                            <table class="data-table">
                                <thead>
                                    <tr>
                                        <th>"Type"</th>
                                        <th>"Gender"</th>
                                        <th>"Age"</th>
                                        <th>"Weight class"</th>
                                        <th>"Snatch"</th>
                                        <th>"Clean & jerk"</th>
                                        <th>"Total"</th>
                                    </tr>
                                </thead>
                                <tbody>{rows}</tbody>
                            </table>
                        </div>
                    }
                    .into_any()
                }
            })}
        </section>
        <Footer />
    }
}
