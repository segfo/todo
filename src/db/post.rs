use super::schema::*;
use super::*;
// #[derive(Queryable)]
#[derive(Queryable, Insertable)]
#[table_name = "posts"]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    done_flag: bool,
}
impl Post {
    pub fn test_new() -> Self {
        Post {
            id: 0,
            title: "".to_owned(),
            body: "".to_owned(),
            done_flag: false,
        }
    }
}
