mod database;

use database::init_connection;

use rusqlite::Result;

fn main() -> Result<()> {
    init_connection()?;

    Ok(())
}
