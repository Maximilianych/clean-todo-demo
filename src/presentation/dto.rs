use crate::domain::entities::Task;

#[derive(serde::Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

#[derive(serde::Serialize)]
pub struct TaskResponse {
    pub id: u32,
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