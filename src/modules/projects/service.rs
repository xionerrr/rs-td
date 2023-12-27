use crate::modules::helpers::errors::Error;

#[derive(Clone, PartialEq, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub lifetime: u32,
}

pub trait DataRepo {
    fn count_my_projects(&self) -> Result<usize, Error>;
    fn get_projects(&self) -> Vec<Project>;
}

#[derive(Default)]
pub struct Data {
    pub projects: Vec<Project>,
}

impl Data {
    pub fn init(initial_values: Vec<Project>) -> Self {
        let mut result = Vec::new();
        for project in initial_values {
            result.push(project);
        }

        Self { projects: result }
    }
}

impl DataRepo for Data {
    fn count_my_projects(&self) -> Result<usize, Error> {
        let count = self.projects.len();

        if count > 0 {
            Ok(count)
        } else {
            Err(Error::NotFound)
        }
    }

    fn get_projects(&self) -> Vec<Project> {
        let projects = &self.projects;
        projects.to_vec()
    }
}
