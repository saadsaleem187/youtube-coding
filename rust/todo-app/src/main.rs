use std::{
    io::{self, Write},
    process::Command,
    str::FromStr,
};

use rusqlite::{params, Connection, Result};

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    fn display(&self) {
        let status = if self.completed { "Yes" } else { "No" };
        println!("{:<5}{:<10}{:<50}", self.id, status, self.description);
    }
}

struct TodoApp {
    conn: Connection,
}

impl TodoApp {
    fn new() -> Result<Self> {
        let conn = Connection::open("db.sqlite3")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                completed INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    fn add_task(&self) {
        let description: String = get_input("\nEnter task description: ");

        self.conn
            .execute(
                "INSERT INTO tasks (description, completed) VALUES (?1, 0)",
                params![description],
            )
            .unwrap();

        println!("\nTask added succesfully.");
    }

    fn view_tasks(&self) {
        let mut stmt = self
            .conn
            .prepare("SELECT id, description, completed FROM tasks ORDER BY id")
            .unwrap();

        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    description: row.get(1)?,
                    completed: row.get::<_, u32>(2)? == 1,
                })
            })
            .unwrap();

        let mut tasks_found: bool = false;

        for task in tasks {
            if !tasks_found {
                println!("\n{:<5}{:<10}{:<50}", "ID", "STATUS", "DESCRIPTION");
                tasks_found = true;
            }

            task.unwrap().display();
        }

        if !tasks_found {
            println!("\nNO TASKS YET.");
        }
    }

    fn update_task(&self) {
        let id: u32 = get_input("\nEnter task ID (complete/incomplete): ");

        let rows_affected = self
            .conn
            .execute(
                "UPDATE tasks SET completed = NOT completed WHERE id = ?1",
                params![id],
            )
            .unwrap();

        if rows_affected == 0 {
            println!("\nTask not found.")
        } else {
            println!("\nTask {} updated.", id);
        }
    }

    fn delete_task(&self) {
        let id: u32 = get_input("\nEnter task id to delete: ");

        let rows_affected = self
            .conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![id])
            .unwrap();

        if rows_affected == 0 {
            println!("\nTask not found.")
        } else {
            println!("\nTask {} deleted.", id);
        }
    }
}

fn main() -> Result<()> {
    let app = TodoApp::new()?;

    clear_screen();

    loop {
        println!("\n============ Todo-App ============");
        println!("1 - Add Task");
        println!("2 - View Tasks");
        println!("3 - Update Task");
        println!("4 - Delete Task");
        println!("5 - Exit");

        let choice: u32 = get_input("\nEnter your choice: ");

        match choice {
            1 => app.add_task(),
            2 => app.view_tasks(),
            3 => app.update_task(),
            4 => app.delete_task(),
            5 => {
                println!("\nGoodbye!\n");
                break;
            }
            _ => {
                println!("\nInvalid choice. Please try again.");
            }
        }
    }

    Ok(())
}

fn get_input<T: FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
