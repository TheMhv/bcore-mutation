use rusqlite::Connection;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    fn init(path: Option<&str>) -> Database {
        let path = path.unwrap_or("mutation.db");
        let conn = Connection::open(path).unwrap();

        Database { conn }
    }
}

pub trait Table<T> {
    fn create(&self);
    fn add(&self, data: T) -> T;
    fn get(&self, id: usize) -> T;
    fn modify(&self, id: usize, data: T) -> T;
    fn delete(&self, id: usize) -> bool;
}
