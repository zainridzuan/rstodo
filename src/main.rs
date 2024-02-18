mod database;

use database::{add_task, init_connection};

use rusqlite::Connection;
use rusqlite::Result;

fn main() -> Result<()> {
    init_connection()?;
    add_task()?;
    Ok(())
}
