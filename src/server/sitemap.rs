use axum::{
    http::{StatusCode, header},
    response::IntoResponse,
};
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct PbItemRef {
    id: String,
    updated: String,
}

#[derive(Deserialize)]
struct PbListResponse {
    items: Vec<PbItemRef>,
}

pub async fn generate_sitemap() -> impl IntoResponse {
    let domain = "https://szoptech.com";

    let mut urls = vec![
        format!(
            "<url><loc>{}/</loc><changefreq>weekly</changefreq><priority>1.0</priority></url>",
            domain
        ),
        format!(
            "<url><loc>{}/shopping</loc><changefreq>daily</changefreq><priority>0.9</priority></url>",
            domain
        ),
        format!(
            "<url><loc>{}/contact</loc><changefreq>monthly</changefreq><priority>0.8</priority></url>",
            domain
        ),
    ];

    let shop_url = "https://db.szoptech.com/api/collections/shop_items/records?fields=id,updated";
    if let Ok(res) = reqwest::get(shop_url).await
        && let Ok(data) = res.json::<PbListResponse>().await
    {
        for item in data.items {
            let date = item.updated.split(' ').next().unwrap_or(&item.updated);
            urls.push(format!(
                "<url><loc>{}/store/{}</loc><lastmod>{}</lastmod></url>",
                domain, item.id, date
            ));
        }
    }

    let pricing_url =
        "https://db.szoptech.com/api/collections/pricing_items/records?fields=id,updated";
    if let Ok(res) = reqwest::get(pricing_url).await
        && let Ok(data) = res.json::<PbListResponse>().await
    {
        for item in data.items {
            let date = item.updated.split(' ').next().unwrap_or(&item.updated);
            urls.push(format!(
                "<url><loc>{}/estimate/{}</loc><lastmod>{}</lastmod></url>",
                domain, item.id, date
            ));
        }
    }

    let xml_body = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>"#,
        urls.join("\n")
    );

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml_body,
    )
}
