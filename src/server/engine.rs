use crate::server::misc::log;
use crate::server::state::AppState;
use tokio::time::{Duration, sleep};

pub async fn run_lyrics_engine(state: AppState) {
    log(&state, 1, "Frontend ready");

    loop {
        sleep(Duration::from_millis(100)).await;
    }
}
