#![feature(lazy_cell)]

#[macro_use] extern crate rocket;

use std::sync::LazyLock;

use rocket::form::{Form, FromForm};
use rocket::fs::FileServer;
use std::fs;
use std::env;

use sha256::digest;

static SECRET_URL: LazyLock<String> = LazyLock::new(|| {
        fs::read_to_string("secret_url.txt")
            .expect("Cannot read the file.") 
});

static PASSWORD_HASH: LazyLock<String> = LazyLock::new(|| {
        let password = env::var("SECRET_PASSWORD")
            .expect("SECRET_PASSWORD is not defined");
        digest(password)
});

#[derive(Debug, FromForm)]
struct Submit<'v> {
    password: &'v str,
}

#[post("/password", data = "<form>")]
fn submit<'r>(form: Form<Submit<'r>>) -> String {
    if PASSWORD_HASH.eq(&digest(form.password)) {
        SECRET_URL.to_string()
    } else {
        String::from("Invalid password!")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![submit])
        .mount("/", FileServer::from("static"))
}
