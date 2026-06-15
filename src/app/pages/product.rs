use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_meta::{Title, Meta};
use crate::app::components::carousel::ImageCarousel;

#[component]
pub fn ShopItemPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    let item_resource = Resource::new(id, |id| async move {
        get_single_shop_item(id.clone()).await.ok()
    });

    view! {
        <Suspense fallback=move || view! { <div class="page-container-wide">"Ładowanie..."</div> }>
            {move || Suspend::new(async move {
                let item_data = item_resource.await;

                match item_data {
                    Some(item) => view! {
                        <Title text=format!("{} | Sklep Szoptech", item.title) />
                        <Meta name="description" content=item.description.clone() />

                        <div class="page-container">
                            <a href="/shopping" class="back-link">"← Wróć do sklepu"</a>

                            <div class="card pricing-card item-link">
                                <ImageCarousel
                                    collection_id=item.collection_id.clone()
                                    record_id=item.id.clone()
                                    images=item.images.clone()
                                    alt_text=item.title.clone()
                                />

                                <div class="pricing-content">
                                    <h2 class="project-title-2">{item.title.clone()}</h2>
                                    <div class="project-desc" inner_html=item.description.clone()></div>
                                </div>

                                <div class="pricing-footer">
                                    <div class="price-group">
                                        <span class="price-label">"Cena:"</span>
                                        <span class="price-value">{item.price.clone()}</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any(),

                    None => view! {
                        <div class="page-container">
                            <h1>"Nie znaleziono produktu."</h1>
                            <a href="/shopping">"Wróć do oferty"</a>
                        </div>
                    }.into_any()
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn PricingItemPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    let item_resource = Resource::new(id, |id| async move {
        get_single_pricing_item(id.clone()).await.ok()
    });

    view! {
        <Suspense fallback=move || view! { <div class="page-container-wide">"Ładowanie..."</div> }>
            {move || Suspend::new(async move {
                let item_data = item_resource.await;

                match item_data {
                    Some(item) => view! {
                        <Title text=format!("{} | Sklep Szoptech", item.title) />
                        <Meta name="description" content=item.description.clone() />

                        <div class="page-container">
                            <a href="/pricing" class="back-link">"← Wróć do sklepu"</a>

                            <div class="card pricing-card item-link">
                                <ImageCarousel
                                    collection_id=item.collection_id.clone()
                                    record_id=item.id.clone()
                                    images=item.images.clone()
                                    alt_text=item.title.clone()
                                />

                                <div class="pricing-content">
                                    <h2 class="project-title-2">{item.title.clone()}</h2>
                                    <div class="project-desc" inner_html=item.description.clone()></div>
                                </div>

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
                        </div>
                    }.into_any(),

                    None => view! {
                        <div class="page-container">
                            <h1>"Nie znaleziono produktu."</h1>
                            <a href="/shopping">"Wróć do oferty"</a>
                        </div>
                    }.into_any()
                }
            })}
        </Suspense>
    }
}

use crate::server::state::{ShopItem, PricingItem};
use reqwest;

#[server]
async fn get_single_shop_item(id: String) -> Result<ShopItem, ServerFnError<String>> {
    let url = format!("https://db.szoptech.com/api/collections/shop_items/records/{}", id);
    let response = reqwest::get(&url).await.map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(ServerFnError::<String>::ServerError("Nie znaleziono produktu.".into()));
    }

    let item = response.json::<ShopItem>().await.map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;
    Ok(item)
}

#[server]
async fn get_single_pricing_item(id: String) -> Result<PricingItem, ServerFnError<String>> {
    let url = format!("https://db.szoptech.com/api/collections/pricing_items/records/{}", id);
    let response = reqwest::get(&url).await.map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(ServerFnError::<String>::ServerError("Nie znaleziono wyceny.".into()));
    }

    let item = response.json::<PricingItem>().await.map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;
    Ok(item)
}

