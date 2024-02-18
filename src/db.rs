use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {
    let db_url = "db.sqlite3";
    SqliteConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}
