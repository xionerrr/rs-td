use crate::modules::db::service::{DatabaseImpl, DatabaseTable};

use super::Task;

#[derive(Clone)]
pub struct TaskModel {
    pub name: String,
}
impl PartialEq for TaskModel {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub trait TaskRepo {
    fn get_tasks(&mut self) -> Vec<Task>;
    fn create_task(&mut self, name: String) -> TaskModel;
}

pub struct TaskRepoImpl {
    db: DatabaseImpl<TaskModel>,
}

impl TaskRepo for TaskRepoImpl {
    fn get_tasks(&mut self) -> Vec<Task> {
        self.db.get_all()
    }

    fn create_task(&mut self, name: String) -> TaskModel {
        let (_, row) = self.db.create(TaskModel { name }).unwrap();
        row
    }
}
