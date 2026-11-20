use leptos::{IntoView, component, view};
use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::app::components::carousel::ImageCarousel;


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

    view! {
        <div class="page-container">
            <A href="/shopping" attr:class="back-link">
                "← Wróć do sklepu"
            </A>
            <div class="card">
                <section id="produkt" class="product-grid">
                    <div class="product-col">
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

                    <div class="product-col">
                        <div class="header">
                            <h1 class="project-title-2">{item.title}</h1>
                            <p class="subtitle">
                                "Klawiatura do Paneli MC300-RS / MC300RS / WEEKE / HOMAG"
                            </p>
                        </div>
                        <p class="price-desc">
                            "Wysyłka urządzeń kupionych od nas bezpośrednio jest darmowa. 
                            Dostawy na terenie całego Śląska realizujemy osobiście nawet 
                            tego samego dnia bez dodatkowych kosztów."
                        </p>
                        <div class="button-grid">
                            <div class="text-col">
                                <span class="m-text orange">"Zainteresowany produktem?"</span>
                                <span class="sm-text">
                                    "Zadzwoń lub sprawdź oferte na allegro"
                                </span>
                            </div>
                            <a href="#" class="btn-1">
                                <span>"Allegro"</span>
                            </a>
                            <a href="#" class="btn-2">
                                <span>"Zadzwoń"</span>
                            </a>
                        </div>
                        <div class="price-container">
                            <span class="prie-text-1">"CENA:"</span>
                            <span class="prie-text-2">"150 netto PLN"</span>
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
                                <table class="spec-table">
                                    <tbody>
                                        <tr class="spec-row">
                                            <td class="spec-key">"Stan:"</td>
                                            <td class="spec-value">"Używany"</td>
                                        </tr>
                                        <tr class="spec-row">
                                            <td class="spec-key">"Wysokość:"</td>
                                            <td class="spec-value">"165mm"</td>
                                        </tr>
                                        <tr class="spec-row">
                                            <td class="spec-key">"Szerokość:"</td>
                                            <td class="spec-value">"85mm"</td>
                                        </tr>
                                        <tr class="spec-row">
                                            <td class="spec-key">"Głębokość"</td>
                                            <td class="spec-value">"110mm"</td>
                                        </tr>
                                        <tr class="spec-row">
                                            <td class="spec-key">"Waga:"</td>
                                            <td class="spec-value">"2kg"</td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>
                        </section>
                    </div>
                    <section id="polecane-produkty" class="product-col">
                        <div class="header">
                            <h3 class="section-header">"Podobne produkty"</h3>
                        </div>
                        <SimilarProduct product=sm_item.clone() />
                        <SimilarProduct product=sm_item.clone() />
                        <SimilarProduct product=sm_item.clone() />
                    </section>
                </div>

            </div>
        </div>
    }
}

#[component]
pub fn SimilarProduct(
    product: SimilarProductItem
) -> impl IntoView { 
    let url = format!("../store/{}", product.id);

    view! {
        <A href=url attr:class="similar-product">
            <img class="similar-product-img" src=product.image />
            <div class="similar-product-content">
                <div class="header">
                    <h3 class="similar-product-title">{product.title}</h3>
                </div>
                <div class="price-container">
                    <span class="prie-text-1">"CENA:"</span>
                    <span class="prie-text-2">{product.price}</span>
                </div>
            </div>
        </A>
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SimilarProductItem {
    pub id: String,
    pub title: String,
    pub image: String,
    pub price: String
}