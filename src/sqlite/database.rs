use crate::diesel::{sqlite::SqliteConnection, Connection};

fn establish_connection() -> SqliteConnection {
    let database_url = "sample.db";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
