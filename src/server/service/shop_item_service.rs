use leptos::{server, server_fn::{ServerFnError, codec::Streaming}};

use crate::server::db::{db_client::{get_collection, get_one}, model::shop_item::ShopItem};

#[server]
pub async fn get_shop_items() -> Result<Vec<ShopItem>, ServerFnError<String>> {
   get_collection::<ShopItem>("shop_item",None)
    .await
    .map_err(ServerFnError::ServerError)
}

#[server]
pub async fn get_shop_item_by_id(
    id: String,
) -> Result<ShopItem, ServerFnError<String>> {
    get_one("shop_item", &id, None)
    .await
    .map_err(ServerFnError::ServerError)
}