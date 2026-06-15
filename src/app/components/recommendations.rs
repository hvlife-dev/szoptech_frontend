use leptos::prelude::*;
use crate::app::pages::home::get_recommendations;

#[component]
pub fn RecommendationsCarousel() -> impl IntoView {
    let recs_resource = Resource::new(|| (), |_| async move {
        get_recommendations().await.unwrap_or_default()
    });

    view! {
        <div class="recommendations-section">
            <Suspense fallback=move || view! { <div></div> }>
                <div class="marquee-container">
                    <div class="marquee-track">
                        {move || Suspend::new(async move {
                            let items = recs_resource.await;

                            if items.is_empty() {
                                return view! { <div></div> }.into_any();
                            }

                            let double_items = [items.clone(), items.clone(), items.clone(), items.clone(), items.clone(), items].concat();

                            double_items.into_iter().map(|item| {
                                let img_url = format!(
                                    "https://db.szoptech.com/api/files/{}/{}/{}",
                                    item.collection_id, item.id, item.logo
                                );

                                view! {
                                    <div class="rec-card">
                                        <img src=img_url alt=item.company_name class="rec-logo"/>
                                        <p class="rec-quote">"\"" {item.quote} "\""</p>
                                    </div>
                                }
                            }).collect_view().into_any()
                        })}
                    </div>
                </div>
            </Suspense>
        </div>
    }
}
