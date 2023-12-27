#[cfg(test)]
use crate::modules::db::service::{DatabaseImpl, DatabaseTable};

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

    let result2 = db.create(DBRow("Test".into()));

    assert!(result2.is_err());
}

#[test]
fn delete() {
    let mut db = DatabaseImpl::<DBRow>::new(vec![DBRow("Test".into())]);

    db.delete(0).unwrap();
    let result = db.get_one(0);

    assert!(result.is_err());

    let (id, _) = db.create(DBRow("Test".into())).unwrap();

    assert!(id != 0);
}
