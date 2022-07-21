// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::schema::users;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum UserRole {
    Nil = 0,
    User = 1,
    Admin = 2,
}

impl diesel::Expression for UserRole {
    type SqlType = diesel::sql_types::Integer;
}

impl From<i32> for UserRole {
    fn from(role: i32) -> Self {
        match role {
            1 => Self::User,
            2 => Self::Admin,
            _ => Self::Nil,
        }
    }
}

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: i32,
    pub salt: String,
    pub hash: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: UserRole,
    pub created: NaiveDateTime,
}

fn user_to_user_info(user: User) -> UserInfo {
    UserInfo {
        id: user.id,
        name: user.name,
        display_name: user.display_name,
        email: user.email,
        role: user.role.into(),
        created: user.created,
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub role: i32,
    pub hash: String,
    pub salt: String,
}

pub fn add_user(conn: &PgConnection, new_user: &NewUser) -> Result<UserInfo, Error> {
    let user = diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)?;
    Ok(user_to_user_info(user))
}

pub fn get_all_users() -> Result<Vec<User>, Error> {
    todo!()
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub fn login(conn: &PgConnection, form: &LoginForm) -> Result<UserInfo, Error> {
    let _user = users::table
        .filter(users::name.eq(&form.username))
        .first::<User>(conn)?;
    todo!()
}

pub fn get_user_info(_id: i32) -> Result<UserInfo, Error> {
    todo!()
}

pub fn delete_user() -> Result<(), Error> {
    todo!()
}
