use std::collections::HashMap;

use crate::modules::{
    db::service::{DatabaseImpl, DatabaseTable},
    helpers::errors::Error,
};

#[derive(Eq, Ord, PartialEq, Clone, PartialOrd, Debug)]
pub struct User {
    name: String,
    last_name: String,
    age: usize,
}

pub trait UserRepo {
    fn get_users(&mut self) -> Result<HashMap<usize, User>, Error>;
    fn get_user(&mut self, user_id: usize) -> Result<User, Error>;
}

impl UserRepo for DatabaseImpl<()> {
    fn get_users(&mut self) -> Result<HashMap<usize, User>, Error> {
        let db: DatabaseImpl<User> = DatabaseImpl::<User>::new(vec![]);
        let users = db.get_all().2;

        if users.is_empty() {
            Err(Error::NotFound)
        } else {
            Ok(users)
        }
    }

    fn get_user(&mut self, user_id: usize) -> Result<User, Error> {
        let db: DatabaseImpl<User> = DatabaseImpl::<User>::new(vec![]);
        let users = db.get_all().2;

        let user_exists = users.iter().find(|user| *user.0 == user_id);

        if user_exists.is_none() {
            Err(Error::NotFound)
        } else {
            Ok(User {
                age: 20,
                last_name: "Test".to_string(),
                name: "Test".to_string(),
            })
        }
    }
}
