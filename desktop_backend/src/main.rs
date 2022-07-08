pub mod imap;
use crate::imap::fetch_inbox_top;

pub mod authentication;
use crate::authentication::*;

pub mod front_api_back;
use crate::front_api_back::*;

pub mod database;
use crate::database::*;

#[macro_use] extern crate rocket;

#[get("/hello/<name>/<password>")]
fn hello(name: &str, password: &str) -> String {
    let value = fetch_inbox_top().unwrap();
    print!("{:?}", value);
    format!("Hello, {} year old named {}!", password, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}


