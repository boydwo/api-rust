#![feature(proc_macro_hygiene, decl_macro)] // enable security macros

#[macro_use] // enable marco from rocket
extern crate rocket;

#[get("/")]
fn index()->  &'static str {
    "Hello API Rocket!"
}
fn main() {
   rocket::ignite().mount("/", routes![index]).launch();
}
