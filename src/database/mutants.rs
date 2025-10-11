use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use rusqlite::{
    params,
    types::{FromSql, FromSqlResult, ValueRef},
};

use crate::database::{database::Database, database::Table};

enum MutantStatus {
    Pending,
    Running,
    Killed,
    Survived,
    Timeout,
    Error,
    Skipped,
    Equivalent,
    Unproductive,
}

struct Mutant {
    run_id: usize,
    diff: String,
    patch_hash: String,
    status: MutantStatus,
    killed: bool,
    command_to_test: String,
    file_path: String,
    operator: String,
}

impl Mutant {
    fn new(
        run_id: usize,
        diff: String,
        patch_hash: String,
        status: MutantStatus,
        killed: bool,
        command_to_test: String,
        file_path: String,
        operator: String,
    ) -> Mutant {
        Mutant {
            run_id,
            diff,
            patch_hash,
            status,
            killed,
            command_to_test,
            file_path,
            operator,
        }
    }
}

impl Display for MutantStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MutantStatus::Pending => write!(f, "pending"),
            MutantStatus::Running => write!(f, "running"),
            MutantStatus::Killed => write!(f, "killed"),
            MutantStatus::Survived => write!(f, "survived"),
            MutantStatus::Timeout => write!(f, "timeout"),
            MutantStatus::Error => write!(f, "error"),
            MutantStatus::Skipped => write!(f, "skipped"),
            MutantStatus::Equivalent => write!(f, "equivalent"),
            MutantStatus::Unproductive => write!(f, "unproductive"),
        }
    }
}

impl FromSql for MutantStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| {
            MutantStatus::from_str(s).map_err(|_| rusqlite::types::FromSqlError::InvalidType)
        })
    }
}

impl FromStr for MutantStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<MutantStatus, Self::Err> {
        match s {
            "pending" => Ok(MutantStatus::Pending),
            "running" => Ok(MutantStatus::Running),
            "killed" => Ok(MutantStatus::Killed),
            "survived" => Ok(MutantStatus::Survived),
            "timeout" => Ok(MutantStatus::Timeout),
            "error" => Ok(MutantStatus::Error),
            "skipped" => Ok(MutantStatus::Skipped),
            "equivalent" => Ok(MutantStatus::Equivalent),
            "unproductive" => Ok(MutantStatus::Unproductive),
            _ => Err(()),
        }
    }
}

impl Table<Mutant> for Database {
    fn create(&self) {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS mutants (
                id                  INTEGER PRIMARY KEY,
                run_id              INTEGER NOT NULL REFERENCES runs(id) ON DELETE CASCADE,
                diff                TEXT NOT NULL,
                patch_hash          TEXT NOT NULL,
                status              TEXT NOT NULL DEFAULT 'pending'
                CHECK               (status IN ('pending','running','killed','survived', 'timeout','error','skipped','equivalent','unproductive')),
                killed              INTEGER GENERATED ALWAYS AS (CASE WHEN status='killed' THEN 1 ELSE 0 END) VIRTUAL,
                command_to_test     TEXT,
                file_path           TEXT,
                operator            TEXT,
                UNIQUE(run_id, patch_hash)
            )",
            ())
            .unwrap();
    }

    fn add(&self, data: Mutant) -> Mutant {
        self.conn
            .execute(
                "INSERT INTO mutants (
                    run_id,
                    diff,
                    patch_hash,
                    status,
                    killed,
                    command_to_test,
                    file_path,
                    operator,
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    &data.run_id,
                    &data.diff,
                    &data.patch_hash,
                    &data.status.to_string(),
                    &data.killed,
                    &data.command_to_test,
                    &data.file_path,
                    &data.operator,
                ],
            )
            .unwrap();

        data
    }

    fn get(&self, id: usize) -> Mutant {
        self.conn
            .query_one("SELECT * FROM mutants WHERE id = ?1", params![id], |row| {
                Ok(Mutant {
                    run_id: row.get_unwrap(1),
                    diff: row.get_unwrap(2),
                    patch_hash: row.get_unwrap(3),
                    status: row.get_unwrap(4),
                    killed: row.get_unwrap(5),
                    command_to_test: row.get_unwrap(6),
                    file_path: row.get_unwrap(7),
                    operator: row.get_unwrap(8),
                })
            })
            .unwrap()
    }

    fn modify(&self, id: usize, data: Mutant) -> Mutant {
        self.conn
            .execute(
                "UPDATE mutants SET
                    run_id = ?1
                    diff = ?2
                    patch_hash = ?3
                    status = ?4
                    killed = ?5
                    command_to_test = ?6
                    file_path = ?7
                    operator = ?8
                WHERE id = ?9",
                params![
                    &data.run_id,
                    &data.diff,
                    &data.patch_hash,
                    &data.status.to_string(),
                    &data.killed,
                    &data.command_to_test,
                    &data.file_path,
                    &data.operator,
                    id
                ],
            )
            .unwrap();

        data
    }

    fn delete(&self, id: usize) -> bool {
        let rows = self
            .conn
            .execute("DELETE FROM mutants WHERE id = ?1", params![id])
            .unwrap();

        rows > 0
    }
}
