use std::{io::{self, Write}, str::FromStr};

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }

    fn display(&self) {
        let status = if self.completed { "Yes" } else { "No" };
        println!("{:<5}{:<10}{:<50}", self.id, status, self.description);
    }
}

struct TodoApp {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self) {
        let description: String = get_input("\nEnter task description: ");
        let task = Task::new(self.next_id, description);
        
        self.tasks.push(task);
        
        println!("\nTask added succesfully.");
        
        self.next_id += 1;
    }

    fn view_tasks(&self) {
        if self.tasks.is_empty() {
            println!("\nNo tasks yet.");
        } else {
            println!("\n{:<5}{:<10}{:<50}", "ID", "STATUS", "DESCRIPTION");
            for task in &self.tasks {
                task.display();
            }
        }
    }

    fn update_task(&mut self) {
        let id: u32 = get_input("\nEnter task ID (complete/incomplete): ");
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed;
            println!("\nTask {} updated", id);
        } else {
            println!("\nTask not found.")
        }
    }

    fn delete_task(&mut self) {
        let id: u32 = get_input("\nEnter task id to delete: ");
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            println!("\nTask {} deleted", id);
        } else {
            println!("\nTask not found.");
        }
    }
}

fn main() {
    let mut app = TodoApp::new();

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
            },
            _ => {
                println!("\nInvalid choice. Please try again.");
            }
        }
    }
}

fn get_input<T: FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to get input");

        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}
