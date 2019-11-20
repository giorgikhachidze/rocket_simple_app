use super::schema::*;

#[derive(Queryable)]
pub struct Users {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Queryable)]
pub struct UsersSessions {
    pub id: u64,
    pub user_id: u64,
    pub token: String
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Insertable)]
#[table_name = "users_sessions"]
pub struct NewUserSession {
    pub user_id: u64,
    pub token: String
}