#![feature(proc_macro_hygiene, decl_macro)] // enable security macros

use std::fmt::format;

#[macro_use] // enable marco from rocket
extern crate rocket;

// methods like controllers
#[get("/")]
fn index()->  &'static str {
    "Hello API Rocket!"
}

#[get("/hello/<name>")] 
fn hello(name:String)-> String {
    format!("Hello, {}", name)
}

#[get("/number/<number>")] 
fn number(number: i32) -> String {
    format!("this numer is {}", number)
}

#[get("/search?<query>&<typ>")] 
fn search(query: String, typ: Option<String>) -> String {
   match typ{
    Some(t) => format!("Searching for '{}' (type:{})", query, t),
    None => format!("Searching for '{}'(no type specified)", query)
   }
}
fn main() {
   rocket::ignite().mount("/", routes![index, hello, number, search]).launch();
}
