use super::schema::*;
use crate::db::post::*;
use crate::db::*;
use crate::diesel::{sqlite::SqliteConnection, Connection};

pub struct PostDao {
    connection: DbConnection,
}
impl PostDao {
    fn new(connection: DbConnection) -> Self {
        PostDao {
            connection: connection,
        }
    }
    fn get_sqlite_connection(&self) -> Option<&SqliteConnection> {
        match &self.connection {
            DbConnection::Sqlite(conn) => Some(&conn),
            _ => None,
        }
    }
}
#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

struct PostCreateResult {
    result: std::result::Result<usize, diesel::result::Error>,
}

impl QueryResult for PostCreateResult {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Dao<Post> for PostDao {
    fn create(&mut self, dto: Post) -> Box<dyn QueryResult> {
        use crate::diesel::RunQueryDsl;
        let res = diesel::insert_into(posts::table)
            .values(&dto)
            .execute(self.get_sqlite_connection().unwrap());
        Box::new(PostCreateResult { result: res })
    }
    fn read(&mut self) -> Box<dyn QueryResult> {
        unimplemented!()
    }
    fn add(&mut self, dto: Post) -> Box<dyn QueryResult> {
        unimplemented!()
    }
    fn delete(&mut self) -> Box<dyn QueryResult> {
        unimplemented!()
    }
}
