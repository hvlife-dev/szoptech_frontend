use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocketBase {
    #[serde(rename = "collectionId")]
    pub collection_id: String,

    #[serde(rename = "collectionName")]
    pub collection_name: String,
    
    pub created: Option<String>,
    pub updated: Option<String>,
}