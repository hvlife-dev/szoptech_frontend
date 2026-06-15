use leptos::prelude::*;
use leptos_router::hooks::use_location;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen(inline_js = r#"
    export function mT(url) {
        window._paq = window._paq || [];
        
        setTimeout(() => {
            window._paq.push(['setCustomUrl', url]);
            window._paq.push(['setDocumentTitle', document.title]);
            window._paq.push(['trackPageView']);
        }, 50);
    }
"#)]
extern "C" {
    fn mT(url: &str);
}

#[component]
pub fn Mt() -> impl IntoView {
    let location = use_location();

    Effect::new(move |_| {
        let path = location.pathname.get();

        #[cfg(feature = "hydrate")]
        {
            mT(&path);
        }
    });

    view! { <></> }
}
