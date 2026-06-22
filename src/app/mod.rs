use leptos::prelude::*;
use leptos_meta::{Link, MetaTags, Script, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

mod components;
mod hooks;
mod pages;
mod metatags;

use crate::app::components::mt::Mt;
use crate::app::components::navbar::Navbar;
use crate::app::pages::client::ClientPage;
use crate::app::pages::contact::ContactPage;
use crate::app::pages::home::HomePage;
use crate::app::pages::pricing::PricingPage;
use crate::app::pages::product::PricingItemPage;
use crate::app::pages::product::ShopItemPage;
use crate::app::pages::projects::ProjectsPage;
use crate::app::pages::services::ServicesPage;
use crate::app::pages::shopping::ShoppingPage;

use leptos_meta::*;
use crate::app::metatags::seomt::*;


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        //@oskif
        <html lang="pl">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>

        //@oskif
        <Meta name="language" content="pl" />
        <Meta property="og:type" content="website" />
        <Meta property="og:locale" content="pl_PL" />
        <Meta name="geo.region" content="PL" />
        <Meta name="robots" content="index, follow" />

        <Link rel="conditional" href="https://szoptech.com/" />
        

        <Link rel="icon" type_="image/x-icon" href="/favicon.svg"/>

        <Script>
            r#"
              var _paq = window._paq = window._paq || [];
              _paq.push(['enableLinkTracking']);
              (function() {
                var u="//sys.szoptech.com/";
                _paq.push(['setTrackerUrl', u+'c']);
                _paq.push(['setSiteId', '1']);
                var d=document, g=d.createElement('script'), s=d.getElementsByTagName('script')[0];
                g.async=true; g.src=u+'t.js'; s.parentNode.insertBefore(g,s);
              })();
            "#
        </Script>

        <Router>
            <Mt/>
            <SeoMetaTags />
            <Routes transition=true fallback=|| "Page not found.".into_view()>
                <ParentRoute path=path!("/") view=Navbar>
                    // @oskif
                    // Add SeoMetaTags in Router
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("services") view=ServicesPage />
                    <Route path=path!("projects") view=ProjectsPage />
                    <Route path=path!("pricing") view=PricingPage />
                    <Route path=path!("contact") view=ContactPage />
                    <Route path=path!("shopping") view=ShoppingPage />
                    <Route path=path!("/client/:id") view=ClientPage/>
                    <Route path=path!("/store/:id") view=ShopItemPage ssr=leptos_router::SsrMode::Async />
                    <Route path=path!("/estimate/:id") view=PricingItemPage ssr=leptos_router::SsrMode::Async />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
