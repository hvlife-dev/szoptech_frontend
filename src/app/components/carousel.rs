use leptos::prelude::*;

#[component]
pub fn ImageCarousel(
    collection_id: String,
    record_id: String,
    images: Vec<String>,
    #[prop(default = String::new())] alt_text: String,
) -> impl IntoView {
    if images.is_empty() {
        return view! { <div></div> }.into_any();
    }

    let images_len = images.len();
    let (current_idx, set_current_idx) = signal(0);
    let (is_zoomed, set_is_zoomed) = signal(false);

    let next_slide = move || {
        set_current_idx.update(|idx| *idx = (*idx + 1) % images_len);
    };

    let prev_slide = move || {
        set_current_idx.update(|idx| {
            if *idx == 0 {
                *idx = images_len - 1;
            } else {
                *idx -= 1;
            }
        });
    };

    let current_img_url = move || {
        let filename = &images[current_idx.get()];
        format!(
            "https://db.szoptech.com/api/files/{}/{}/{}",
            collection_id, record_id, filename
        )
    };

    view! {
        <div class="carousel-container">
            <img
                src=current_img_url.clone()
                alt=alt_text.clone()
                class="carousel-image cursor-zoom"
                on:click=move |_| set_is_zoomed.set(true)
            />

            {if images_len > 1 {
                view! {
                    <button class="carousel-btn prev" on:click=move |_| prev_slide() aria-label="Previous image">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="15 18 9 12 15 6"></polyline>
                        </svg>
                    </button>

                    <button class="carousel-btn next" on:click=move |_| next_slide() aria-label="Next image">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="9 18 15 12 9 6"></polyline>
                        </svg>
                    </button>

                    <div class="carousel-indicators">
                        {(0..images_len).map(|i| {
                            view! {
                                <div
                                    class="carousel-dot"
                                    class=("active", move || current_idx.get() == i)
                                    on:click=move |_| set_current_idx.set(i)
                                ></div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>

        {move || {
            if is_zoomed.get() {
                view! {
                    <div class="carousel-lightbox-overlay" on:click=move |_| set_is_zoomed.set(false)>

                        <button class="lightbox-close" aria-label="Close">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="18" y1="6" x2="6" y2="18"></line>
                                <line x1="6" y1="6" x2="18" y2="18"></line>
                            </svg>
                        </button>

                        <img
                            src=current_img_url()
                            alt=alt_text.clone()
                            class="lightbox-image"
                            on:click=move |e| e.stop_propagation()
                        />

                        {if images_len > 1 {
                            view! {
                                <button class="carousel-btn prev lightbox-btn" on:click=move |e| { e.stop_propagation(); prev_slide(); }>
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round">
                                        <polyline points="15 18 9 12 15 6"></polyline>
                                    </svg>
                                </button>
                                <button class="carousel-btn next lightbox-btn" on:click=move |e| { e.stop_propagation(); next_slide(); }>
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round">
                                        <polyline points="9 18 15 12 9 6"></polyline>
                                    </svg>
                                </button>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }}
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }
        }}
    }.into_any()
}
