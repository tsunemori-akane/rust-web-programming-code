use actix_web::{web::Json, HttpResponse};
use glue::errors::NanoServiceError;
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_core::structs::ToDoItem;

pub async fn create(body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    let _ = create_core(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(get_all_core().await?))
}
