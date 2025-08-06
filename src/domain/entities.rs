#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub id: TaskId, // Уникальный идентификатор задачи
    pub title: String, // Название задачи
    pub description: String, // Описание задачи
    pub status: bool // Статус выполнения задачи
}

// Псевдоним для идентификатора задачи
pub type TaskId = i64;