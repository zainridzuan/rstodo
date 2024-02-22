mod database;

use database::{add_task, init_connection};
use database::{reset_db, show_tasks};

use rusqlite::Connection;
use rusqlite::Result;

fn main() -> Result<()> {
    // reset_db()?;
    // init_connection()?;
    // add_task()?;
    Ok(())
}
