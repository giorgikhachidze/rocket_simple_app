#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate rand;
extern crate serde;
extern crate time;
extern crate rocket_simple_app;

use time::Duration;
use diesel::prelude::*;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;


use rocket_simple_app::models::UsersSessions;
use rocket_simple_app::{create_new_user_session, estabilish_connection};

#[derive(Serialize)]
struct IndexContext { 
    header: String
}

#[derive(FromForm)]
struct User { 
    email: String,
    password: String
}


#[get("/")]
fn index(cookies: Cookies) -> Template {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if user_id == 1 {
                let context = IndexContext {
                    header: "You're logged in!".to_string(),
                };
                Template::render("index", &context)
            } else {
                let context = IndexContext {
                    header: "Sign in".to_string(),
                };
                Template::render("index", &context)
            }
        }
        Err(_not_logged_in) => {
            let context = IndexContext {
                header: "Sign in".to_string(),
            };
            Template::render("index", &context)
        }
    }
}

#[post("/auth", data = "<input>")]
fn login(input: Form<User>, mut cookies: Cookies) -> Result<Redirect, String> {
    if input.email == "admin@admin.ge".to_string() && input.password == "admin" {
        match generate_session_token(64) {
            Ok(session_token) => {
                let connection = estabilish_connection();
                let user_id = 1;

                create_new_user_session(&connection, user_id, session_token.clone());

                let mut session_token = Cookie::new("session_token", session_token);
                
                session_token.set_max_age(Duration::days(1));
                cookies.add_private(session_token);

                Ok(Redirect::to("/"))
            }
            Err(_) => Err(String::from("Login failed"))
        }
    } else {
        Err(String::from("Username or password incorrect"))
    }
}

fn get_user_id_from_cookies(mut cookies: Cookies) -> Result<u64, std::io::Error> {
    match cookies.get_private("session_token") {
        Some(cookie) => match get_user_id_from_session_cookies(cookie.value().to_string()) {
            Ok(user_id) => Ok(user_id),
            Err(error) => Err(error)
        },
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "token not found",
            ))
        }
    }
}

fn get_user_id_from_session_cookies(session_token: String) -> Result<u64, std::io::Error> {
    use rocket_simple_app::schema::users_sessions::dsl::*;

    let connection = estabilish_connection();
    let results = users_sessions
        .filter(token.eq(session_token))
        .limit(1)
        .load::<UsersSessions>(&connection)
        .expect("Error loading sessions!");

    if results.len() == 1 {
        return Ok(results[0].user_id);
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "token not found",
        ))
    }
}

fn generate_session_token(length: u8) -> Result<String, std::io::Error> {
    let bytes: Vec<u8> = (0..length).map(|_| rand::random::<u8>()).collect();
    let strings: Vec<String> = bytes.iter().map(|byte| format!("{:02X}", byte)).collect();
    
    return Ok(strings.join(""));
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, login])
        .attach(Template::fairing())
        .launch();
}