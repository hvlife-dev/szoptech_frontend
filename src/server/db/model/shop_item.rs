use serde::{Deserialize, Serialize};
use crate::server::db::model::{category::Category, pb::PocketBase, producer::Producer, tag::Tag};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItem {
    #[serde(flatten)]
    pub base: PocketBase,
    pub id: String,
    pub title: String,
    pub sub_title: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub category: Option<Category>,
    pub producer: Option<Producer>,
    pub description: String,
    pub specyfication: Option<Vec<(String,String)>>,
    pub price: u32,
    pub is_avialable: bool
}