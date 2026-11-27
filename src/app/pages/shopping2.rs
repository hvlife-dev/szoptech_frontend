use leptos::prelude::*;
use leptos_router::components::A;

use crate::server::service::shop_item_service::get_shop_items;

#[component]
pub fn ShoppingPage2() -> impl IntoView {
    let shop_items_resource = Resource::new(
        || (),
        |_| get_shop_items()
    );

    view! {
        <div>
            <Suspense fallback=|| {
                view! { <p>"Ładowanie..."</p> }
            }>
                {move || {
                    shop_items_resource
                        .get()
                        .map(|result| {
                            match result {
                                Ok(items) => {
                                    view! {
                                        <div>
                                            {items
                                                .into_iter()
                                                .map(|item| {
                                                    view! { <span>{item.title}</span> }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => {

                                    view! { <p>{e.to_string()}</p> }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}