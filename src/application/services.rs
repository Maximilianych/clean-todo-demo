use crate::domain::{entities::{Task, TaskId}, repositories:: TaskRepository};

struct TaskService {
    task_repository: Box<dyn TaskRepository>,
    last_id: TaskId
}

impl TaskService {
    pub fn new(task_repository: Box<dyn TaskRepository>) -> TaskService {
        TaskService { task_repository, last_id: 0 }
    }

    pub fn get_all(&self) -> Vec<Task> {
        self.task_repository.get_all()
    }

    pub fn get_by_id(&self, id: TaskId) -> Result<Task, TaskServiceError> {
        self.task_repository.get_by_id(id).map_err(|_| TaskServiceError::TaskNotFound)
    }

    pub fn create(&mut self, title: String, description: String) -> Result<(), TaskServiceError> {
        if title.is_empty() {
            return Err(TaskServiceError::MissingTitle);
        }
        if description.is_empty() {
            return Err(TaskServiceError::MissingDescription);
        }
        let task = Task { id: 0, title, description, status: false };
        self.task_repository.create(task).map_err(|_| TaskServiceError::TaskAlreadyExists)
    }

    pub fn delete(&mut self, id: TaskId) -> Result<(), TaskServiceError> {
        self.task_repository.delete(id).map_err(|_| TaskServiceError::TaskNotFound)
    }

    pub fn toggle(&mut self, id: TaskId) -> Result<(), TaskServiceError> {
        self.task_repository.toggle(id).map_err(|_| TaskServiceError::TaskNotFound)
    }
}

enum TaskServiceError {
    MissingTitle,
    MissingDescription,
    TaskNotFound,
    TaskAlreadyExists
}