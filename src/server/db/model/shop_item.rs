use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::server::db::model::{category::Category, pb::PocketBase, producer::Producer, tag::Tag};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pictures {
    pub pictures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItem {
    #[serde(flatten)]
    pub base: PocketBase,
    pub id: String,
    pub title: String,
    pub sub_title: Option<String>,
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub producer: Option<String>,
    pub description: String,
    pub pictures: Option<Pictures>,
    pub specyfication: Option<HashMap<String, String>>,
    pub price: u32,
    pub quantity: u32,
    pub is_available: bool,
}