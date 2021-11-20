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
    fn create(dto: Post) -> Result<(), DaoError> {
        unimplemented!();
    }
    fn read() -> Result<Post, DaoError> {
        unimplemented!();
    }
    fn add(dto: Post) -> Result<(), DaoError> {
        unimplemented!();
    }
    fn delete() -> Result<(), DaoError> {
        unimplemented!();
    }
}
