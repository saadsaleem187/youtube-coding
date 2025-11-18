use rusqlite::{params, Connection, Result};

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: u32,
    name: String,
    age: u32,
}

fn main() -> Result<()> {
    let conn = connect_db()?;

    create_table(&conn)?;

    insert_user(&conn, "DHH", 50)?;
    insert_user(&conn, "Tsoding", 60)?;

    list_users(&conn)?;

    update_user_age(&conn, 1, 40)?;

    list_users(&conn)?;
    
    delete_user(&conn, 1)?;
    
    list_users(&conn)?;
    
    Ok(())
}

// Connect to SQLite database
fn connect_db() -> Result<Connection> {
    let conn = Connection::open("db.sqlite3")?;

    print!("\nCONNECTED TO DATABASE\n");

    Ok(conn)
}

// Create table
fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [],
    )?;

    println!("Created users table or already exists.\n");

    Ok(())
}

// Insert user
fn insert_user(conn: &Connection, name: &str, age: u32) -> Result<()> {
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params![name, age],
    )?;

    println!("User '{}' inserted.", name);

    Ok(())
}

// Update user age
fn update_user_age(conn: &Connection, id: u32, age: u32) -> Result<()> {
    conn.execute(
        "UPDATE users SET age = ?1 WHERE id = ?2",
        params![age, id],
    )?;

    println!("\nUser '{}' age updated\n", id);

    Ok(())
}

// Delete user
fn delete_user(conn: &Connection, id: u32) -> Result<()> {
    conn.execute(
        "DELETE FROM users WHERE id = ?1",
        params![id],
    )?;

    println!("\nDeleted user '{}'.\n", id);

    Ok(())
}

// List user
fn list_users(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM users")?;
    let users_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    let mut found_users: bool = false;

    for user in users_iter {
        println!("{:?}", user?);
        found_users = true;
    }

    if !found_users {
        println!("NO USERS FOUND IN THE DATABASE.\n");
    }

    Ok(())
}
