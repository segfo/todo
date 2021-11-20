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
    fn create(dto: T) -> Box<dyn CreateDaoBuilder<Item = T>>;
    fn read() -> Box<dyn ReadDaoBuilder<Item = T>>;
    fn add(dto: T) -> Box<dyn AddDaoBuilder<Item = T>>;
    fn delete() -> Box<dyn DeleteDaoBuilder>;
}
trait CreateDaoBuilder {
    type Item;
    fn run_query(self) -> Result<Self::Item, DaoError>;
}
trait ReadDaoBuilder {
    type Item;
    fn run_query(self) -> Result<Self::Item, DaoError>;
}
trait AddDaoBuilder {
    type Item;
    fn run_query(self) -> Result<(), DaoError>;
}
trait DeleteDaoBuilder {
    fn run_query(self) -> Result<(), DaoError>;
}
