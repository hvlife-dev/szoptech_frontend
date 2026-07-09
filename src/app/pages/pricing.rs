use leptos::prelude::*;
use leptos_router::components::A;

use crate::app::components::carousel::ImageCarousel;
use crate::app::components::divider::Divider;
use crate::app::metatags::seomt::SeoMetaTags;


#[component]
pub fn PricingPage() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());

    let pricing_resource = Resource::new(|| (), |_| async move {
        get_pricing_items().await.unwrap_or_default()
    });

    view! {
        // @oskif
        <SeoMetaTags />
        
        <div class="page-container-wide">
            <div class="contact-hero">
                <h1 class="hero-title">"Konkretne Wyceny"</h1>
                <p class="hero-subtitle">
                    "Cena gwarantowana niezależnie od stanu urządzenia."
                    <br/>
                    "Roczna gwarancja na wszystkie wymienione naprawy."
                </p>
            </div>

            <Divider />

            <div class="compact-search-container">
                <div class="search-input-wrapper">
                    <svg xmlns="http://www.w3.org/2000/svg" class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="11" cy="11" r="8"></circle>
                        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                    </svg>
                    <input
                        type="text"
                        class="compact-search-input"
                        placeholder="Szukaj po nazwie..."
                        prop:value=search_query
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <Suspense fallback=move || view! { <div class="loading-state">"Pobieranie cennika..."</div> }>
                <div class="pricing-list-grid">
                    {move || Suspend::new(async move {
                        let items = pricing_resource.await;

                        if items.is_empty() {
                            return view! {
                                <div class="card error" style="grid-column: 1 / -1; text-align: center;">
                                    "Brak dostępnych wycen w bazie danych."
                                </div>
                            }.into_any();
                        }

                        view! {
                            {move || {
                                let query = search_query.get().to_lowercase();
                                let filtered_items: Vec<_> = items.iter().filter(|item| {
                                    if query.is_empty() {
                                        true
                                    } else {
                                        item.title.to_lowercase().contains(&query)
                                    }
                                }).cloned().collect();

                                if filtered_items.is_empty() {
                                    return view! {
                                        <div class="card error" style="grid-column: 1 / -1; text-align: center;">
                                            "Brak wyników dla: " <span class="text-accent">{query.clone()}</span>
                                        </div>
                                    }.into_any();
                                }

                                filtered_items.into_iter().map(|item| {
                                    view! {
                                        <div class="card pricing-card item-link">
                                            <ImageCarousel
                                                collection_id=item.collection_id.clone()
                                                record_id=item.id.clone()
                                                images=item.images.clone()
                                                alt_text=item.title.clone()
                                            />

                                            <A href=format!("/estimate/{}", item.id) attr:class="pricing-content">
                                                <h2 class="project-title">{item.title.clone()}</h2>
                                                <div class="project-desc" inner_html=item.description.clone()></div>
                                            </A>

                                            <div class="pricing-footer">
                                                <div class="price-group">
                                                    <span class="price-label">"Naprawa do 5 dni roboczych:"</span>
                                                    <span class="price-value">{item.estimated_price.clone()}</span>
                                                </div>
                                                <div class="price-group">
                                                    <span class="price-label">"Tryb Express (48h):"</span>
                                                    <span class="price-value">{item.express_price.clone()}</span>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }}
                        }.into_any()
                    })}
                </div>
            </Suspense>
        </div>
    }
}

use crate::server::state::{PbResponse, PricingItem};
use reqwest;

#[server]
pub async fn get_pricing_items() -> Result<Vec<PricingItem>, ServerFnError<String>> {
    let url = "https://db.szoptech.com/api/collections/pricing_items/records?sort=-created";

    let response = reqwest::get(url)
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    let pb_data = response
        .json::<PbResponse<PricingItem>>()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    Ok(pb_data.items)
}
