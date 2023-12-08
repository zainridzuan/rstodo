use rusqlite::{Connection, Result};

pub fn init_connection() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS lists (
            id INTEGER PRIMARY KEY,
            list_name TEXT NOT NULL,
            created_on TEXT NOT NULL 
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            task_name TEXT NOT NULL,
            task_info TEXT NOT NULL,
            created_on TEXT NOT NULL,
            due_by TEXT,
            completed INTEGEr,
            FOREIGN KEY(list_id) REFERENCES lists(id)
        )",
        [],
    )?;
    Ok(conn)
}

// pub fn
