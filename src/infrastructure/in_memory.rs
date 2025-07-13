use crate::domain::entities::{Task, TaskId};
use crate::domain::repositories::{TaskRepository, RepositoryError};

pub struct InMemoryTaskRepository {
    tasks: Vec<Task>,
    last_id: TaskId,
}

impl InMemoryTaskRepository {
    pub fn new() -> InMemoryTaskRepository {
        InMemoryTaskRepository { tasks: Vec::new(), last_id: 0 }
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn next_id(&mut self) -> TaskId {
        self.last_id += 1;
        self.last_id
    }

    fn get_all(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    fn get_by_id(&self, id: TaskId) -> Result<Task, RepositoryError> {
        self.tasks
            .iter()
            .find(|task| task.id == id)
            .cloned()
            .ok_or(RepositoryError::TaskNotFound)
    }

    fn create(&mut self, task: Task) -> Result<(), RepositoryError> {
        if self.tasks.iter().any(|t| t.id == task.id) {
            return Err(RepositoryError::TaskAlreadyExists);
        }
        self.tasks.push(task);
        Ok(())
    }

    fn delete(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(RepositoryError::TaskNotFound)
        }
    }

    fn toggle(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = !task.status;
            Ok(())
        } else {
            Err(RepositoryError::TaskNotFound)
        }
    }
}