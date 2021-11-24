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
    // 個別のResultへダウンキャストを行うためのインタフェース
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Dao<T> {
    fn create(&mut self) -> Box<dyn QueryResult>;
    fn read(&mut self) -> Box<dyn QueryResult>;
    fn update(&mut self, dto: T) -> Box<dyn QueryResult>;
    fn delete(&mut self, dto: T) -> Box<dyn QueryResult>;
}

pub trait Selectable {
    type Columns;
    fn columns() -> Self::Columns;
}
