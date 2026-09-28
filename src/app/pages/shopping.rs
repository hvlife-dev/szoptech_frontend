use leptos::prelude::*;
use leptos_router::components::A;
use crate::app::components::divider::Divider;
use crate::app::components::carousel::ImageCarousel;
use crate::app::metatags::seomt::SeoMetaTags;

struct ShopLink {
    name: &'static str,
    url: &'static str,
    logo_path: &'static str,
    brand_class: &'static str,
}

#[component]
pub fn ShoppingPage() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());

    let shops = vec![
        ShopLink {
            name: "Allegro",
            url: "https://allegro.pl/uzytkownik/SzopTech",
            logo_path: "/allegro.svg",
            brand_class: "brand-allegro",
        },
        ShopLink {
            name: "",
            url: "https://ebay.pl/",
            logo_path: "/ebay.svg",
            brand_class: "brand-ebay",
        },
    ];

    let shop_items_resource = Resource::new(|| (), |_| async move {
        get_shop_items().await.unwrap_or_default()
    });

    view! {
        // @oskif
        <SeoMetaTags />

        <div class="page-container-wide">

            <div class="shopping-hero-grid">

                <div class="compact-shops-stack">
                    <A href="/contact" attr:class="card shop-card brand-contact">
                        <div class="shop-header">
                            <svg xmlns="http://www.w3.org/2000/svg" class="shop-logo" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color: var(--brand-main);">
                                <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
                                <polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline>
                                <line x1="12" y1="22.08" x2="12" y2="12"></line>
                            </svg>
                            <h2 class="shop-title">"Odkup"</h2>
                        </div>
                        <svg xmlns="http://www.w3.org/2000/svg" class="external-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="5" y1="12" x2="19" y2="12"></line>
                            <polyline points="12 5 19 12 12 19"></polyline>
                        </svg>
                    </A>

                    <A href="/contact" attr:class="card shop-card brand-support">
                        <div class="shop-header">
                            <svg xmlns="http://www.w3.org/2000/svg" class="shop-logo" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color: var(--brand-main);">
                                <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"></path>
                            </svg>
                            <h2 class="shop-title">"Sprzedaż"</h2>
                        </div>
                        <svg xmlns="http://www.w3.org/2000/svg" class="external-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="5" y1="12" x2="19" y2="12"></line>
                            <polyline points="12 5 19 12 12 19"></polyline>
                        </svg>
                    </A>
                </div>

                <div class="hero-section hero-compact">
                    <h1 class="hero-title">"Zakupy"</h1>
                    <p class="hero-subtitle">
                        "Wysyłka urządzeń kupionych od nas bezpośrednio jest darmowa."<br/><br/>
                        <span class="text-accent">"Dostawy na terenie całego Śląska realizujemy osobiście nawet tego samego dnia bez dodatkowych kosztów."</span><br/>
                    </p>
                </div>

                <div class="compact-shops-stack">
                    {shops.into_iter().map(|shop| {
                        view! {
                            <a href=shop.url target="_blank" rel="noopener noreferrer" class=format!("card shop-card {}", shop.brand_class)>
                                <div class="shop-header">
                                    <img src=shop.logo_path alt=shop.name class="shop-logo" />
                                    <h2 class="shop-title">{shop.name}</h2>
                                </div>

                                <svg xmlns="http://www.w3.org/2000/svg" class="external-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                                    <polyline points="15 3 21 3 21 9"></polyline>
                                    <line x1="10" y1="14" x2="21" y2="3"></line>
                                </svg>
                            </a>
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="hero-extension">
                <p>
                    "Odpłatnie oferujemy możliwość montażu w maszynie, przekopiowania parametryzacji lub przeniesienia istniejącego projektu klienta." <br/>
                </p>
            </div>
            <div class="hero-extension">
                <p>
                    <span class="text-accent">"Prowadzimy również skup elektroniki przemysłowej niezależnie od stanu."</span>
                </p>
            </div>

            <Divider text="Oferta Bezpośrednia" />

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

            <Suspense fallback=move || view! { <div class="loading-state">"Ładowanie oferty..."</div> }>
                <div class="pricing-list-grid">
                    {move || Suspend::new(async move {
                        let items = shop_items_resource.await;

                        if items.is_empty() {
                            return view! {
                                <div class="card error" style="grid-column: 1 / -1; text-align: center;">
                                    "Obecnie brak przedmiotów w ofercie bezpośredniej."
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

                                            <A href=format!("/store/{}", item.id) attr:class="pricing-content">
                                                <h2 class="project-title">{item.title.clone()}</h2>
                                                <div class="project-desc" inner_html=item.description.clone()></div>
                                            </A>

                                            <div class="pricing-footer">
                                                <div class="price-group">
                                                    <span class="price-label">"Cena:"</span>
                                                    <span class="price-value">{item.price.clone()}</span>
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

use crate::server::state::{PbResponse, ShopItem};
use reqwest;

#[server]
pub async fn get_shop_items() -> Result<Vec<ShopItem>, ServerFnError<String>> {
    let url = "https://db.szoptech.com/api/collections/shop_items/records?sort=-created";

    let response = reqwest::get(url)
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    let pb_data = response
        .json::<PbResponse<ShopItem>>()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    Ok(pb_data.items)
}
