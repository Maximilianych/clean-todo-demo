use sqlx::SqlitePool;

use crate::domain::{
    entities::{Task, TaskId},
    repositories::{RepositoryError, TaskRepository},
};

struct SqliteTaskRepository {
    pool: SqlitePool,
    next_id: TaskId,
}

impl SqliteTaskRepository {
    pub async fn new(pool: SqlitePool) -> SqliteTaskRepository {
        let id = sqlx::query!(r#"SELECT MAX(id) as max_id FROM tasks"#)
            .fetch_optional(&pool)
            .await
            .unwrap()
            .unwrap()
            .max_id
            .unwrap_or(0);

        SqliteTaskRepository {
            pool,
            next_id: id as TaskId,
        }
    }
}

#[async_trait::async_trait]
impl TaskRepository for SqliteTaskRepository {
    async fn next_id(&mut self) -> TaskId {
        self.next_id += 1;
        self.next_id
    }

    async fn get_all(&self) -> Vec<Task> {
        sqlx::query_as!(Task, r#"SELECT id as "id!", title as "title!", description as "description!", status as "status!" FROM tasks"#)
            .fetch_all(&self.pool)
            .await
            .unwrap_or_else(|e| {
                eprintln!("Ошибка при получении всех задач: {:?}", e);
                Vec::new()
            })
    }

    async fn get_by_id(&self, id: TaskId) -> Result<Task, RepositoryError> {
        let task = sqlx::query_as!(Task, r#"SELECT id as "id!", title as "title!", description as "description!", status as "status!" FROM tasks WHERE id = ?"#, id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                eprintln!("Ошибка при получении задачи по ID: {:?}", e);
                RepositoryError::InternalError
            })?;
        task.ok_or(RepositoryError::TaskNotFound)
    }

    async fn create(&mut self, task: Task) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"INSERT INTO tasks (id, title, description, status) VALUES (?, ?, ?, ?)"#,
            task.id,
            task.title,
            task.description,
            task.status
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            eprintln!("Ошибка при создании задачи: {:?}", e);
            match e {
                sqlx::Error::Database(e) if e.code().unwrap_or_default() == "1555" => {
                    RepositoryError::TaskAlreadyExists
                }
                _ => RepositoryError::InternalError,
            }
        })?;
        Ok(())
    }

    async fn delete(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        let affected_rows = sqlx::query!(r#"DELETE FROM tasks WHERE id = ?"#, id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                eprintln!("Ошибка при удалении задачи: {:?}", e);
                RepositoryError::InternalError
            })?
            .rows_affected();

        if affected_rows == 0 {
            Err(RepositoryError::TaskNotFound)
        } else {
            Ok(())
        }
    }

    async fn toggle(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        let affected_rows =
            sqlx::query!(r#"UPDATE tasks SET status = NOT status WHERE id = ?"#, id)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    eprintln!("Ошибка при переключении статуса задачи: {:?}", e);
                    RepositoryError::InternalError
                })?
                .rows_affected();

        if affected_rows == 0 {
            Err(RepositoryError::TaskNotFound)
        } else {
            Ok(())
        }
    }
}

// Проверяем реализацию репозитория c SQLite
#[cfg(test)]
mod sqlite_task_repository_tests {
    use std::path::Path;

    use sqlx::SqlitePool;
    use sqlx::migrate::Migrator;

    use crate::domain::entities::Task;
    use crate::domain::repositories::{RepositoryError, TaskRepository};
    use crate::infrastructure::sqlite::SqliteTaskRepository;

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to in-memory SQLite");

        Migrator::new(Path::new("./migrations"))
            .await
            .expect("Failed to create migrator")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn create_and_get_all_tasks() {
        // Проверяем создание задачи и получение всех задач
        let pool = setup_db().await;
        let mut repo = SqliteTaskRepository::new(pool).await;

        let task1 = Task {
            id: repo.next_id().await,
            title: "Task 1".to_string(),
            description: "Desc 1".to_string(),
            status: false,
        };
        let task2 = Task {
            id: repo.next_id().await,
            title: "Task 2".to_string(),
            description: "Desc 2".to_string(),
            status: true,
        };

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
        let pool = setup_db().await;
        let mut repo = SqliteTaskRepository::new(pool).await;

        let task = Task {
            id: repo.next_id().await,
            title: "Test Task".to_string(),
            description: "Description".to_string(),
            status: false,
        };
        repo.create(task.clone()).await.unwrap();

        let fetched_task = repo.get_by_id(task.id).await.unwrap();
        assert_eq!(fetched_task.id, task.id);
        assert_eq!(fetched_task.title, "Test Task");
    }

    #[tokio::test]
    async fn get_by_id_non_existing_task() {
        // Проверяем получение несуществующей задачи по ID
        let pool = setup_db().await;
        let repo = SqliteTaskRepository::new(pool).await;
        let result = repo.get_by_id(99).await;
        assert!(matches!(result, Err(RepositoryError::TaskNotFound)));
    }

    #[tokio::test]
    async fn create_task_already_exists() {
        // Проверяем попытку создать задачу с уже существующим ID
        let pool = setup_db().await;
        let mut repo = SqliteTaskRepository::new(pool).await;
        let task = Task {
            id: repo.next_id().await,
            title: "Task".to_string(),
            description: "Desc".to_string(),
            status: false,
        };
        repo.create(task.clone()).await.unwrap();
        let result = repo.create(task.clone()).await;
        assert!(matches!(result, Err(RepositoryError::TaskAlreadyExists)));
    }

    #[tokio::test]
    async fn delete_existing_task() {
        // Проверяем удаление существующей задачи
        let pool = setup_db().await;
        let mut repo = SqliteTaskRepository::new(pool).await;
        let task = Task {
            id: repo.next_id().await,
            title: "Test Task".to_string(),
            description: "Description".to_string(),
            status: false,
        };
        repo.create(task.clone()).await.unwrap();

        repo.delete(task.id).await.unwrap();
        let all_tasks = repo.get_all().await;
        assert!(all_tasks.is_empty());
    }

    #[tokio::test]
    async fn delete_non_existing_task() {
        // Проверяем попытку удалить несуществующую задачу
        let pool = setup_db().await;
        let mut repo = SqliteTaskRepository::new(pool).await;
        let result = repo.delete(99).await;
        assert!(matches!(result, Err(RepositoryError::TaskNotFound)));
    }

    #[tokio::test]
    async fn toggle_existing_task() {
        // Проверяем переключение статуса существующей задачи
        let pool = setup_db().await;
        let mut repo = SqliteTaskRepository::new(pool).await;
        let task = Task {
            id: repo.next_id().await,
            title: "Test Task".to_string(),
            description: "Description".to_string(),
            status: false,
        };
        repo.create(task.clone()).await.unwrap();

        repo.toggle(task.id).await.unwrap();
        let toggled_task = repo.get_by_id(task.id).await.unwrap();
        assert!(toggled_task.status);

        repo.toggle(task.id).await.unwrap();
        let toggled_back_task = repo.get_by_id(task.id).await.unwrap();
        assert!(!toggled_back_task.status);
    }

    #[tokio::test]
    async fn toggle_non_existing_task() {
        // Проверяем попытку переключить статус несуществующей задачи
        let pool = setup_db().await;
        let mut repo = SqliteTaskRepository::new(pool).await;
        let result = repo.toggle(99).await;
        assert!(matches!(result, Err(RepositoryError::TaskNotFound)));
    }
}