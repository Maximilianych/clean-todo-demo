use crate::domain::entities::{Task, TaskId};

#[mockall::automock]
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    async fn get_all(&self) -> Vec<Task>;
    async fn get_by_id(&self, id: TaskId) -> Result<Task, RepositoryError>;
    async fn create(&mut self, task: Task) -> Result<(), RepositoryError>;
    async fn delete(&mut self, id: TaskId) -> Result<(), RepositoryError>;
    async fn toggle(&mut self, id: TaskId) -> Result<(), RepositoryError>;
    async fn next_id(&mut self) -> TaskId;
}

#[derive(Debug)]
pub enum RepositoryError {
    TaskNotFound,
    TaskAlreadyExists
}