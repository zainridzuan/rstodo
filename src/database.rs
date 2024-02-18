use chrono::{DateTime, Local, SubsecRound, Utc};
use rusqlite::{params, Connection, Result};
use std::io::{self, Write};
use uuid::Uuid;

#[derive(Debug)]
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
        task_name: task_name.trim().to_string(),
        task_info,
        created_on: Local::now().round_subsecs(0).naive_local().to_string(),
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

pub fn show_tasks() -> Result<()> {
    let conn = Connection::open("todo.db")?;

    let mut stmt = conn.prepare("SELECT * from tasks")?;

    let tasks = stmt.query_map((), |row| {
        Ok(Task {
            id: row.get(0)?,
            task_name: row.get(1)?,
            task_info: row.get(2)?,
            created_on: row.get(3)?,
            completed: row.get(4)?,
        })
    })?;

    println!("#####################################");
    for task in tasks {
        let task = task.unwrap();
        print!("id: {}\n", task.id);
        print!("task name: {}\n", task.task_name);
        match task.task_info {
            Some(x) => print!("task info: {}", x),
            None => print!("task info: "),
        }
        print!("created on: {}\n", task.created_on);
        print!("completed: {}\n", task.completed);
        println!("#####################################")
    }

    Ok(())
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
