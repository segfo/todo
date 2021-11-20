pub mod database;
pub mod schema;
// sqlite独自のDaoオブジェクトとかを書く
pub use crate::diesel::{sqlite::SqliteConnection, Connection};
