use crate::domain::{entities::{Task, TaskId}, repositories:: TaskRepository};

pub struct TaskService {
    task_repository: Box<dyn TaskRepository>,
}

impl TaskService {
    pub fn new(task_repository: Box<dyn TaskRepository>) -> TaskService {
        TaskService { task_repository }
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
        let id = self.task_repository.next_id();
        let task = Task { id, title, description, status: false };
        self.task_repository.create(task).map_err(|_| TaskServiceError::TaskAlreadyExists)
    }

    pub fn delete(&mut self, id: TaskId) -> Result<(), TaskServiceError> {
        self.task_repository.delete(id).map_err(|_| TaskServiceError::TaskNotFound)
    }

    pub fn toggle(&mut self, id: TaskId) -> Result<(), TaskServiceError> {
        self.task_repository.toggle(id).map_err(|_| TaskServiceError::TaskNotFound)
    }
}

#[derive(Debug)]
pub enum TaskServiceError {
    MissingTitle,
    MissingDescription,
    TaskNotFound,
    TaskAlreadyExists
}

// Проверяем работу сервиса, используя mockall для имитации поведения TaskRepository
#[cfg(test)]
mod task_service_tests {
    use crate::application::services::{TaskService, TaskServiceError};
    use crate::domain::entities::Task;
    use crate::domain::repositories::{MockTaskRepository, RepositoryError};
    use mockall::predicate::*;

    #[test]
    fn get_all_tasks_returns_empty_vec_if_no_tasks() {
        // Проверяем, что get_all возвращает пустой вектор, если задач нет
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_get_all().times(1).returning(|| vec![]);

        let service = TaskService::new(Box::new(mock_repo));
        let tasks = service.get_all();
        assert!(tasks.is_empty());
    }

    #[test]
    fn get_all_tasks_returns_all_tasks() {
        // Проверяем, что get_all возвращает все задачи
        let mut mock_repo = MockTaskRepository::new();
        let task1 = Task { id: 1, title: "T1".to_string(), description: "D1".to_string(), status: false };
        let task2 = Task { id: 2, title: "T2".to_string(), description: "D2".to_string(), status: true };
        mock_repo.expect_get_all().times(1).returning(move || vec![task1.clone(), task2.clone()]);

        let service = TaskService::new(Box::new(mock_repo));
        let tasks = service.get_all();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
    }

    #[test]
    fn get_by_id_returns_task_if_found() {
        // Проверяем, что get_by_id возвращает задачу, если она найдена
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_get_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |id| Ok(Task { id, title: "Test".to_string(), description: "Desc".to_string(), status: false }));

        let service = TaskService::new(Box::new(mock_repo));
        let result = service.get_by_id(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 1);
    }

    #[test]
    fn get_by_id_returns_error_if_not_found() {
        // Проверяем, что get_by_id возвращает ошибку, если задача не найдена
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_get_by_id()
            .with(eq(99))
            .times(1)
            .returning(|_| Err(RepositoryError::TaskNotFound));

        let service = TaskService::new(Box::new(mock_repo));
        let result = service.get_by_id(99);
        assert!(matches!(result, Err(TaskServiceError::TaskNotFound)));
    }

    #[test]
    fn create_task_success() {
        // Проверяем успешное создание задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_next_id().times(1).returning(|| 1);
        mock_repo.expect_create()
            .with(function(|task: &Task| task.id == 1 && task.title == "New Task" && task.description == "New Description"))
            .times(1)
            .returning(|_| Ok(()));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.create("New Task".to_string(), "New Description".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn create_task_missing_title() {
        // Проверяем создание задачи с отсутствующим заголовком
        let mock_repo = MockTaskRepository::new(); // Mock не будет использоваться, но нужен для создания сервиса
        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.create("".to_string(), "Description".to_string());
        assert!(matches!(result, Err(TaskServiceError::MissingTitle)));
    }

    #[test]
    fn create_task_missing_description() {
        // Проверяем создание задачи с отсутствующим описанием
        let mock_repo = MockTaskRepository::new(); // Mock не будет использоваться
        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.create("Title".to_string(), "".to_string());
        assert!(matches!(result, Err(TaskServiceError::MissingDescription)));
    }

    #[test]
    fn create_task_already_exists() {
        // Проверяем создание задачи, которая уже существует (по ID)
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_next_id().times(1).returning(|| 1);
        mock_repo.expect_create()
            .times(1)
            .returning(|_| Err(RepositoryError::TaskAlreadyExists));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.create("Existing Task".to_string(), "Description".to_string());
        assert!(matches!(result, Err(TaskServiceError::TaskAlreadyExists)));
    }

    #[test]
    fn delete_task_success() {
        // Проверяем успешное удаление задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_delete()
            .with(eq(1))
            .times(1)
            .returning(|_| Ok(()));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.delete(1);
        assert!(result.is_ok());
    }

    #[test]
    fn delete_task_not_found() {
        // Проверяем удаление несуществующей задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_delete()
            .with(eq(99))
            .times(1)
            .returning(|_| Err(RepositoryError::TaskNotFound));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.delete(99);
        assert!(matches!(result, Err(TaskServiceError::TaskNotFound)));
    }

    #[test]
    fn toggle_task_success() {
        // Проверяем успешное переключение статуса задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_toggle()
            .with(eq(1))
            .times(1)
            .returning(|_| Ok(()));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.toggle(1);
        assert!(result.is_ok());
    }

    #[test]
    fn toggle_task_not_found() {
        // Проверяем переключение статуса несуществующей задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_toggle()
            .with(eq(99))
            .times(1)
            .returning(|_| Err(RepositoryError::TaskNotFound));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.toggle(99);
        assert!(matches!(result, Err(TaskServiceError::TaskNotFound)));
    }
}