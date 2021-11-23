pub mod error;
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

pub trait QueryResult {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Dao<T> {
    fn create(&mut self, dto: T) -> Box<dyn CreateDaoBuilder>;
    fn read(&mut self) -> Box<dyn ReadDaoBuilder>;
    fn add(&mut self, dto: T) -> Box<dyn AddDaoBuilder>;
    fn delete(&mut self) -> Box<dyn DeleteDaoBuilder>;
}

pub trait CreateDaoBuilder {
    fn run_query(&self) -> Box<dyn QueryResult>;
}
pub trait ReadDaoBuilder {
    fn run_query(self) -> Box<dyn QueryResult>;
}
pub trait AddDaoBuilder {
    fn run_query(self) -> Box<dyn QueryResult>;
}
pub trait DeleteDaoBuilder {
    fn run_query(self) -> Box<dyn QueryResult>;
}
