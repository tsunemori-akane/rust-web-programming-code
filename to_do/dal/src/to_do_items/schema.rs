use super::enums::TaskStatus;
use glue::errors::NanoServiceError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub id: i32,
    pub title: String,
    pub status: String,
}
