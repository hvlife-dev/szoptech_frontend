use leptos::prelude::*;
use crate::server::db::api::get_collection;
use crate::server::db::model::tag::Tag;

#[server]
pub async fn get_tags() -> Result<Vec<Tag>, ServerFnError<String>> {
    get_collection::<Tag>("tag",None)
        .await
        .map_err(ServerFnError::ServerError)
}
