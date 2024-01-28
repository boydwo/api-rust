use rocket::{get, post, put, delete};
use rocket::State;
use rocket_contrib::json::Json;


use crate::db::connection::{DbPool};
use crate::models::user::{User, NewUser};



#[post("/users", format = "application/json", data = "<new_user>")]
pub fn create(new_user: Json<NewUser>, pool: State<DbPool>) -> Json<User> {
    let conn = pool.get().expect("Can't get DB connection from pool");
    let new_user = NewUser {
      name: new_user.name,
      email: new_user.email,
  };

  let user_created = User::create(new_user, &conn).expect("Failed to create user");

  Json(user_created)
}

// #[get("/users")]
// fn read_all(pool: &State<DbPool>) -> Json<Vec<User>> {
//     let conn = pool.get().expect("Can't get DB connection from pool");
//     // Implemente a lógica para ler todos os usuários
// }

// #[get("/users/<id>")]
// fn read(id: i32, pool: &State<DbPool>) -> Json<User> {
//     let conn = pool.get().expect("Can't get DB connection from pool");
//     // Implemente a lógica para ler um usuário específico pelo ID
// }

// #[put("/users/<id>", format = "application/json", data = "<user>")]
// fn update(id: i32, user: Json<UpdateUser>, pool: &State<DbPool>) -> Json<User> {
//     let conn = pool.get().expect("Can't get DB connection from pool");
//     // Implemente a lógica para atualizar um usuário
// }

// #[delete("/users/<id>")]
// fn delete(id: i32, pool: &State<DbPool>) -> Json<bool> {
//     let conn = pool.get().expect("Can't get DB connection from pool");
//     // Implemente a lógica para deletar um usuário
// }