use std::collections::HashMap;

use leptos::{IntoView, component, view};
use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};


use crate::app::components::carousel::ImageCarousel;
use crate::app::components::similarproduct::{SimilarProduct, SimilarProductItem};
use crate::app::components::spectable::SpecTable;


#[component]
pub fn TmpShopItem() -> impl IntoView {
    let item = ShopItem {
        id: String::from("5w258hj77it7656"),
        collection_id:String::from("pbc_3635970959"),
        title:String::from("Klawiatura do Paneli MC300-RS / MC300RS / WEEKE / HOMAG"),
        description:String::from("Zestaw dedykowany i w pełni kompatybilny. Wykorzystywany w naprawach urządzeń automatyki."),
        price:String::from("150"),
        images:vec![
            String::from("img_e0570_gha84zyddo.jpg"),
            String::from("img_e0571_m1fycav30v.jpg"),
            String::from("img_e0569_c9sik1ghug.jpg")            
        ],
        created:String::from(""),
    };

    let sm_item: SimilarProductItem = SimilarProductItem {
        id: item.id.clone(),
        title: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
        image: String::from("https://db.szoptech.com/api/files/pbc_3635970959/5w258hj77it7656/img_e0570_gha84zyddo.jpg"),
        price: String::from("150 netto PLN"),
    };

    let sm_item2: SimilarProductItem = SimilarProductItem {
        id: item.id.clone(),
        title: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
        image: String::from("https://db.szoptech.com/api/files/pbc_3635970959/upd98gj91806s18/agc_9_2_2026_07_23_18_11_28_wgcfc8q4ce._Carlos_V11.jpg"),
        price: String::from("1500 netto PLN"),
    };

    

    let spec: Vec<(String, String)> = vec![
        ("Wysokość".to_string(), "21,5cm".to_string()),
        ("Szerokość".to_string(), "16,2cm".to_string()),
        ("Długość".to_string(), "28,7cm".to_string()),
        ("Waga".to_string(), "2,6kg".to_string()),
        ("Stan".to_string(), "Używany".to_string()),
    ];

    view! {
        <div class="page-container">
            <A href="/shopping" attr:class="back-link">
                "← Wróć do sklepu"
            </A>
            <div class="card">
                <section id="produkt" class="product-grid">
                    <div class="product-col product-gallery">
                        <ImageCarousel
                            collection_id=item.collection_id.clone()
                            record_id=item.id.clone()
                            images=item.images.clone()
                            alt_text=item.title.clone()
                        />

                        <div class="tag-row">
                            <div class="tag">
                                <span class="tag-name">"Producent:"</span>
                                <span class="tag-content">"HOMAG"</span>
                            </div>
                            <div class="tag">
                                <span class="tag-name">"Kategoria:"</span>
                                <span class="tag-content">"AKCESORIA"</span>
                            </div>
                        </div>
                    </div>

                    <div class="product-details">
                        <div class="product-title header">
                            <h1 class="project-title-2">{item.title.clone()}</h1>
                            <p class="subtitle">{item.title.clone()}</p>
                        </div>

                        <div class="product-col product-info">
                            <p class="price-desc">
                                "Wysyłka urządzeń kupionych od nas bezpośrednio jest darmowa.
                                Dostawy na terenie całego Śląska realizujemy osobiście nawet
                                tego samego dnia bez dodatkowych kosztów."
                            </p>

                            <div class="button-grid">
                                <div class="text-col">
                                    <span class="m-text orange">"Zainteresowany produktem?"</span>
                                    <span class="sm-text">"Skontaktuj się z nami"</span>
                                </div>
                                <A href="/contact" attr:class="btn">
                                    <span>"Kontakt"</span>
                                </A>
                            </div>

                            <div class="price-container">
                                <span class="prie-text-1">"CENA:"</span>
                                <span class="prie-text-2">"150 netto PLN"</span>
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
                            <div class="description">
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit,
                                sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
                                Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris
                                nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
                                reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."

                            </div>
                        </section>

                        <section id="specyfikacja" class="product-col">
                            <div class="header">
                                <h3 class="section-header">"Specyfikacja"</h3>
                            </div>
                            <div class="specyfication">
                                <SpecTable specification=spec />
                            </div>
                        </section>
                    </div>
                    <section id="polecane-produkty" class="product-col">
                        <div class="header">
                            <h3 class="section-header">"Podobne produkty"</h3>
                        </div>
                        <SimilarProduct product=sm_item.clone() />
                        <SimilarProduct product=sm_item2.clone() />
                        <SimilarProduct product=sm_item.clone() />
                    </section>
                </div>
            </div>
        </div>
    }
}



#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ShopItem {
    pub id: String,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    pub title: String,
    pub description: String,
    pub price: String,
    pub images: Vec<String>,
    pub created: String,
}

