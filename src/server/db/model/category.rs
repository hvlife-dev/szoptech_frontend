use serde::{Deserialize, Serialize};
use crate::server::db::model::pb::PocketBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    #[serde(flatten)]
    pub base: PocketBase,
    pub id:String,
    pub name: String
}