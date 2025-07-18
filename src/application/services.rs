use crate::domain::{entities::{Task, TaskId}, repositories::{RepositoryError,  TaskRepository}};

pub struct TaskService {
    task_repository: Box<dyn TaskRepository>,
}

impl TaskService {
    pub fn new(task_repository: Box<dyn TaskRepository>) -> TaskService {
        TaskService { task_repository }
    }

    pub async fn get_all(&self) -> Vec<Task> {
        self.task_repository.get_all().await
    }

    pub async fn get_by_id(&self, id: TaskId) -> Result<Task, TaskServiceError> {
        self.task_repository.get_by_id(id).await.map_err(|e| 
            match e {
                RepositoryError::TaskNotFound => TaskServiceError::TaskNotFound,
                _ => TaskServiceError::UnexpectedError
            })
    }

    pub async fn create(&mut self, title: String, description: String) -> Result<(), TaskServiceError> {
        if title.is_empty() {
            return Err(TaskServiceError::MissingTitle);
        }
        if description.is_empty() {
            return Err(TaskServiceError::MissingDescription);
        }
        let id = self.task_repository.next_id().await;
        let task = Task { id, title, description, status: false };
        self.task_repository.create(task).await.map_err(|e|
            match e {
                RepositoryError::TaskAlreadyExists => TaskServiceError::TaskAlreadyExists,
                _ => TaskServiceError::UnexpectedError
            })
    }

    pub async fn delete(&mut self, id: TaskId) -> Result<(), TaskServiceError> {
        self.task_repository.delete(id).await.map_err(|e| 
            match e {
                RepositoryError::TaskNotFound => TaskServiceError::TaskNotFound,
                _ => TaskServiceError::UnexpectedError
            })
    }

    pub async fn toggle(&mut self, id: TaskId) -> Result<(), TaskServiceError> {
        self.task_repository.toggle(id).await.map_err(|e| 
            match e {
                RepositoryError::TaskNotFound => TaskServiceError::TaskNotFound,
                _ => TaskServiceError::UnexpectedError
            }
        )
    }
}

#[derive(Debug)]
pub enum TaskServiceError {
    MissingTitle,
    MissingDescription,
    TaskNotFound,
    TaskAlreadyExists,
    UnexpectedError
}

// Проверяем работу сервиса, используя mockall для имитации поведения TaskRepository
#[cfg(test)]
mod task_service_tests {
    use crate::application::services::{TaskService, TaskServiceError};
    use crate::domain::entities::Task;
    use crate::domain::repositories::{MockTaskRepository, RepositoryError};
    use mockall::predicate::*;

    #[tokio::test]
    async fn get_all_tasks_returns_empty_vec_if_no_tasks() {
        // Проверяем, что get_all возвращает пустой вектор, если задач нет
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_get_all().times(1).returning(|| vec![]);

        let service = TaskService::new(Box::new(mock_repo));
        let tasks = service.get_all().await;
        assert!(tasks.is_empty());
    }

    #[tokio::test]
    async fn get_all_tasks_returns_all_tasks() {
        // Проверяем, что get_all возвращает все задачи
        let mut mock_repo = MockTaskRepository::new();
        let task1 = Task { id: 1, title: "T1".to_string(), description: "D1".to_string(), status: false };
        let task2 = Task { id: 2, title: "T2".to_string(), description: "D2".to_string(), status: true };
        mock_repo.expect_get_all().times(1).returning(move || vec![task1.clone(), task2.clone()]);

        let service = TaskService::new(Box::new(mock_repo));
        let tasks = service.get_all().await;
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
    }

    #[tokio::test]
    async fn get_by_id_returns_task_if_found() {
        // Проверяем, что get_by_id возвращает задачу, если она найдена
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_get_by_id()
            .with(eq(1))
            .times(1)
            .returning(move |id| Ok(Task { id, title: "Test".to_string(), description: "Desc".to_string(), status: false }));

        let service = TaskService::new(Box::new(mock_repo));
        let result = service.get_by_id(1).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 1);
    }

    #[tokio::test]
    async fn get_by_id_returns_error_if_not_found() {
        // Проверяем, что get_by_id возвращает ошибку, если задача не найдена
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_get_by_id()
            .with(eq(99))
            .times(1)
            .returning(|_| Err(RepositoryError::TaskNotFound));

        let service = TaskService::new(Box::new(mock_repo));
        let result = service.get_by_id(99).await;
        assert!(matches!(result, Err(TaskServiceError::TaskNotFound)));
    }

    #[tokio::test]
    async fn create_task_success() {
        // Проверяем успешное создание задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_next_id().times(1).returning(|| 1);
        mock_repo.expect_create()
            .with(function(|task: &Task| task.id == 1 && task.title == "New Task" && task.description == "New Description"))
            .times(1)
            .returning(|_| Ok(()));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.create("New Task".to_string(), "New Description".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_task_missing_title() {
        // Проверяем создание задачи с отсутствующим заголовком
        let mock_repo = MockTaskRepository::new(); // Mock не будет использоваться, но нужен для создания сервиса
        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.create("".to_string(), "Description".to_string()).await;
        assert!(matches!(result, Err(TaskServiceError::MissingTitle)));
    }

    #[tokio::test]
    async fn create_task_missing_description() {
        // Проверяем создание задачи с отсутствующим описанием
        let mock_repo = MockTaskRepository::new(); // Mock не будет использоваться
        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.create("Title".to_string(), "".to_string()).await;
        assert!(matches!(result, Err(TaskServiceError::MissingDescription)));
    }

    #[tokio::test]
    async fn create_task_already_exists() {
        // Проверяем создание задачи, которая уже существует (по ID)
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_next_id().times(1).returning(|| 1);
        mock_repo.expect_create()
            .times(1)
            .returning(|_| Err(RepositoryError::TaskAlreadyExists));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.create("Existing Task".to_string(), "Description".to_string()).await;
        assert!(matches!(result, Err(TaskServiceError::TaskAlreadyExists)));
    }

    #[tokio::test]
    async fn delete_task_success() {
        // Проверяем успешное удаление задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_delete()
            .with(eq(1))
            .times(1)
            .returning(|_| Ok(()));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.delete(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_task_not_found() {
        // Проверяем удаление несуществующей задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_delete()
            .with(eq(99))
            .times(1)
            .returning(|_| Err(RepositoryError::TaskNotFound));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.delete(99).await;
        assert!(matches!(result, Err(TaskServiceError::TaskNotFound)));
    }

    #[tokio::test]
    async fn toggle_task_success() {
        // Проверяем успешное переключение статуса задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_toggle()
            .with(eq(1))
            .times(1)
            .returning(|_| Ok(()));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.toggle(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn toggle_task_not_found() {
        // Проверяем переключение статуса несуществующей задачи
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_toggle()
            .with(eq(99))
            .times(1)
            .returning(|_| Err(RepositoryError::TaskNotFound));

        let mut service = TaskService::new(Box::new(mock_repo));
        let result = service.toggle(99).await;
        assert!(matches!(result, Err(TaskServiceError::TaskNotFound)));
    }
}