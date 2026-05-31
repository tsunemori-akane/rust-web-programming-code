use crate::structs::ToDoItem;

// if json-file-storage feature is activated
#[cfg(feature = "json-file-storage")]
use glue::errors::NanoServiceError;
use to_do_dal::json_file::save_one;

pub async fn create(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let _ = save_one(&item.title.to_string(), &item)?;
    Ok(item)
}
