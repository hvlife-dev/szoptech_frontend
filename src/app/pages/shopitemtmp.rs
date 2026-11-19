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

    view! {
        <div class="page-container">
            <A href="/shopping" attr:class="back-link">"← Wróć do sklepu"</A>
            <div class="card">
                <div class="product-grid">
                    // <div>
                        <ImageCarousel
                            collection_id=item.collection_id.clone()
                            record_id=item.id.clone()
                            images=item.images.clone()
                            alt_text=item.title.clone()
                        />
                    // </div>
                    
                    <div class="product-col">
                        <div class="header">
                            <h1 class="project-title-2">{item.title}</h1>
                            <h3>"Klawiatura do Paneli MC300-RS / MC300RS / WEEKE / HOMAG"</h3>
                        </div>
                        <div class="tag-row">
                            <p>"Producent: test"</p>
                            <p>"Kategoria: test"</p>
                        </div>
                        <p class="price-desc">
                            "Wysyłka urządzeń kupionych od nas bezpośrednio jest darmowa. 
                            Dostawy na terenie całego Śląska realizujemy osobiście nawet 
                            tego samego dnia bez dodatkowych kosztów."
                        </p>
                        <div class="button-grid">
                            <div>
                                <p>"Zainteresowany produktem?"</p>
                                <span class="sm-text">"Zadzwoń lub sprawdź oferte na allegro"</span>
                            </div>
                            <a href="#" class="btn-1"><span>"Allegro"</span></a>
                            <a href="#" class="btn-2"><span>"Zadzwoń"</span></a>
                        </div>
                        <div class="price-container">
                            <span class="prie-text-1">"CENA:"</span>
                            <span class="prie-text-2">"150 netto PLN"</span>
                        </div>
                    </div>
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
