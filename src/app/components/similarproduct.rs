use leptos::{IntoView, component, view};
use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SimilarProductItem {
    pub id: String,
    pub title: String,
    pub image: String,
    pub price: String
}

#[component]
pub fn SimilarProduct(
    product: SimilarProductItem
) -> impl IntoView { 
    let url = format!("../store/{}", product.id);

    view! {
        <A href=url attr:class="similar-product">
            <div class="sp-img-container">
                <img class="sp-img" src=product.image />
            </div>
            <div class="sp-content">
                <div class="header">
                    <h3 class="sp-title">{product.title}</h3>
                </div>
                <div class="price-container">
                    <span class="prie-text-1">"CENA:"</span>
                    <span class="prie-text-2">{product.price}</span>
                </div>
            </div>
        </A>
    }
}