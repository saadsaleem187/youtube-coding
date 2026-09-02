use rusqlite::{Connection, Result, params};

use crate::models::Todo;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let connection = Connection::open("todos.db")?;

        connection.execute(
            "
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            )
            ",
            [],
        )?;

        Ok(Self { connection })
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let mut statement = self.connection.prepare(
            "
            SELECT id, title, completed
            FROM todos
            ORDER BY id DESC
            ",
        )?;

        let todos = statement
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get::<_, i32>(2)? != 0,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(todos)
    }

    pub fn add_todos(&self, title: &str) -> Result<()> {
        self.connection.execute(
            "
            INSERT INTO todos (title, completed) VALUES (?1, 0)
            ",
            params![title],
        )?;

        Ok(())
    }

    pub fn delete_todo(&self, id: i64) -> Result<()> {
        self.connection.execute(
            "
            DELETE FROM todos WHERE id = ?1
            ",
            params![id],
        )?;

        Ok(())
    }

    pub fn clear_completed(&self) -> Result<()> {
        self.connection
            .execute("DELETE FROM todos WHERE completed = 1", [])?;

        Ok(())
    }

    pub fn toggle_todo(&self, id: i64, completed: bool) -> Result<()> {
        self.connection.execute(
            "
            UPDATE todos
            SET completed = ?1
            WHERE id = ?2
            ",
            params![completed as i32, id],
        )?;

        Ok(())
    }
}
