use crate::domain::entities::{Task, TaskId};

#[mockall::automock]
#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    // Получение всех задач
    async fn get_all(&self) -> Vec<Task>;
    // Получение задачи по идентификатору
    async fn get_by_id(&self, id: TaskId) -> Result<Task, RepositoryError>;
    // Создание новой задачи
    async fn create(&mut self, task: Task) -> Result<(), RepositoryError>;
    // Удаление задачи
    async fn delete(&mut self, id: TaskId) -> Result<(), RepositoryError>;
    // Переключение статуса задачи
    async fn toggle(&mut self, id: TaskId) -> Result<(), RepositoryError>;
    // Получение следующего доступного идентификатора для новой задачи
    async fn next_id(&mut self) -> TaskId;
}

// Перечисление возможных ошибок, которые могут возникнуть при работе с репозиторием
#[derive(Debug)]
pub enum RepositoryError {
    TaskNotFound, // Задача не найдена
    TaskAlreadyExists, // Задача уже существует
    InternalError // Внутренняя ошибка репозитория
}
