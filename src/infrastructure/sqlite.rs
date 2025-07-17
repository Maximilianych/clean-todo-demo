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

impl TaskRepository for SqliteTaskRepository {
    fn next_id(&mut self) -> TaskId {
        todo!()
    }

    fn get_all(&self) -> Vec<Task> {
        todo!()
    }

    fn get_by_id(&self, id: TaskId) -> Result<Task, RepositoryError> {
        todo!()
    }

    fn create(&mut self, task: Task) -> Result<(), RepositoryError> {
        todo!()
    }

    fn delete(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        todo!()
    }

    fn toggle(&mut self, id: TaskId) -> Result<(), RepositoryError> {
        todo!()
    }
}