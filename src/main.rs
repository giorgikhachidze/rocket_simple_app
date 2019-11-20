#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

extern crate regex;
extern crate rand;
extern crate time;
extern crate serde;
extern crate bcrypt;
extern crate rocket_simple_app;

use regex::Regex;
use time::Duration;
use diesel::prelude::*;
use rocket::request::Form;
use rocket::response::Redirect;
use bcrypt::{hash, DEFAULT_COST};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;

use rocket_simple_app::models::UsersSessions;
use rocket_simple_app::{create_new_user, create_new_user_session, estabilish_connection};

lazy_static! {
    static ref EMAIL_REGEX: regex::Regex = Regex::new("^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
}

#[derive(Serialize)]
struct Context { 
    header: String,
}

#[derive(FromForm)]
struct UserLogin { 
    email: String,
    password: String
}

#[derive(FromForm)]
struct UserRegister {
    name: String,
    email: String,
    password: String,
    confirm_password: String
}

#[get("/")]
fn index() -> Template {
    let context = Context {
        header: "Main page".to_string()
    };
    Template::render("index", &context)
}


#[get("/login")]
fn login(cookies: Cookies) -> Template {
    match get_user_id_from_cookies(cookies) {
        Ok(user_id) => {
            if user_id == 1 {
                let context = Context {
                    header: "You're logged in!".to_string()
                };
                Template::render("login", &context)
            } else {
                let context = Context {
                    header: "Login".to_string()
                };
                Template::render("login", &context)
            }
        }
        Err(_not_logged_in) => {
            let context = Context {
                header: "Login".to_string()
            };
            Template::render("auth/login", &context)
        }
    }
}

#[post("/login", data = "<input>")]
fn authorization(input: Form<UserLogin>, mut cookies: Cookies) -> Result<Redirect, String> {
    if input.email == "admin@admin.ge".to_string() && input.password == "admin" {
        match generate_session_token(64) {
            Ok(session_token) => {
                let connection = estabilish_connection();
                let user_id = 1;

                create_new_user_session(&connection, user_id, session_token.clone());

                let mut session_token = Cookie::new("session_token", session_token);
                
                session_token.set_max_age(Duration::days(1));
                cookies.add_private(session_token);

                Ok(Redirect::to("/login"))
            }
            Err(_) => Err(String::from("Login failed"))
        }
    } else {
        Err(String::from("Username or password incorrect"))
    }
}

#[get("/register")]
fn register() -> Template {
    let context = Context {
        header: "Registration page".to_string()
    };
    Template::render("auth/register", &context)
}

#[post("/register", data = "<input>")]
fn registration(input: Form<UserRegister>) -> String {
    let input: UserRegister = input.into_inner();

    if input.name.is_empty() {
        return String::from("The name field required.");
    } else if !input.name.chars().all(char::is_alphanumeric) {
        return String::from("The name format is invalid.");
    } else if input.email.is_empty() {
        return String::from("The email field required.");
    } else if !EMAIL_REGEX.is_match(&input.email) {
        return String::from("The email format is invalid.");
    } else if input.password.is_empty() {
        return String::from("The password field required.");
    } else if input.confirm_password.is_empty() {
        return String::from("The confirm password field required.");
    } else if input.password != input.confirm_password {
        return String::from("The passwords not match.");
    } else {
        match hash(&input.password, DEFAULT_COST) {
            Ok(hashed_password) => {
                let connection = estabilish_connection();
                create_new_user(&connection, input.name, input.email, hashed_password);
                return String::from("User registered");
            }
            Err(_) => return String::from("registration failed!"),
        }
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
        .mount("/", routes![index, login, authorization, register, registration])
        .attach(Template::fairing())
        .launch();
}