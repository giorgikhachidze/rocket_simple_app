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
use rocket::Request;
use serde_json::json;
use diesel::prelude::*;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::serve::StaticFiles;
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket_contrib::templates::Template;

use rocket_simple_app::models::{Users, UsersSessions};
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
fn index(cookies: Cookies) -> Template {
    match get_user_id_from_cookies(cookies) {
        Ok(_user_logged_in) => {
            let context = Context {
                header: "You'r logged in!".to_string()
            };
            Template::render("index", &context)
        },
        Err(_not_logged_in) => {
            let context = Context {
                header: "Main page".to_string()
            };
            Template::render("index", &context)
        }
    }
}


#[get("/login")]
fn login(cookies: Cookies) -> Result<Template, Redirect> {
    match get_user_id_from_cookies(cookies) {
        Ok(_user_logged_in) => Err(Redirect::to("/")),
        Err(_not_logged_in) => {
            let context = Context {
                header: "Login".to_string()
            };
            Ok(Template::render("auth/login", &context))
        }
    }
}

#[post("/login", data = "<input>")]
fn authorization(input: Form<UserLogin>, mut cookies: Cookies) -> Result<Redirect, String> {
    let input: UserLogin = input.into_inner();

    match get_password_hash_from_email(input.email.clone()) {
        Ok(password_hash) => match verify(&input.password, &password_hash) {
            Ok(password_match) => {
                if password_match {
                    match generate_session_token(64) {
                        Ok(session_token) => {
                            let connection = estabilish_connection();

                            create_new_user_session(&connection, get_user_id_from_email(input.email), session_token.clone());
            
                            let mut session_token = Cookie::new("session_token", session_token);
                            
                            session_token.set_max_age(Duration::days(1));
                            cookies.add_private(session_token);
            
                            Ok(Redirect::to("/login"))
                        }
                        Err(_) => Err(String::from("Login failed"))
                    }
                } else {
                    Err(String::from("Password is incorrect."))
                }
            },
            Err(_) => Err(String::from("An error occurred")),
        }
        Err(_) => Err(String::from("No user with this email address.")),
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

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let context = json!({
        "header": "404 error",
        "path": req.uri().to_string()
    });

    Template::render("404", &context)
}

fn get_password_hash_from_email(data: String) -> Result<String, std::io::Error> {
    use rocket_simple_app::schema::users::dsl::*;

    let connection = estabilish_connection();
    let results = users
        .filter(email.eq(data))
        .limit(1)
        .load::<Users>(&connection)
        .expect("Error loading users");

    if results.len() == 1 {
        return Ok(results[0].password.to_string());
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No user found"
        ));
    }
}

fn get_user_id_from_email(data: String) -> u64 {
    use rocket_simple_app::schema::users::dsl::*;

    let connection = estabilish_connection();
    let user = users
        .filter(email.eq(data))
        .limit(1)
        .load::<Users>(&connection)
        .expect("Error loading users");

    return user[0].id;
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
        .mount("/", StaticFiles::from("public"))
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}