use actix_web::{web::Json, HttpResponse};
use glue::errors::NanoServiceError;
use to_do_core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};
use to_do_dal::to_do_items::schema::ToDoItem;
use to_do_dal::to_do_items::transactions::{get::GetAll, update::UpdateOne};
/// putting the HeaderToken as a parameter in the update function
/// will execute the FromRequest trait and pass the result into the function
/// if the FromRequest trait passes. This is the same as our Json<ToDoItem>
pub async fn update<T: UpdateOne + GetAll>(
    body: Json<ToDoItem>,
) -> Result<HttpResponse, NanoServiceError> {
    let _ = update_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}
