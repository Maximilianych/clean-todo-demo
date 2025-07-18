use sqlx::SqlitePool;

use crate::domain::{entities::{Task, TaskId}, repositories::{RepositoryError, TaskRepository}};

struct SqliteTaskRepository {
    pool: SqlitePool
}

impl SqliteTaskRepository {
    pub fn new(pool: SqlitePool) -> SqliteTaskRepository {
        SqliteTaskRepository { pool }
    }
}

#[async_trait::async_trait]
impl TaskRepository for SqliteTaskRepository {
    async fn next_id(&mut self) -> TaskId {
        todo!()
    }

    async fn get_all(&self) -> Vec<Task> {
        todo!()
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