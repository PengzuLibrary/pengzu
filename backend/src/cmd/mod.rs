// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use clap::Command;
use std::env;

use crate::error::Error;
use crate::import::daemon;
use crate::routers;

mod add_user;

const CMD_RUN: &str = "run";

fn run_server_cmd() -> Command<'static> {
    Command::new(CMD_RUN).about("Run server")
}

fn run_server() -> Result<(), Error> {
    let rt = actix_rt::Runtime::new()?;
    let handle = rt.spawn(async { routers::run().await });
    rt.block_on(handle).unwrap()
}

pub fn run() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut cmd = Command::new("backend")
        .version(env!("VERGEN_GIT_SEMVER"))
        .author("Xu Shaohua <shaohua@biofan.org>")
        .about("Pengzu backend app")
        .subcommand(run_server_cmd())
        .subcommand(add_user::add_user_cmd())
        .subcommand(daemon::parse_cmdline());
    let matches = cmd.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches(add_user::CMD_ADD_USER) {
        return add_user::add_user(&matches);
    }
    if let Some(_matches) = matches.subcommand_matches(CMD_RUN) {
        return run_server();
    }
    if let Some(matches) = matches.subcommand_matches(daemon::CMD_IMPORT_LIBRARY) {
        return daemon::run_daemon(matches);
    }

    cmd.print_help().map_err(Into::into)
}
