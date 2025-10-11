use rusqlite::params;

use crate::database::{database::Database, database::Table};

struct Project {
    name: String,
    repository_url: String,
}

impl Project {
    fn new(name: String, repository_url: String) -> Project {
        Project {
            name,
            repository_url,
        }
    }
}

impl Table<Project> for Database {
    fn create(&self) {
        self.conn
            .execute(
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
    }

    fn add(&self, data: Project) -> Project {
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
