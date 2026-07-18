use glue::errors::NanoServiceError;
// use to_do_dal::json_file::get_all as get_all_handle;
use to_do_dal::to_do_items::schema::AllToDOItems;
use to_do_dal::to_do_items::transactions::get::GetAll;

pub async fn get_all<T: GetAll>() -> Result<AllToDOItems, NanoServiceError> {
    let all_items = T::get_all().await?;
    AllToDOItems::from_vec(all_items)
}

// Gets a task by its name from the JSON file.
// pub async fn get_by_name(name: &str) -> Result<ToDoItem, NanoServiceError> {
//     Ok(get_all_handle::<ToDoItem>()?
//         .remove(name)
//         .ok_or(NanoServiceError::new(
//             format!("Item with name {} not found", name),
//             NanoServiceErrorStatus::NotFound,
//         ))?)
// }
