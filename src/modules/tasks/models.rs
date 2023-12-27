pub enum TaskStatus {
    Todo,
    InProgress,
    Completed,
    Failed,
}

pub struct Task {
    pub id: usize,
    pub task_name: String,
    pub status: TaskStatus,
}
