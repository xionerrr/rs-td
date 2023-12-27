#[cfg(test)]
use crate::modules::projects::service::{Data, Project};

#[test]
fn count_my_projects() {
    let project = Project {
        name: "Test".to_string(),
        description: "Test description".to_string(),
        lifetime: 60,
    };
    let date = Data::init(vec![project]);

    let result = date.projects.len();

    assert!(result > 0);
}

#[test]
fn get_projects() {
    let project1 = Project {
        name: "Test".to_string(),
        description: "Test description".to_string(),
        lifetime: 60,
    };
    let project2 = Project {
        name: "Test".to_string(),
        description: "Test description".to_string(),
        lifetime: 60,
    };
    let date = Data::init(vec![project1.clone(), project2.clone()]);

    let result = date.projects;

    assert_eq!(result, vec![project1, project2]);
}
