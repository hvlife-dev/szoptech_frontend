use std::collections::HashMap;

use leptos::{IntoView, component, view};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::app::components::carousel::ImageCarousel2;
use crate::app::components::similarproduct::{SimilarProduct, SimilarProductItem};
use crate::app::components::spectable::SpecTable;
use crate::server::service::shop_item_service::{get_shop_item_by_id, get_shop_item_view_by_id};


#[component]
pub fn TmpShopItem() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    let item_resource = Resource::new(
        id, 
        |id| async move {
            get_shop_item_view_by_id(id.clone()).await.ok()
        });


    view! {
        <div class="page-container shop-item-container">
            <A href="/shopping" attr:class="back-link">
                "← Wróć do sklepu"
            </A>
            <Suspense fallback=move || {
                view! { <div class="page-container-wide">"Ładowanie..."</div> }
            }>
                {move || Suspend::new(async move {
                    let item_data = item_resource.await;
                    match item_data {
                        Some(item) => {
                            view! {
                                <div class="card">
                                    <section id="produkt" class="product-grid">
                                        <div class="product-col product-gallery">
                                            <ImageCarousel2
                                                images=item.images
                                                alt_text="".to_string()
                                            />

                                            // Tu dodać
                                            <div class="tag-row">
                                                <div class="tag">
                                                    <span class="tag-name">"Producent:"</span>
                                                    <span class="tag-content">{item.producer}</span>
                                                </div>
                                                <div class="tag">
                                                    <span class="tag-name">"Kategoria:"</span>
                                                    <span class="tag-content">{item.category}</span>
                                                </div>
                                            </div>
                                        </div>

                                        <div class="product-details">
                                            <div class="product-title header">
                                                <h1 class="project-title-2">{item.title}</h1>
                                                {
                                                    let sub_title = item.sub_title;
                                                    match sub_title {
                                                        Some(sub_title) => {
                                                            view! { <p class="subtitle">{sub_title}</p> }.into_any()
                                                        }
                                                        None => view! { <></> }.into_any(),
                                                    }
                                                }
                                            </div>

                                            <div class="product-col product-info">
                                                <p class="price-desc">
                                                    "Wysyłka urządzeń kupionych od nas bezpośrednio jest darmowa.
                                                    Dostawy na terenie całego Śląska realizujemy osobiście nawet
                                                    tego samego dnia bez dodatkowych kosztów."
                                                </p>

                                                <div class="button-grid">
                                                    <div class="text-col">
                                                        <span class="m-text orange">
                                                            "Zainteresowany produktem?"
                                                        </span>
                                                        <span class="sm-text">"Skontaktuj się z nami"</span>
                                                    </div>
                                                    <A href="/contact" attr:class="btn">
                                                        <span>"Kontakt"</span>
                                                    </A>
                                                </div>

                                                <div class="price-container">
                                                    <span class="prie-text-1">"CENA:"</span>
                                                    <span class="prie-text-2">{item.price}" netto PLN"</span>
                                                </div>
                                            </div>
                                        </div>
                                    </section>

                                    <div style="height:26px; width:100%" />

                                    <div class="bottom-grid">
                                        <div class="product-col">
                                            <section id="opis" class="product-col">
                                                <div class="header">
                                                    <h3 class="section-header">"Opis"</h3>
                                                </div>
                                                <div class="description" inner_html=item.description></div>
                                            </section>

                                            {
                                                let specification = item.specification;
                                                match specification {
                                                    Some(specification) => {
                                                        view! {
                                                            <section id="specyfikacja" class="product-col">
                                                                <div class="header">
                                                                    <h3 class="section-header">"Specyfikacja"</h3>
                                                                </div>
                                                                <div class="specyfication">
                                                                    <SpecTable specification=specification />
                                                                </div>
                                                            </section>
                                                        }
                                                            .into_any()
                                                    }
                                                    None => view! { <></> }.into_any(),
                                                }
                                            }
                                        </div>
                                        <section id="polecane-produkty" class="product-col">
                                            <div class="header">
                                                <h3 class="section-header">"Podobne produkty"</h3>
                                            </div>
                                        // <SimilarProduct product=sm_item.clone() />
                                        // <SimilarProduct product=sm_item2.clone() />
                                        // <SimilarProduct product=sm_item.clone() />
                                        </section>
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                        None => {

                            // Tu dodać

                            // <SimilarProduct product=sm_item.clone() />
                            // <SimilarProduct product=sm_item2.clone() />
                            // <SimilarProduct product=sm_item.clone() />
                            // <ImageCarousel
                            // collection_id=item_tmp.collection_id.clone()
                            // record_id=item_tmp.id.clone()
                            // images=item_tmp.images.clone()
                            // alt_text=item_tmp.title.clone()
                            // />

                            // Tu dodać
                            // <div class="tag-row">
                            // <TagRow />

                            // <SpecTable specification=spec />
                            // <SimilarProduct product=sm_item.clone() />
                            // <SimilarProduct product=sm_item2.clone() />
                            // <SimilarProduct product=sm_item.clone() />

                            view! { <h1>"Nie znaleziono produktu."</h1> }
                                .into_any()
                        }
                    }
                })}
            </Suspense>
        </div>
    }
}