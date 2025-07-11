use crate::domain::entities::{Task, TaskId};
use crate::domain::repositories::{TaskRepository, RepositoryError};

pub struct InMemoryTaskRepository {
    tasks: Vec<Task>,
}

impl TaskRepository for InMemoryTaskRepository {
    fn get_all(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    fn get_by_id(&self, id: TaskId) -> Result<Task, RepositoryError> {
        self.tasks
            .iter()
            .find(|task| task.id == id)
            .cloned()
            .ok_or(RepositoryError::TaskIdNotFound)
    }

    fn create(&mut self, task: Task) -> Result<(), RepositoryError> {
        if self.tasks.iter().any(|task| task.id == task.id) {
            return Err(RepositoryError::TaskAlreadyExists);
        }
        self.tasks.push(task);
        Ok(())
    }

    fn delete(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(RepositoryError::TaskNotFound)
        }
    }

    fn toggle(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.status = !task.status;
            Ok(())
        } else {
            Err(RepositoryError::TaskNotFound)
        }
    }
}

impl InMemoryTaskRepository {
    pub fn new() -> InMemoryTaskRepository {
        InMemoryTaskRepository { tasks: Vec::new() }
    }
}