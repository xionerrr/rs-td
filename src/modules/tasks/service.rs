use std::fmt::Error;

use super::{repo::TaskRepo, Task, TaskStatus};

pub trait TaskService {
    fn getTasks(&self) -> Result<Vec<Task>, Error>;
    fn createTask(&self, task_name: String) -> Result<Task, Error>;
    fn editTask(&self, task_id: usize, task_status: TaskStatus) -> Result<Task, Error>;
    fn removeTask(&self, task_id: usize) -> Result<Task, Error>;
}

pub struct TaskServiceImpl {
    task_repo: impl TaskRepo,
}

impl TaskService for TaskServiceImpl {
    fn getTasks(&self) -> Result<Vec<Task>, Error> {
        todo!()
    }

    fn createTask(&self, task_name: String) -> Result<Task, Error> {
        self.task_repo.create(task_name);
        todo!()
    }

    fn editTask(&self, task_id: usize, task_status: TaskStatus) -> Result<Task, Error> {
        todo!()
    }

    fn removeTask(&self, task_id: usize) -> Result<Task, Error> {
        todo!()
    }
}
