use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn get_handle(path: Option<&str>) -> Result<File, NanoServiceError> {
    let path = match path {
        Some(p) => p,
        None => &env::var("JSON_STORE_PATH").unwrap_or("./tasks.json".to_string()),
    };
    // let file = OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .create(true)
    //     .open(&path)
    //     .map_err(|e| format!("Failed to open file: {}", e))?;
    // Ok(file)

    safe_eject!(
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path),
        NanoServiceErrorStatus::Unknown,
        "Error writing tasks to JSON to file"
    )
}

pub fn get_all<T: DeserializeOwned>() -> Result<HashMap<String, T>, NanoServiceError> {
    let mut file = get_handle(None)?;
    let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .map_err(|e| format!("Failed to read file: {}", e))?;
    // let tasks: HashMap<String, T> = serde_json::from_str(&contents)
    //     .map_err(|e| format!("Failed to deserialize JSON: {}", e))?;
    safe_eject!(
        file.read_to_string(&mut contents),
        NanoServiceErrorStatus::Unknown,
        "Error reading JSON file to get all tasks"
    )?;
    let task: HashMap<String, T> = safe_eject!(
        serde_json::from_str(&contents),
        NanoServiceErrorStatus::Unknown,
        "Error deserializing JSON to get all tasks"
    )?;
    Ok(task)
}

/// Saves all the tasks to the JSON file.
///
/// # Arguments
/// - `tasks` - a hashmap of tasks.
pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) -> Result<(), NanoServiceError> {
    let mut file = get_handle(None)?;
    let json = safe_eject!(
        serde_json::to_string_pretty(tasks),
        NanoServiceErrorStatus::Unknown,
        "Error serializing JSON before saving tasks"
    )?;
    safe_eject!(
        file.write_all(json.as_bytes()),
        NanoServiceErrorStatus::Unknown,
        "Error writing tasks to JSON to file"
    )?;
    Ok(())
}

/// Gets a task from the JSON file.
///
/// # Arguments
/// - `id` - a string slice that specifies the id of the task.
///
/// # Returns
/// a task.
pub fn get_one<T: DeserializeOwned + Clone>(id: &str) -> Result<T, NanoServiceError> {
    let tasks = get_all::<T>()?;
    match tasks.get(id) {
        Some(t) => Ok(t.clone()),
        None => Err(NanoServiceError::new(
            format!("Task with id {} not found", id),
            NanoServiceErrorStatus::Unknown,
        )),
    }
}

/// Saves a task to the JSON file.
///
/// # Arguments
/// - `id` - a string slice that specifies the id of the task.
/// - `task` - a reference to the task to save.
pub fn save_one<T>(id: &str, task: &T) -> Result<(), NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}

/// Deletes a task from the JSON file.
///
/// # Arguments
/// - `id` - a string slice that specifies the id of the task to delete.
pub fn delete_one<T>(id: &str) -> Result<(), NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or(HashMap::new());
    tasks.remove(id);
    save_all(&tasks)
}
