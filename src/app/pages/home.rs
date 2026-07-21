use crate::app::{
    components::recommendations::RecommendationsCarousel, metatags::seomt::SeoMetaTags,
};
use leptos::prelude::*;
use leptos_router::components::A;
// use crate::app::components::divider::Divider;

use leptos_meta::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let h2_style = "color: #ebdbb2; max-width: 600px; font-size: 1.1rem; text-align: center; font-weight: 400; margin: 0 auto;";

    view! {
        // oskif
        <SeoMetaTags />
        <Title text="Szoptech - Elektronika Przemysłowa & Automatyka"/>


        <div class="page-container">
            <div class="hero-section">
                <h1 class="hero-title">"Elektronika Przemysłowa & Automatyka"</h1>

                <div class="hero-subtitle">
                    <h2 style=h2_style>"Naprawiamy od lat, a z elektroniką lubimy się całe życie"</h2>
                    <h2 style=h2_style>"Jesteśmy małą grupą ludzi, która robi to co kocha"</h2>
                    <h2 style=h2_style>"Niesamowicie cieszy nas, że możemy z tego żyć"</h2>
                </div>

                <div class="hero-actions">
                    <A href="/contact" attr:class="btn-primary">
                        <svg class="icon-svg" style="margin-right: 0.5rem;" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"></path>
                        </svg>
                        "Zgłoś Awarię"
                    </A>
                    <A href="/services" attr:class="btn-toggle">"Nasze Usługi"</A>
                </div>
            </div>

            // @oskif
            // Delete style from service-grid selector
            <div class="services-grid">
                <div class="card">
                    <h3 class="service-title" style="color: var(--color-green);">"Pogotowie Serwisowe 24h/7"</h3>
                    <p class="service-desc">
                        "Awarie nie znają harmonogramu pracy. Dysponujemy w pełni wyposażonym mobilnym warsztatem, co pozwala nam na szybki dojazd, diagnozę, a nawet naprawę bezpośrednio przy maszynie klienta o każdej porze."
                    </p>
                </div>

                <div class="card">
                    <h3 class="service-title" style="color: var(--color-blue);">"Kompleksowe Utrzymanie Ruchu"</h3>
                    <p class="service-desc">
                        "Od lutowania mikrokontrolerów na płytach PCB, przez modyfikacje kodu w sterownikach PLC, aż po zarządzanie przemysłową infrastrukturą IT. Jeden partner, pełne wsparcie technologiczne."
                    </p>
                </div>
            </div>
            <div style="margin-top: 4rem;">
                // <Divider text="Zaufali Nam" />
                <RecommendationsCarousel />
            </div>
        </div>
    }
}

use crate::server::state::{PbResponse, Recommendation};
use reqwest;

#[server]
pub async fn get_recommendations() -> Result<Vec<Recommendation>, ServerFnError<String>> {
    let url = "https://db.szoptech.com/api/collections/recommendations/records?sort=-created";

    let response = reqwest::get(url)
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    let pb_data = response
        .json::<PbResponse<Recommendation>>()
        .await
        .map_err(|e| ServerFnError::<String>::ServerError(e.to_string()))?;

    Ok(pb_data.items)
}
