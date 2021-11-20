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
    fn create(dto: Post) -> Box<dyn CreateDaoBuilder> {
        unimplemented!();
    }
    fn read() -> Box<dyn ReadDaoBuilder> {
        unimplemented!();
    }
    fn add(dto: Post) -> Box<dyn AddDaoBuilder> {
        unimplemented!();
    }
    fn delete() -> Box<dyn DeleteDaoBuilder> {
        unimplemented!();
    }
}

struct PostCreateDaoBuilder {}
impl CreateDaoBuilder for PostCreateDaoBuilder {}

// impl Dao<Post> for PostDao {
//     fn create(dto: Post) -> Result<(), DaoError> {
//         unimplemented!();
//     }
//     fn read() -> Result<Post, DaoError> {
//         unimplemented!();
//     }
//     fn add(dto: Post) -> Result<(), DaoError> {
//         unimplemented!();
//     }
//     fn delete() -> Result<(), DaoError> {
//         unimplemented!();
//     }
// }
