use crate::modules::helpers::errors::Error;

pub struct Project {
    pub name: String,
    pub description: String,
    pub lifetime: u32,
}

pub trait DataRepo {
    fn count_my_projects(&self) -> Result<usize, Error>;
}

pub struct Data {
    pub projects: Vec<Project>,
}

impl DataRepo for Data {
    fn count_my_projects(&self) -> Result<usize, Error> {
        todo!()
    }
}
