use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EngineCommand {
    Active(bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalState {
    pub active: bool,

    pub logs: VecDeque<String>,
    pub toast_counter: usize,
    pub latest_toast: Option<(u8, String)>,

    pub is_api_running: bool,
}

pub type AppState = Arc<RwLock<GlobalState>>;

pub fn init_state() -> AppState {
    Arc::new(RwLock::new(GlobalState {
        active: false,

        logs: VecDeque::new(),
        toast_counter: 0,
        latest_toast: None,

        is_api_running: false,
    }))
}

pub struct ScannerGuard {
    state: AppState,
}

impl ScannerGuard {
    pub fn try_claim(state: AppState) -> Option<Self> {
        let mut data = state.write().unwrap();
        if !data.is_api_running {
            data.is_api_running = true;
            drop(data);
            Some(Self { state })
        } else {
            None
        }
    }
}

impl Drop for ScannerGuard {
    fn drop(&mut self) {
        if let Ok(mut data) = self.state.write() {
            data.is_api_running = false;
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PbResponse<T> {
    pub page: u32,
    #[serde(rename = "perPage")]
    pub per_page: u32,
    #[serde(rename = "totalItems")]
    pub total_items: u32,
    pub items: Vec<T>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectStatus {
    Accepted,
    InProgress,
    Sent,
    Completed,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Project {
    pub id: String,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    pub project_type: String,
    pub company_name: Option<String>,
    pub client: Option<String>,
    pub is_public: bool,
    pub title: String,
    pub description: String,
    pub status: ProjectStatus,
    pub images: Vec<String>,
    pub created: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Client {
    pub id: String,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    pub name: String,
    pub qr_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ClientDashboardData {
    pub client: Client,
    pub projects: Vec<Project>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

// Add this below your Project struct
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Recommendation {
    pub id: String,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    pub company_name: String,
    pub logo: String,
    pub quote: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PricingItem {
    pub id: String,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    pub title: String,
    pub description: String,
    pub estimated_price: String,
    pub express_price: String,
    pub images: Vec<String>,
    pub created: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ShopItem {
    pub id: String,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    pub title: String,
    pub description: String,
    pub price: String,
    pub images: Vec<String>,
    pub created: String,
}
