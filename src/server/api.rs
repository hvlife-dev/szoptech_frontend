use crate::server::state::{EngineCommand, GlobalState};
use leptos::prelude::*;

#[server(GetEngineStatus, "/api")]
pub async fn get_engine_status() -> Result<GlobalState, ServerFnError> {
    use crate::server::state::AppState;
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;

    let data = state
        .read()
        .map_err(|_| ServerFnError::new("Lock poisoned"))?
        .clone();

    Ok(data)
}

#[server(SendCommand, "/api/command")]
pub async fn send_command(cmd: EngineCommand) -> Result<(), ServerFnError> {
    use crate::server::state::AppState;
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;

    let mut data = state
        .write()
        .map_err(|_| ServerFnError::new("Lock error"))?;

    match cmd {
        EngineCommand::Active(v) => {
            data.active = v;
        }
    }

    Ok(())
}
