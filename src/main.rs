#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// use rocket::{Rocket, Build};

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build>  {
    rocket::build().mount("/", routes![hello]);
}
