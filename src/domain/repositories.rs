use crate::domain::entities::{Task, TaskId};

pub trait TaskRepository: Send + Sync {
    fn get_all(&self) -> Vec<Task>;
    fn get_by_id(&self, id: TaskId) -> Result<Task, RepositoryError>;
    fn create(&mut self, task: Task) -> Result<(), RepositoryError>;
    fn delete(&mut self, id: TaskId) -> Result<(), RepositoryError>;
    fn toggle(&mut self, id: TaskId) -> Result<(), RepositoryError>;
}

pub enum RepositoryError {
    TaskNotFound,
    TaskAlreadyExists,
    TaskIdNotFound,
}