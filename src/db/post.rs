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
    fn create(dto: Post) -> Box<dyn CreateDaoBuilder<Item = Post>> {
        Box::new(PostCreateDaoBuilder::new(dto))
    }
    fn read() -> Box<dyn ReadDaoBuilder<Item = Post>> {
        Box::new(PostReadDaoBuilder {})
    }
    fn add(dto: Post) -> Box<dyn AddDaoBuilder<Item = Post>> {
        Box::new(PostAddDaoBuilder::new(dto))
    }
    fn delete() -> Box<dyn DeleteDaoBuilder> {
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
    type Item = Post;
    fn run_query(self) -> Result<Self::Item, DaoError> {
        unimplemented!()
    }
}
struct PostReadDaoBuilder {}
impl PostReadDaoBuilder {}
impl ReadDaoBuilder for PostReadDaoBuilder {
    type Item = Post;
    fn run_query(self) -> Result<Self::Item, DaoError> {
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
    type Item = Post;
    fn run_query(self) -> Result<(), DaoError> {
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
    fn run_query(self) -> Result<(), DaoError> {
        unimplemented!()
    }
}
