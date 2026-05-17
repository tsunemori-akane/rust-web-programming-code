use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::DONE => write!(f, "Status Done"),
            TaskStatus::PENDING => write!(f, "Status Pending"),
        }
    }
}

/// convert string to TaskStatus
impl TaskStatus {
    pub fn from_string(status: &String) -> Result<TaskStatus, NanoServiceError> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(NanoServiceError::new(
                "Invalid status".to_string(),
                NanoServiceErrorStatus::BadRequest,
            )),
        }
    }
}
