use std::collections::HashMap;

use itertools::Itertools;

use crate::modules::helpers::errors::Error;

pub trait DatabaseTable<T> {
    fn get_all(&self) -> Vec<(usize, T)>;
    fn get_one(&self, id: usize) -> Result<(usize, T), Error>;
    fn create(&mut self, data: T) -> Result<(usize, T), Error>;
    fn delete(&mut self, id: usize) -> Result<(usize, T), Error>;
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

    fn get_one(&self, id: usize) -> Result<(usize, T), Error> {
        self.storage
            .get(&id)
            .cloned()
            .map(|data| (id, data))
            .ok_or(Error::NotFound)
    }

    fn create(&mut self, data: T) -> Result<(usize, T), Error> {
        let already_exists = self.storage.iter().find(|(_, db_item)| **db_item == data);

        if already_exists.is_some() {
            Err(Error::AlreadyExists)
        } else {
            let id = self.last_id;
            self.last_id += 1;

            self.storage.insert(id, data.clone());
            Ok((id, data))
        }
    }

    fn delete(&mut self, id: usize) -> Result<(usize, T), Error> {
        if let Some((db_id, data)) = self.storage.remove_entry(&id) {
            Ok((db_id, data))
        } else {
            Err(Error::NotFound)
        }
    }
}
