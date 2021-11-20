use super::schema::*;
use super::*;
#[derive(Queryable, Insertable)]
#[table_name = "posts"]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    done_flag: bool,
}

struct PostDao;
impl Dao<Post> for PostDao {
    fn create(dto: Self) -> Box<dyn CreateDaoBuilder<Item = Post>> {
        unimplemented!();
    }
    fn read() -> Box<dyn ReadDaoBuilder<Item = Post>> {
        unimplemented!();
    }
    fn add(dto: Self) -> Box<dyn AddDaoBuilder<Item = Post>> {
        unimplemented!();
    }
    fn delete() -> Box<dyn DeleteDaoBuilder> {
        unimplemented!();
    }
}

struct PostCreateDaoBuilder {}
impl CreateDaoBuilder for PostCreateDaoBuilder {
    type Item = Post;
    fn run_query(self) -> Result<Self::Item, DaoError> {
        unimplemented!()
    }
}
struct PostReadDaoBuilder {}
impl ReadDaoBuilder for PostReadDaoBuilder {
    type Item = Post;
    fn run_query(self) -> Result<Self::Item, DaoError> {
        unimplemented!()
    }
}
struct PostAddDaoBuilder {}
impl AddDaoBuilder for PostAddDaoBuilder {
    type Item = Post;
    fn add(&self, item: Self::Item) -> &Self {
        self
    }
    fn run_query(self) -> Result<Self::Item, DaoError> {
        unimplemented!()
    }
}
struct PostDeleteBuilder {}
impl DeleteDaoBuilder for PostDeleteBuilder {
    fn run_query(self) -> Result<(), DaoError> {
        unimplemented!()
    }
}
