use super::schema::*;
use super::*;
#[derive(Queryable, Insertable, Identifiable, AsChangeset)]
#[table_name = "posts"]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    done_flag: bool,
}
impl Post {
    pub fn create_new() -> Self {
        Post {
            id: 0,
            title: "".to_owned(),
            body: "".to_owned(),
            done_flag: false,
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
