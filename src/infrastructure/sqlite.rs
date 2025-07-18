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
            .unwrap();

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
        sqlx::query!(r#"INSERT INTO tasks (id, title, description, status) VALUES (?, ?, ?, ?)"#, task.id, task.title, task.description, task.status)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                eprintln!("Ошибка при создании задачи: {:?}", e);
                RepositoryError::InternalError
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
        let affected_rows = sqlx::query!(r#"UPDATE tasks SET status = NOT status WHERE id = ?"#, id)
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