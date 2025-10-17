use crate::database::{Mutants, Projects, Runs};
use rusqlite::Connection;

pub struct Database {
    conn: Connection,
    pub projects: Projects,
    pub runs: Runs,
    pub mutants: Mutants,
}

impl Database {
    pub fn init(path: Option<&str>) -> Database {
        let path = path.unwrap_or("mutation.db");
        let conn = Connection::open(path).unwrap();

        let projects = Projects::init(&conn);
        let runs = Runs::init(&conn);
        let mutants = Mutants::init(&conn);

        Database {
            conn,
            projects,
            runs,
            mutants,
        }
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}

// CRUD SYSTEM
pub trait Table<T> {
    fn add(&self, data: T) -> T;
    fn get(&self, id: usize) -> T;
    fn modify(&self, id: usize, data: T) -> T;
    fn delete(&self, id: usize) -> bool;
}
