pub mod imap;
use crate::imap::fetch_inbox_top;

pub mod authentication;
use crate::authentication::*;

pub mod front_api_back;
use crate::front_api_back::*;

pub mod database;
use crate::database::*;

#[macro_use] extern crate rocket;

#[get("/home")]
fn home() -> String {
    format!("Welcome to the main Page!")
}

#[get("/getmail")]
fn getmail() -> String {
    let value = fetch_inbox_top().unwrap();
    format!("{:?}", value)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        home,
        getmail
    ])
}


