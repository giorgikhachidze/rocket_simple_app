use super::schema::*;

#[derive(Queryable)]
pub struct UsersSessions {
    pub id: u64,
    pub user_id: u64,
    pub token: String
}

#[derive(Insertable)]
#[table_name = "users_sessions"]
pub struct NewUserSession {
    pub user_id: u64,
    pub token: String
}