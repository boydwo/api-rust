use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use crate::schema::users;

#[derive(Insertable,Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub name: &'a str,
  pub email: &'a str,
}


impl User {
    pub fn create(new_user: NewUser, conn: &PgConnection) -> QueryResult<User> {
      diesel::insert_into(users::table)
      .values(&new_user)
      .get_result(conn)
}
    

    pub fn read(conn: &PgConnection) -> QueryResult<Vec<User>> {
        users::table.load::<User>(conn)
    }


}