#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

pub mod control;
pub mod system;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, cpu_usage])
        .launch();
}
