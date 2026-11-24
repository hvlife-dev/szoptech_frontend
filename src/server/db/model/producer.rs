use serde::{Deserialize, Serialize};
use crate::server::db::model::pb::PocketBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producer {
    #[serde(flatten)]
    pub base: PocketBase,
    pub id:String,
    pub name: String,
    pub image: Option<String>
}