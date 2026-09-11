use rusqlite::{Connection, Result, params};

use crate::models::Note;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let connection = Connection::open("notes.db")?;

        connection.execute(
            "
            CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                pinned INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            )
            ",
            [],
        )?;

        Ok(Self { connection })
    }

    pub fn get_notes(&self) -> Result<Vec<Note>> {
        let mut statement = self.connection.prepare(
            "
                SELECT * FROM notes
                ORDER BY pinned DESC, updated_at DESC 
            ",
        )?;

        let notes = statement
            .query_map([], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    pinned: row.get::<_, i32>(3)? != 0,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;

        Ok(notes)
    }

    pub fn create_note(&self, title: &str, content: &str) -> Result<i64> {
        self.connection.execute(
            "
            INSERT INTO notes (title, content) VALUES (?1, ?2)
            ",
            params![title, content],
        )?;

        Ok(self.connection.last_insert_rowid())
    }

    pub fn update_note(&self, id: i64, title: &str, content: &str) -> Result<()> {
        self.connection.execute(
            "
            UPDATE notes
            SET
                title = ?1,
                content = ?2,
                updated_at = CURRENT_TIMESTAMP,
            WHERE
                id = ?3
            ",
            params![id, title, content],
        )?;

        Ok(())
    }

    pub fn delete_note(&self, id: i64) -> Result<()> {
        self.connection.execute(
            "
            DELETE FROM notes WHERE id = ?1
            ",
            params![id],
        )?;

        Ok(())
    }

    pub fn toggle_pin(&self, id: i64, pinned: bool) -> Result<()> {
        self.connection.execute(
            "
            UPDATE notes
            SET
                pinned = ?1,
                updated_at = CURRENT_TIMESTAMP
            WHERE
                id = ?2
            ",
            params![pinned as i32, id],
        )?;

        Ok(())
    }
}
