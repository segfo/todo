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

pub struct PostCreateResult {
    result: std::result::Result<usize, diesel::result::Error>,
}
impl PostCreateResult {
    pub fn get_result(&self) -> &std::result::Result<usize, diesel::result::Error> {
        &self.result
    }
}

impl QueryResult for PostCreateResult {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
pub struct PostReadResult {
    result: std::result::Result<usize, diesel::result::Error>,
}
impl PostReadResult {
    pub fn get_result(&self) -> &std::result::Result<usize, diesel::result::Error> {
        &self.result
    }
}

impl QueryResult for PostReadResult {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct PostAddResult {
    result: std::result::Result<usize, diesel::result::Error>,
}
impl PostAddResult {
    pub fn get_result(&self) -> &std::result::Result<usize, diesel::result::Error> {
        &self.result
    }
}

impl QueryResult for PostAddResult {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Selectable for Post {
    type Columns = (posts::id, posts::title, posts::body, posts::done_flag);
    fn columns() -> Self::Columns {
        (posts::id, posts::title, posts::body, posts::done_flag)
    }
}
use crate::diesel::RunQueryDsl;
impl Dao<Post> for PostDao {
    fn create(&mut self) -> Box<dyn QueryResult> {
        let dto = Post::create_new();
        let res = diesel::insert_into(posts::table)
            .values(&dto)
            .execute(self.get_sqlite_connection().unwrap());
        Box::new(PostCreateResult { result: res })
    }
    fn read(&mut self) -> Box<dyn QueryResult> {
        use super::schema::posts::dsl::*;
        use crate::diesel::QueryDsl;
        let res = posts
            .select(Post::columns())
            .execute(self.get_sqlite_connection().unwrap());
        Box::new(PostReadResult { result: res })
    }
    fn update(&mut self, dto: Post) -> Box<dyn QueryResult> {
        let res = diesel::update(posts::table)
            .set(&dto)
            .execute(self.get_sqlite_connection().unwrap());
        Box::new(PostAddResult { result: res })
    }
    fn delete(&mut self, dto: Post) -> Box<dyn QueryResult> {
        use super::schema::posts::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::QueryDsl;
        let res = diesel::delete(posts.filter(id.eq(dto.get_id())))
            .execute(self.get_sqlite_connection().unwrap());
        Box::new(PostAddResult { result: res })
    }
}
