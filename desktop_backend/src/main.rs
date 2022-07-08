pub mod imap;
use crate::imap::fetch_inbox_top;

#[macro_use] extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    let value = fetch_inbox_top().unwrap();
    print!("{:?}", value);
    rocket::build().mount("/", routes![hello])
}


