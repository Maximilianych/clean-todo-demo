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
                Vec::new()
            })
    }

    async fn get_by_id(&self, id: TaskId) -> Result<Task, RepositoryError> {
        todo!()
    }

    async fn create(&mut self, task: Task) -> Result<(), RepositoryError> {
        todo!()
    }

    async fn delete(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        todo!()
    }

    async fn toggle(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        todo!()
    }
}

