#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{Users, NewUser, NewUserSession, UsersSessions};

pub fn estabilish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set, on .env file!");
    MysqlConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

pub fn create_new_user_session(connection: &MysqlConnection, user_id: u64, token: String) -> UsersSessions {
    use schema::users_sessions;

    let new_user_session = NewUserSession {
        user_id: user_id,
        token: token
    };

    diesel::insert_into(users_sessions::table)
        .values(&new_user_session)
        .execute(connection)
        .expect("Error saving new session!");

    users_sessions::table
        .order(users_sessions::id.desc())
        .first(connection)
        .unwrap()
}

pub fn create_new_user(connection: &MysqlConnection, email: String, password: String) -> Users {
    use schema::users;

    let new_user = NewUser {
        email: email,
        password: password
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user!");

    users::table
        .order(users::id.desc())
        .first(connection)
        .unwrap()
}