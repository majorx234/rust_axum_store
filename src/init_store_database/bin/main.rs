use rusqlite::Connection;

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "db.sqlite";
    let db = Connection::open(db_path)?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS messages (text TEXT NOT NULL)",
        [],
    )?;
    db.execute(
        "INSERT OR IGNORE INTO messages (text) VALUES ('hello_world')",
        [],
    )?;
    Ok(())
}
