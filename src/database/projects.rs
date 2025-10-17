use rusqlite::{params, Connection};

use crate::database::{database::Table, Database};

pub struct Project {
    pub name: String,
    pub repository_url: String,
}

pub struct Projects {
    conn: &Connection,
}

impl Projects {
    pub fn init(conn: &Connection) -> Projects {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                repository_url TEXT,
                UNIQUE(name),
                UNIQUE(repository_url)
            )",
            (),
        )
        .unwrap();

        conn.execute(
            "INSERT OR IGNORE INTO projects (name, repository_url) VALUES (?1, ?2)",
            ("Bitcoin Core", "https://github.com/bitcoin/bitcoin"),
        )
        .unwrap();

        Projects { conn }
    }

    pub fn add(&self, data: Project) -> Project {
        self.conn
            .execute(
                "INSERT INTO projects (name, repository_url) VALUES (?1, ?2)",
                (&data.name, &data.repository_url),
            )
            .unwrap();

        data
    }

    fn get(&self, id: usize) -> Project {
        self.conn
            .query_one("SELECT * FROM projects WHERE id = ?1", params![id], |row| {
                Ok(Project {
                    name: row.get_unwrap(1),
                    repository_url: row.get_unwrap(2),
                })
            })
            .unwrap()
    }

    fn modify(&self, id: usize, data: Project) -> Project {
        self.conn
            .execute(
                "UPDATE projects SET name = ?1, repository_url = ?2 WHERE id = ?3",
                params![&data.name, &data.repository_url, id],
            )
            .unwrap();

        data
    }

    fn delete(&self, id: usize) -> bool {
        let rows = self
            .conn
            .execute("DELETE FROM projects WHERE id = ?1", params![id])
            .unwrap();

        rows > 0
    }
}

impl Table<Project> for Database {
    fn add(&self, data: Project) -> Project {
        Projects::add(&self.projects, data)
    }

    fn get(&self, id: usize) -> Project {
        Projects::get(&self.projects, id)
    }

    fn modify(&self, id: usize, data: Project) -> Project {
        Projects::modify(&self.projects, id, data)
    }

    fn delete(&self, id: usize) -> bool {
        Projects::delete(&self.projects, id)
    }
}
