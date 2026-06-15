use crate::app::components::carousel::ImageCarousel;
use leptos::prelude::*;

use crate::app::components::divider::Divider;
use crate::server::state::{ClientDashboardData, ProjectStatus};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ClientPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").unwrap_or_default());

    let dashboard_resource = Resource::new(id, |client_id| async move {
        get_client_dashboard(client_id).await.ok()
    });

    view! {
        <div class="page-container">
            <Suspense fallback=move || view! { <p>"Loading client portal..."</p> }>
                {move || Suspend::new(async move {
                    let Some(data) = dashboard_resource.await else {
                        return view! { <div class="card error">"Client not found or access denied."</div> }.into_any();
                    };

                    view! {
                        <div class="client-header" style="margin-bottom: 2rem;">
                            <h1>{data.client.name}</h1>
                            <p class="hero-subtitle">
                                <span class="keyword">"Ref ID: "</span> {data.client.id}
                            </p>
                        </div>

                        <Divider text="Historia twoich zamówień" />

                        <div class="grid">
                            {if data.projects.is_empty() {
                                view! { <p>"Brak projektów na tym koncie."</p> }.into_any()
                            } else {
                                data.projects.into_iter().map(|item| {
                                    let (status_text, status_class) = match item.status {
                                        ProjectStatus::Accepted => ("Zaakceptowane", "badge-received"),
                                        ProjectStatus::InProgress => ("W realizacji", "badge-progress"),
                                        ProjectStatus::Sent => ("Wysłane", "badge-review"),
                                        ProjectStatus::Completed => ("Zrealizowane", "badge-completed"),
                                    };

                                    view! {
                                        <div class="card">
                                            <ImageCarousel
                                                collection_id=item.collection_id.clone()
                                                record_id=item.id.clone()
                                                images=item.images.clone()
                                                alt_text=item.title.clone()
                                            />

                                            <div style="display: flex; justify-content: space-between; align-items: start; margin-top: 1rem;">
                                                <h2 style="margin-bottom: 0;">{item.title}</h2>
                                                <span class=format!("status-badge {}", status_class)>
                                                    {status_text}
                                                </span>
                                            </div>

                                            <div inner_html=item.description></div>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }}
                        </div>
                    }.into_any()
                })}
            </Suspense>
        </div>
    }
}

use crate::server::state::{AuthResponse, Client, PbResponse, Project};
use reqwest;

#[server]
pub async fn get_client_dashboard(
    client_id: String,
) -> Result<ClientDashboardData, ServerFnError<String>> {
    let client_req = reqwest::Client::new();

    let auth_url = "https://db.szoptech.com/api/collections/_superusers/auth-with-password";
    let auth_payload = serde_json::json!({
        "identity": "dev@retardigrade.me",
        "password": "7uWiOFTV4wiIaj"
    });

    let auth_res = client_req
        .post(auth_url)
        .json(&auth_payload)
        .send()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?
        .json::<AuthResponse>()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    let admin_token = auth_res.token;

    let client_url = format!(
        "https://db.szoptech.com/api/collections/clients/records/{}",
        client_id
    );
    let client = client_req
        .get(&client_url)
        .header("Authorization", format!("Bearer {}", admin_token))
        .send()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?
        .json::<Client>()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    let projects_url = "https://db.szoptech.com/api/collections/projects/records";
    let filter_query = format!("client='{}'", client_id);

    let projects_resp = client_req
        .get(projects_url)
        .header("Authorization", format!("Bearer {}", admin_token))
        .query(&[("filter", filter_query), ("sort", "-created".to_string())])
        .send()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?
        .json::<PbResponse<Project>>()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    Ok(ClientDashboardData {
        client,
        projects: projects_resp.items,
    })
}
