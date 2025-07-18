use actix_web::HttpResponse;

use crate::{application::services::TaskServiceError, domain::entities::{Task, TaskId}};

#[derive(serde::Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

#[derive(serde::Serialize)]
pub struct TaskResponse {
    pub id: TaskId,
    pub title: String,
    pub description: String,
    pub status: bool,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        TaskResponse {
            id: task.id,
            title: task.title,
            description: task.description,
            status: task.status,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl From<TaskServiceError> for HttpResponse {
    fn from(e: TaskServiceError) -> Self {
        let response = match e {
            TaskServiceError::MissingTitle => ApiErrorResponse {
                code: "validation_failed".to_string(),
                message: "Title is required".to_string(),
                details: None,
            },
            TaskServiceError::MissingDescription => ApiErrorResponse {
                code: "validation_failed".to_string(),
                message: "Description is required".to_string(),
                details: None,
            },
            TaskServiceError::TaskNotFound => ApiErrorResponse {
                code: "not_found".to_string(),
                message: "Task not found".to_string(),
                details: None,
            },
            TaskServiceError::TaskAlreadyExists => ApiErrorResponse {
                code: "conflict".to_string(),
                message: "Task already exists".to_string(),
                details: None,
            }
        };

        match e {
            TaskServiceError::MissingTitle | TaskServiceError::MissingDescription => {
                HttpResponse::BadRequest().json(response)
            },
            TaskServiceError::TaskNotFound => {
                HttpResponse::NotFound().json(response)
            },
            TaskServiceError::TaskAlreadyExists => {
                HttpResponse::Conflict().json(response)
            }
        }
    }
}
