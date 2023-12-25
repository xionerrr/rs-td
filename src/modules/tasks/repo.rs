use crate::modules::db::service::DatabaseImpl;

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
    fn create(&mut self, name: String) -> TaskModel;
}

pub struct TaskRepoImpl {
    db: DatabaseImpl<TaskModel>,
}

impl TaskRepo for TaskRepoImpl {
    fn create(&mut self, name: String) -> TaskModel {
        let (_, row) = self.db.create(TaskModel { name }).unwrap();
        row
    }
}
