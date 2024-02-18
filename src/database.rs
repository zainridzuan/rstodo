use chrono::{DateTime, Local, Utc};
use rusqlite::{params, Connection, Result};
use std::io::{self, Write};
use uuid::Uuid;

struct Task {
    id: String,
    task_name: String,
    task_info: Option<String>,
    created_on: String,
    completed: bool,
}

pub fn init_connection() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "DROP TABLE IF EXISTS person",
        (), // empty list of parameters.
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS lists (
            id         STRING PRIMARY KEY,
            list_name  TEXT NOT NULL,
            created_on TEXT NOT NULL
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id STRING PRIMARY KEY,
            task_name TEXT NOT NULL,
            task_info BLOB,
            created_on TEXT NOT NULL,
            completed INTEGER
        )",
        (),
    )?;
    Ok(conn)
}

pub fn add_task() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;

    let mut task_name = String::new();
    loop {
        print!("Enter the task name: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut task_name).unwrap();
        match task_name.trim() {
            "" => println!("Task name cannot be empty!"),
            _ => break (),
        }
    }

    let mut desc_str = String::new();
    print!("Enter a brief description of the task (optional): ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut desc_str).unwrap();
    let task_info = match desc_str.is_empty() {
        true => None,
        false => Some(desc_str),
    };

    // let mut lists_name = String::new();

    let task = Task {
        id: Uuid::new_v4().to_string(),
        task_name,
        task_info,
        created_on: Local::now().naive_local().to_string(),
        completed: false,
    };

    conn.execute(
        "INSERT INTO tasks (id, task_name, task_info, created_on, completed) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            &task.id,
            &task.task_name,
            &task.task_info,
            &task.created_on,
            &task.completed,
        ],
    )?;

    Ok(conn)
}
