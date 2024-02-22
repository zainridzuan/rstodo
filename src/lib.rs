use chrono::{DateTime, Local, SubsecRound, Utc};
use rusqlite::{params, Connection, Result};
use std::io::{self, Write};
use uuid::Uuid;

#[derive(Debug)]
struct Task {
    id: i32,
    task_name: String,
    task_info: Option<String>,
    created_on: String,
    is_done: bool,
}

impl Task {
    // Task constructor
    pub fn new(
        id: i32,
        task_name: String,
        task_info: Option<String>,
        created_on: String,
        is_done: bool,
    ) -> Self {
        Task {
            id,
            task_name,
            task_info,
            created_on,
            is_done,
        }
    }

    // Add a new Task to the todo database
    pub fn add(conn: &Connection) -> Result<()> {
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

        conn.execute(
            "INSERT INTO tasks (task_name, task_info) VALUES (?1, ?2)",
            params![task_name, task_info],
        )?;

        Ok(())
    }

    // Returns a vec of all Tasks in database
    pub fn list(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare("SELECT * FROM todo ORDER by is_done, id")?;
        let task_iter = stmt.query_map((), |row| {
            Ok(Task::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    // Prints list of Tasks
    pub fn print_list(tasks: Vec<Task>) -> Result<()> {
        for task in tasks {
            println!(
                "{:>4} | {:<44} {:<8} {}",
                task.id, &task.task_name, task.is_done, task.created_on,
            );
        }
        Ok(())
    }

    // Toggle 'is_done' field for a Task
    pub fn toggle(conn: &Connection, id: i32) -> Result<()> {
        conn.execute("UPDATE todo SET is_done = 1 - is_done WHERE id = ?", &[&id])?;
        Ok(())
    }
}

pub fn init_connection() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;

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
            id          INTEGER PRIMARY KEY,
            task_name   TEXT NOT NULL,
            task_info   BLOB,
            created_on  REAL NOT NULL DEFAULT current_timestamp,
            completed   INTEGER NOT NULL DEFAULT 0
        )",
        (),
    )?;
    Ok(conn)
}

// util function to reset database for testing
pub fn reset_db() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;
    conn.execute("DROP TABLE IF EXISTS tasks", ())?;
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
