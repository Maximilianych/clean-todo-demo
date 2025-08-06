use crate::domain::entities::{Task, TaskId};
use crate::domain::repositories::{TaskRepository, RepositoryError};

pub struct InMemoryTaskRepository {
    tasks: Vec<Task>, // Вектор задач
    last_id: TaskId, // Последний ID
}

impl InMemoryTaskRepository {
    pub fn new() -> InMemoryTaskRepository {
        InMemoryTaskRepository { tasks: Vec::new(), last_id: 0 }
    }
}

#[async_trait::async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn next_id(&mut self) -> TaskId {
        self.last_id += 1;
        self.last_id
    }

    async fn get_all(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    async fn get_by_id(&self, id: TaskId) -> Result<Task, RepositoryError> {
        self.tasks
            .iter()
            .find(|task| task.id == id)
            .cloned()
            .ok_or(RepositoryError::TaskNotFound)
    }

    async fn create(&mut self, task: Task) -> Result<(), RepositoryError> {
        if self.tasks.iter().any(|t| t.id == task.id) {
            return Err(RepositoryError::TaskAlreadyExists);
        }
        self.tasks.push(task);
        Ok(())
    }

    async fn delete(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(RepositoryError::TaskNotFound)
        }
    }

    async fn toggle(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = !task.status;
            Ok(())
        } else {
            Err(RepositoryError::TaskNotFound)
        }
    }
}

// Проверяем реализацию репозитория в памяти
#[cfg(test)]
mod in_memory_task_repository_tests {
    use crate::domain::entities::Task;
    use crate::domain::repositories::{TaskRepository, RepositoryError};
    use crate::infrastructure::in_memory::InMemoryTaskRepository;

    #[tokio::test]
    async fn next_id_increments_correctly() {
        // Проверяем, что next_id правильно инкрементирует ID
        let mut repo = InMemoryTaskRepository::new();
        assert_eq!(repo.next_id().await, 1);
        assert_eq!(repo.next_id().await, 2);
    }

    #[tokio::test]
    async fn create_and_get_all_tasks() {
        // Проверяем создание задачи и получение всех задач
        let mut repo = InMemoryTaskRepository::new();
        let task1 = Task { id: 1, title: "Task 1".to_string(), description: "Desc 1".to_string(), status: false };
        let task2 = Task { id: 2, title: "Task 2".to_string(), description: "Desc 2".to_string(), status: true };

        repo.create(task1.clone()).await.unwrap();
        repo.create(task2.clone()).await.unwrap();

        let all_tasks = repo.get_all().await;
        assert_eq!(all_tasks.len(), 2);
        assert!(all_tasks.contains(&task1));
        assert!(all_tasks.contains(&task2));
    }

    #[tokio::test]
    async fn get_by_id_existing_task() {
        // Проверяем получение существующей задачи по ID
        let mut repo = InMemoryTaskRepository::new();
        let task = Task { id: 1, title: "Test Task".to_string(), description: "Description".to_string(), status: false };
        repo.create(task.clone()).await.unwrap();

        let fetched_task = repo.get_by_id(1).await.unwrap();
        assert_eq!(fetched_task.id, 1);
        assert_eq!(fetched_task.title, "Test Task");
    }

    #[tokio::test]
    async fn get_by_id_non_existing_task() {
        // Проверяем получение несуществующей задачи по ID
        let repo = InMemoryTaskRepository::new();
        let result = repo.get_by_id(99).await;
        assert!(matches!(result, Err(RepositoryError::TaskNotFound)));
    }

    #[tokio::test]
    async fn create_task_already_exists() {
        // Проверяем попытку создать задачу с уже существующим ID
        let mut repo = InMemoryTaskRepository::new();
        let task = Task { id: 1, title: "Task".to_string(), description: "Desc".to_string(), status: false };
        repo.create(task.clone()).await.unwrap();
        let result = repo.create(task.clone()).await;
        assert!(matches!(result, Err(RepositoryError::TaskAlreadyExists)));
    }

    #[tokio::test]
    async fn delete_existing_task() {
        // Проверяем удаление существующей задачи
        let mut repo = InMemoryTaskRepository::new();
        let task = Task { id: 1, title: "Test Task".to_string(), description: "Description".to_string(), status: false };
        repo.create(task.clone()).await.unwrap();

        repo.delete(1).await.unwrap();
        let all_tasks = repo.get_all().await;
        assert!(all_tasks.is_empty());
    }

    #[tokio::test]
    async fn delete_non_existing_task() {
        // Проверяем попытку удалить несуществующую задачу
        let mut repo = InMemoryTaskRepository::new();
        let result = repo.delete(99).await;
        assert!(matches!(result, Err(RepositoryError::TaskNotFound)));
    }

    #[tokio::test]
    async fn toggle_existing_task() {
        // Проверяем переключение статуса существующей задачи
        let mut repo = InMemoryTaskRepository::new();
        let task = Task { id: 1, title: "Test Task".to_string(), description: "Description".to_string(), status: false };
        repo.create(task.clone()).await.unwrap();

        repo.toggle(1).await.unwrap();
        let toggled_task = repo.get_by_id(1).await.unwrap();
        assert!(toggled_task.status);

        repo.toggle(1).await.unwrap();
        let toggled_back_task = repo.get_by_id(1).await.unwrap();
        assert!(!toggled_back_task.status);
    }

    #[tokio::test]
    async fn toggle_non_existing_task() {
        // Проверяем попытку переключить статус несуществующей задачи
        let mut repo = InMemoryTaskRepository::new();
        let result = repo.toggle(99).await;
        assert!(matches!(result, Err(RepositoryError::TaskNotFound)));
    }
}