use crate::app::components::carousel::ImageCarousel;
use crate::app::metatags::seomt::SeoMetaTags;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects_resource = Resource::new(
        || (),
        |_| async move { get_public_projects().await.unwrap_or_default() },
    );

    let (selected_type, set_selected_type) = signal(String::new());
    let (selected_company, set_selected_company) = signal(String::new());

    view! {
        // oskif
        <SeoMetaTags />

        <div class="page-container-wide">
            <Suspense fallback=move || view! { <div class="loading-state">"Pobieranie danych telemetrycznych..."</div> }>
                {move || Suspend::new(async move {
                    let all_projects = projects_resource.await;

                    let projects_for_grid = all_projects.clone();

                    let mut unique_types: Vec<String> = all_projects.iter()
                        .map(|p| p.project_type.clone())
                        .filter(|t| !t.is_empty())
                        .collect();
                    unique_types.sort();
                    unique_types.dedup();

                    let mut unique_companies: Vec<String> = all_projects.iter()
                        .filter_map(|p| p.company_name.clone())
                        .filter(|c| !c.is_empty())
                        .collect();
                    unique_companies.sort();
                    unique_companies.dedup();

                    view! {
                        <div class="filter-panel">
                            <div class="form-group">
                                <label>"Kategorie / Rodzaje napraw"</label>
                                <select
                                    class="form-control filter-select"
                                    on:change=move |ev| set_selected_type.set(event_target_value(&ev))
                                >
                                    <option value="">"Wszystkie Technologie"</option>
                                    {unique_types.into_iter().map(|t| view! { <option value=t.clone()>{t.clone()}</option> }).collect_view()}
                                </select>
                            </div>

                            <div class="form-group">
                                <label>"Firmy"</label>
                                <select
                                    class="form-control filter-select"
                                    on:change=move |ev| set_selected_company.set(event_target_value(&ev))
                                >
                                    <option value="">"Wszyscy Klienci"</option>
                                    {unique_companies.into_iter().map(|c| view! { <option value=c.clone()>{c.clone()}</option> }).collect_view()}
                                </select>
                            </div>
                        </div>

                        <div class="projects-grid-3">
                            {move || {
                                let t_filter = selected_type.get();
                                let c_filter = selected_company.get();

                                let filtered_projects: Vec<_> = projects_for_grid.iter().filter(|p| {
                                    let match_type = t_filter.is_empty() || p.project_type == t_filter;
                                    let match_comp = c_filter.is_empty() || p.company_name.as_deref().unwrap_or("") == c_filter;
                                    match_type && match_comp
                                }).cloned().collect();

                                if filtered_projects.is_empty() {
                                    return view! {
                                        <div class="card error" style="grid-column: 1 / -1; text-align: center;">
                                            "Brak wyników spełniających podane kryteria."
                                        </div>
                                    }.into_any();
                                }

                                filtered_projects.into_iter().map(|item| {
                                    view! {
                                        <div class="card project-card item-link item-link">
                                            <ImageCarousel
                                                collection_id=item.collection_id.clone()
                                                record_id=item.id.clone()
                                                images=item.images.clone()
                                                alt_text=item.title.clone()
                                            />

                                            <div class="project-meta">
                                                <span class="meta-badge type">{item.project_type.clone()}</span>
                                                {if let Some(comp) = &item.company_name {
                                                    view! { <span class="meta-badge company">{comp.clone()}</span> }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </div>

                                            <A href=format!("/project/{}", item.id) attr:class="pricing-content">
                                                <h2 class="project-title">{item.title.clone()}</h2>
                                                <div class="project-desc" inner_html=item.description.clone()></div>
                                            </A>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }}
                        </div>
                    }
                })}
            </Suspense>
        </div>
    }
}

use crate::server::state::{PbResponse, Project};
use reqwest;

#[server]
pub async fn get_public_projects() -> Result<Vec<Project>, ServerFnError<String>> {
    let url = "https://db.szoptech.com/api/collections/projects/records?sort=-created&perPage=500";

    let response = reqwest::get(url)
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    let pb_data = response
        .json::<PbResponse<Project>>()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    Ok(pb_data.items)
}
