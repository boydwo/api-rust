#![feature(proc_macro_hygiene, decl_macro)] // enable security macros

// enable marco from rocket
#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;

// import modules
mod schema;
mod db;
mod routes;
mod models;

use dotenv::dotenv;
use routes::user_routes::*;

#[get("/")]
fn index()->  &'static str {
    "Hello API Rocket!"
}


fn main() {
    dotenv().ok();

   rocket::ignite().mount("/", routes![index,create])
   .manage(db::connection::establish_connection()).launch();

}
