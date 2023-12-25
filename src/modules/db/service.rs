use std::collections::HashMap;

#[derive(Debug)]
pub enum DBError {
    NotFound,
    AlreadyExists,
}

pub trait DatabaseTable<T> {
    fn get(&self) -> Option<T>;
    fn create(&mut self, data: T) -> Result<(usize, T), DBError>;
    fn edit(&mut self, id: usize, data: T) -> T;
    fn delete(&mut self, id: usize) -> T;
}

#[derive(Debug, Default)]
pub struct DatabaseImpl<T> {
    storage: HashMap<usize, T>,
    last_id: usize,
}

impl<T: PartialEq + Clone> DatabaseTable<T> for DatabaseImpl<T> {
    fn get(&self) -> Option<T> {
        todo!()
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

    fn edit(&mut self, id: usize, data: T) -> T {
        todo!()
    }

    fn delete(&mut self, id: usize) -> T {
        todo!()
    }
}
