use std::fmt::Error;

use super::{repo::TaskRepo, Task, TaskStatus};

pub trait TaskService {
    fn getTasks(&self) -> Result<Vec<Task>, Error>;
    fn createTask(&mut self, task_name: String) -> Result<Task, Error>;
    fn editTask(&self, task_id: usize, task_status: TaskStatus) -> Result<Task, Error>;
    fn removeTask(&self, task_id: usize) -> Result<Task, Error>;
}

pub struct TaskServiceImpl {
    task_repo: dyn TaskRepo,
}

impl TaskService for TaskServiceImpl {
    fn getTasks(&self) -> Result<Vec<Task>, Error> {
        self.task_repo.get_tasks();
        todo!()
    }

    fn createTask(&mut self, task_name: String) -> Result<Task, Error> {
        self.task_repo.create_task(task_name);
        todo!()
    }

    fn editTask(&self, task_id: usize, task_status: TaskStatus) -> Result<Task, Error> {
        todo!()
    }

    fn removeTask(&self, task_id: usize) -> Result<Task, Error> {
        todo!()
    }
}
