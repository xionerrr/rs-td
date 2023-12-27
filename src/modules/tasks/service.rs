use std::fmt::Error;

use crate::modules::helpers::statuses::TaskStatus;

use super::repo::TaskRepo;

pub struct Task {
    pub id: usize,
    pub task_name: String,
    pub status: TaskStatus,
}

pub trait TaskService {
    fn getTasks(&mut self) -> Result<Vec<Task>, Error>;
    fn createTask(&mut self, task_name: String) -> Result<Task, Error>;
    fn editTask(&mut self, task_id: usize, task_status: TaskStatus) -> Result<Task, Error>;
    fn removeTask(&mut self, task_id: usize) -> Result<Task, Error>;
}

pub struct TaskServiceImpl<T: TaskRepo> {
    task_repo: T,
}

impl<T: TaskRepo> TaskService for TaskServiceImpl<T> {
    fn getTasks(&mut self) -> Result<Vec<Task>, Error> {
        self.task_repo.get_tasks();
        todo!()
    }

    fn createTask(&mut self, task_name: String) -> Result<Task, Error> {
        self.task_repo.create_task(task_name);
        todo!()
    }

    fn editTask(&mut self, task_id: usize, task_status: TaskStatus) -> Result<Task, Error> {
        todo!()
    }

    fn removeTask(&mut self, task_id: usize) -> Result<Task, Error> {
        todo!()
    }
}
