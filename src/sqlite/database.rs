use super::schema::*;
use crate::db::post::*;
use crate::db::*;
use crate::diesel::{sqlite::SqliteConnection, Connection};

impl Dao<Post> for PostDao {
    fn create(&mut self, dto: Post) -> Box<dyn CreateDaoBuilder> {
        Box::new(PostCreateDaoBuilder::new(dto))
    }
    fn read(&mut self) -> Box<dyn ReadDaoBuilder> {
        Box::new(PostReadDaoBuilder {})
    }
    fn add(&mut self, dto: Post) -> Box<dyn AddDaoBuilder> {
        Box::new(PostAddDaoBuilder::new(dto))
    }
    fn delete(&mut self) -> Box<dyn DeleteDaoBuilder> {
        Box::new(PostDeleteBuilder::new())
    }
}

struct PostCreateDaoBuilder {
    dto: Post,
}
impl PostCreateDaoBuilder {
    fn new(dto: Post) -> Self {
        PostCreateDaoBuilder { dto: dto }
    }
}
impl CreateDaoBuilder for PostCreateDaoBuilder {
    fn run_query(&self) -> Box<dyn QueryResult> {
        unimplemented!()
    }
}
struct PostReadDaoBuilder {}
impl PostReadDaoBuilder {}
impl ReadDaoBuilder for PostReadDaoBuilder {
    fn run_query(self) -> Box<dyn QueryResult> {
        unimplemented!()
    }
}
struct PostAddDaoBuilder {
    dto: Post,
}
impl PostAddDaoBuilder {
    fn new(dto: Post) -> Self {
        PostAddDaoBuilder { dto: dto }
    }
}
impl AddDaoBuilder for PostAddDaoBuilder {
    fn run_query(self) -> Box<dyn QueryResult> {
        unimplemented!()
    }
}

struct PostDeleteBuilder {}
impl PostDeleteBuilder {
    fn new() -> Self {
        PostDeleteBuilder {}
    }
}
impl DeleteDaoBuilder for PostDeleteBuilder {
    fn run_query(self) -> Box<dyn QueryResult> {
        unimplemented!()
    }
}
