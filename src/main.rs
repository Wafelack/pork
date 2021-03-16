/*
 *     This file is part of rad.
 *
 *   rad is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   rad is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 *
 *   You should have received a copy of the GNU General Public License
 *
 *  along with rad.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{env, path::Path, process::Command};

mod config;
mod errors;

pub use errors::{error, RadError, Result};
use libc::{setuid, getuid, getpwuid};
use std::mem::size_of_val;

pub fn get_username(uid: u32) -> Result<String> {
    let returned = unsafe {
        getpwuid(uid)
    };

    if returned.is_null() {
        Err(error(format!("No user found for uid `{}`.", uid)))
    } else {
        let raw_name = unsafe {
            (*returned).pw_name
        };

        let mut to_ret = String::new();
        for i in 0..size_of_val(&raw_name) {
            unsafe {
                to_ret.push(*raw_name.offset(i as isize) as u8 as char); 
            }
        }

        Ok(to_ret)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn username() -> Result<()> {

        let user = get_username(unsafe {
            getuid()
        })?;

        println!("{}", user);

        Ok(())
    }
}

fn main() -> Result<()> {

    let config_file = "/etc/rad.toml";

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let usage = format!("Usage: {} command ARGS...", env!("CARGO_PKG_NAME"));

    if args.len() < 1 {
        return Err(error(usage));
    }

    if args[0] == "-h" || args[0] == "--help" {
        println!("{} - execute commands as administrator", env!("CARGO_PKG_NAME"));
        println!("{}", env!("CARGO_PKG_VERSION"));
        println!();
        println!("{}", usage);
        println!();
        println!("OPTIONS:");
        println!("\tcommand      \tThe command to run as root.");
        println!("\targs...      \tThe arguments to pass to the previously mentionned command.");
        println!("\t-h, --help   \tDisplays this message.");
        println!("\t-v, --version\tDisplays version information.");
        return Ok(());
    } else if args[0] == "-v" || args[0] == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(())
    }

    let command = &args[0];

    if !Path::new(config_file).exists() {
        Err(error(format!(
                    "Cannot find file `{}`. Consider creating it and adding content to it to use {}",
                    config_file,
                    env!("CARGO_PKG_NAME")
                    )))
    } else {

        let user = get_username(unsafe {
            getuid()
        })?;

        let (authorized, no_password) =
            config::can_run_program(command, &user, config_file)?;

        if !authorized {
            return Err(error("You are not authorized to perform this !"));
        }

        if !no_password {
            let mut pass =
                rpassword::prompt_password_stdout(&format!("[rad] Password for {}: ", user))
                .unwrap();
            let mut counter = 1;

            let mut auth = pam::Authenticator::with_password("system-auth").unwrap();
            auth.get_handler().set_credentials(&user, pass);

            while !auth.authenticate().is_ok() && counter < 3 {
                eprintln!("Authentication failed, please retry.");
                counter += 1;

                pass = rpassword::prompt_password_stdout(&format!("[rad] Password for {}: ", &user))
                    .unwrap();

                auth.get_handler().set_credentials(&user, pass);
            }

            if counter >= 3 {
                return Err(error("3 invalid password attempts. Aborting."));
            }
        }

        unsafe {
            if setuid(0) != 0 {
                return Err(error("Failed to change user id."));
            }
        }

        let arguments = if args.len() > 1 {
            args[1..].to_vec()
        } else {
            vec![]
        };

        Command::new(&command)
            .args(&arguments)
            .status()?;

        Ok(())
    }
}
