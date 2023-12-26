use crate::modules::db::service::{DatabaseImpl, DatabaseTable};

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
    type Model;

    fn get_tasks(&mut self) -> Vec<(usize, Self::Model)>;
    fn get_task(&mut self, id: usize) -> (usize, Self::Model);
    fn create_task(&mut self, name: String) -> (usize, Self::Model);
    fn delete_task(&mut self, id: usize);
}

pub struct TaskRepoImpl {
    db: DatabaseImpl<TaskModel>,
}

impl TaskRepo for TaskRepoImpl {
    type Model = TaskModel;

    fn get_tasks(&mut self) -> Vec<(usize, Self::Model)> {
        todo!()
    }

    fn get_task(&mut self, id: usize) -> (usize, Self::Model) {
        todo!()
    }

    fn create_task(&mut self, name: String) -> (usize, Self::Model) {
        todo!()
    }

    fn delete_task(&mut self, id: usize) {
        todo!()
    }
}
