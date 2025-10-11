use rusqlite::params;

use crate::database::{database::Database, database::Table};

struct Run {
    project_id: usize,
    commit_hash: String,
    pr_number: usize,
    tool_version: String,
}

impl Run {
    fn new(project_id: usize, commit_hash: String, pr_number: usize, tool_version: String) -> Run {
        Run {
            project_id,
            commit_hash,
            pr_number,
            tool_version,
        }
    }
}

impl Table<Run> for Database {
    fn create(&self) {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS runs (
                    id INTEGER PRIMARY KEY,
                    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
                    commit_hash TEXT NOT NULL,
                    pr_number INTEGER,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    tool_version TEXT
                )",
                (),
            )
            .unwrap();

        self.conn
            .execute(
                "CREATE INDEX IF NOT EXISTS idx_runs_project_created ON runs(project_id, created_at DESC)",
                (),
            )
            .unwrap();

        self.conn
            .execute(
                "CREATE INDEX IF NOT EXISTS idx_runs_commit ON runs(commit_hash)",
                (),
            )
            .unwrap();
    }

    fn add(&self, data: Run) -> Run {
        self.conn.execute("INSERT INTO runs (project_id, commit_hash, pr_number, tool_version) VALUES (?1, ?2, ?3, ?4)", params![&data.project_id, &data.commit_hash, &data.pr_number, &data.tool_version]).unwrap();

        data
    }

    fn get(&self, id: usize) -> Run {
        self.conn
            .query_one(
                "SELECT (project_id, commit_hash, pr_number, tool_version) FROM runs WHERE id = ?1",
                params![id],
                |row| {
                    Ok(Run {
                        project_id: row.get_unwrap(0),
                        commit_hash: row.get_unwrap(1),
                        pr_number: row.get_unwrap(2),
                        tool_version: row.get_unwrap(3),
                    })
                },
            )
            .unwrap()
    }

    fn modify(&self, id: usize, data: Run) -> Run {
        self.conn.execute(
                "UPDATE runs SET project_id = ?1, commit_hash = ?2, pr_number = ?3, tool_version = ?4 WHERE id = ?5",
                params![&data.project_id, &data.commit_hash, &data.pr_number, &data.tool_version, id])
            .unwrap();

        data
    }

    fn delete(&self, id: usize) -> bool {
        let rows = self
            .conn
            .execute("DELETE FROM runs WHERE id = ?1", params![id])
            .unwrap();

        rows > 0
    }
}
