#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: String,
    pub status: bool
}

pub type TaskId = i64;