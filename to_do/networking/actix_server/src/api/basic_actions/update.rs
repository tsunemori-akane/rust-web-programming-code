use actix_web::{web::Json, HttpResponse};
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};
use to_do_core::structs::ToDoItem;

/// putting the HeaderToken as a parameter in the update function
/// will execute the FromRequest trait and pass the result into the function
/// if the FromRequest trait passes. This is the same as our Json<ToDoItem>
pub async fn update(
    token: HeaderToken,
    body: Json<ToDoItem>,
) -> Result<HttpResponse, NanoServiceError> {
    println!("Token: {}", token.message);
    let _ = update_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
