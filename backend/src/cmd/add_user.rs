// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use clap::{Arg, ArgMatches, Command};

use crate::db::get_connection_pool;
use crate::error::Error;
use crate::models::users;
use crate::models::users::{NewUserReq, UserRole};

pub const CMD_ADD_USER: &str = "add-user";
const OPT_NAME: &str = "username";
const OPT_PASSWORD: &str = "password";
const OPT_EMAIL: &str = "email";
const OPT_DISPLAY_NAME: &str = "display-name";

pub fn add_user_cmd() -> Command<'static> {
    Command::new(CMD_ADD_USER)
        .about("Add admin user")
        .arg(
            Arg::new(OPT_NAME)
                .long(OPT_NAME)
                .takes_value(true)
                .required(true)
                .value_name(OPT_NAME)
                .help("Specify username"),
        )
        .arg(
            Arg::new(OPT_PASSWORD)
                .long(OPT_PASSWORD)
                .takes_value(true)
                .required(true)
                .value_name(OPT_PASSWORD)
                .help("Specify password"),
        )
        .arg(
            Arg::new(OPT_EMAIL)
                .long(OPT_EMAIL)
                .takes_value(true)
                .required(true)
                .value_name(OPT_EMAIL)
                .help("Specify email address"),
        )
        .arg(
            Arg::new(OPT_DISPLAY_NAME)
                .long(OPT_DISPLAY_NAME)
                .takes_value(true)
                .required(false)
                .value_name(OPT_DISPLAY_NAME)
                .help("Optionally specify display name"),
        )
}

pub fn add_user(matches: &ArgMatches) -> Result<(), Error> {
    let username = matches.value_of(OPT_NAME).unwrap();
    let password = matches.value_of(OPT_PASSWORD).unwrap();
    let email = matches.value_of(OPT_EMAIL).unwrap();
    let display_name = matches
        .value_of(OPT_DISPLAY_NAME)
        .unwrap_or_else(|| username);

    let db_pool = get_connection_pool()?;
    let pg_conn = db_pool.get()?;

    let new_user_req = NewUserReq {
        name: username.to_string(),
        display_name: display_name.to_string(),
        email: email.to_string(),
        role: UserRole::Admin,
        password: password.to_string(),
    };

    let user_info = users::add_user(&pg_conn, new_user_req)?;
    log::info!("New user: {:?}", user_info);

    Ok(())
}