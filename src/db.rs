pub mod post;
use crate::sqlite::schema;
use crate::sqlite::SqliteConnection;
pub enum DbConnection {
    Sqlite(SqliteConnection),
}

use std::error::Error;

#[derive(Debug)]
struct DaoError {}
impl Error for DaoError {}
impl std::fmt::Display for DaoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

trait Dao<T> {
    fn create(dto: T) -> Result<(), DaoError>;
    fn read() -> Result<T, DaoError>;
    fn add(dto: T) -> Result<(), DaoError>;
    fn delete() -> Result<(), DaoError>;
}
