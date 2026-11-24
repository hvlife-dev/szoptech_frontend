use std::fmt::format;

use leptos::prelude::*;
use serde::{Deserialize, de::DeserializeOwned};

use crate::server::state::PbResponse;

const API_URL: &'static str = "http://localhost:8090/api/collections";

fn get_url(collection: &str) -> String {
    String::from(format!("{API_URL}/{collection}/records?sort=-created&perPage=500"))
}

pub async fn get_collection<T>(
    collection: &str,
    expand: Option<&str>
) -> Result<Vec<T>, String>
where
    T: DeserializeOwned,
{
    let mut url = get_url(collection);

    if let Some(expand) = expand {
        url = format!("{}?expand={}", url, expand);
    }

    let response = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!(
            "PocketBase error: {}",
            response.status()
        ));
    }

    let pb_data: PbResponse<T> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(pb_data.items)
}

pub async fn get_one<T>(
    collection: &str,
    id: &str,
    expand: Option<&str>,
    
) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let mut url = format!("{}/{}", get_url(collection), id);

    if let Some(expand) = expand {
        url = format!("{}?expand={}", url, expand);
    }


    let response = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!(
            "PocketBase error: {}",
            response.status()
        ));
    }

    let item: T = response
        .json::<T>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(item)
}
