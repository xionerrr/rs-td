use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
pub enum DBError {
    NotFound,
    AlreadyExists,
}

pub trait DatabaseTable<T> {
    fn get_all(&self) -> Vec<(usize, T)>;
    fn get_one(&self, id: usize) -> Result<(usize, T), DBError>;
    fn create(&mut self, data: T) -> Result<(usize, T), DBError>;
    fn delete(&mut self, id: usize) -> Result<(usize, T), DBError>;
}

#[derive(Debug, Default)]
pub struct DatabaseImpl<T> {
    storage: HashMap<usize, T>,
    last_id: usize,
}

impl<T> DatabaseImpl<T> {
    pub fn new(initial_values: Vec<T>) -> Self {
        let mut data = HashMap::new();
        for (index, item) in initial_values.into_iter().enumerate() {
            data.insert(index, item);
        }

        Self {
            last_id: data.len(),
            storage: data,
        }
    }
}

impl<T: PartialEq + Clone + Ord> DatabaseTable<T> for DatabaseImpl<T> {
    fn get_all(&self) -> Vec<(usize, T)> {
        let mut result = self
            .storage
            .iter()
            .map(|(id, data)| (*id, data.clone()))
            .collect_vec();

        result.sort();
        result
    }

    fn get_one(&self, id: usize) -> Result<(usize, T), DBError> {
        self.storage
            .get(&id)
            .cloned()
            .map(|data| (id, data))
            .ok_or(DBError::NotFound)
    }

    fn create(&mut self, data: T) -> Result<(usize, T), DBError> {
        let already_exists = self.storage.iter().find(|(_, db_item)| **db_item == data);

        if already_exists.is_some() {
            Err(DBError::AlreadyExists)
        } else {
            let id = self.last_id;
            self.last_id += 1;

            self.storage.insert(id, data.clone());
            Ok((id, data))
        }
    }

    fn delete(&mut self, id: usize) -> Result<(usize, T), DBError> {
        if let Some((db_id, data)) = self.storage.remove_entry(&id) {
            Ok((db_id, data))
        } else {
            Err(DBError::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use super::{DatabaseImpl, DatabaseTable};

    #[derive(Eq, Ord, PartialOrd, Default, PartialEq, Clone, Debug)]
    struct DBRow(String);

    #[test]
    fn get_all() {
        let db: DatabaseImpl<DBRow> = DatabaseImpl::<DBRow>::new(vec![
            DBRow("Test".into()),
            DBRow("Test1".into()),
            DBRow("Test2".into()),
        ]);

        let result = db.get_all();
        let expected_result = vec![
            (0, DBRow("Test".into())),
            (1, DBRow("Test1".into())),
            (2, DBRow("Test2".into())),
        ];

        assert_eq!(result, expected_result);
    }

    #[test]
    fn get_one() {
        let db = DatabaseImpl::<DBRow>::new(vec![
            DBRow("Test".into()),
            DBRow("Test1".into()),
            DBRow("Test2".into()),
        ]);

        let result = db.get_one(0).unwrap();
        let expected_result = (0, DBRow("Test".into()));

        assert_eq!(result, expected_result);

        let result = db.get_one(usize::MAX);
        assert!(result.is_err());
    }

    #[test]
    fn create() {
        let mut db = DatabaseImpl::<DBRow>::new(vec![]);

        let result = db.create(DBRow("Test".into())).unwrap();
        let expected_result = (0, DBRow("Test".into()));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn delete() {
        let mut db = DatabaseImpl::<DBRow>::new(vec![DBRow("Test".into())]);

        db.delete(0).unwrap();
        let expected_result = HashMap::new();

        assert_eq!(db.storage, expected_result);
    }
}
