use crate::server::api::{get_engine_status, send_command};
use crate::server::state::{EngineCommand, GlobalState};
use gloo_timers::callback::Interval;
use leptos::prelude::*;

#[derive(Clone)]
pub struct EngineController {
    pub status: Resource<Result<GlobalState, ServerFnError>>,

    pub set_active: Callback<bool>,
}

pub fn use_engine() -> EngineController {
    let status = Resource::new(|| (), |_| async move { get_engine_status().await });
    Effect::new(move |_| {
        let handle = Interval::new(1000, move || status.refetch());
        move || drop(handle)
    });
    let dispatch = Action::new(|cmd: &EngineCommand| {
        let cmd = cmd.clone();
        async move { send_command(cmd).await }
    });

    let set_active = Callback::new(move |new_val: bool| {
        dispatch.dispatch(EngineCommand::Active(new_val));
        status.update(|data| {
            if let Some(Ok(state)) = data {
                state.active = new_val;
            }
        });
        set_timeout(
            move || status.refetch(),
            std::time::Duration::from_millis(200),
        );
    });

    EngineController { status, set_active }
}
