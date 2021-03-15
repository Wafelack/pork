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
use crate::*;
use pwhash::{md5_crypt, sha256_crypt, sha512_crypt};
use std::fs;

#[allow(deprecated)] // Only for MD5 and SHA256 hash that are still too used today.
pub fn check_password(user: &str, password: &str) -> Result<bool> {
    let content = fs::read_to_string("/etc/shadow")?;

    for line in content.lines() {
        let splited = line.split(':').collect::<Vec<_>>();

        if splited[0] == user {
            let password_fields = splited[1].split('$').collect::<Vec<_>>();

            let salt = format!("${}${}", password_fields[1], password_fields[2]);

            let hashed = match password_fields[1] {
                "1"  => {
                    match md5_crypt::hash_with(salt.as_str(), password) {
                        Ok(s) => s,
                        Err(_) => return Err(error("Failed to hash given password.")),
                    }
                }
                "5"  => {
                    match sha256_crypt::hash_with(salt.as_str(), password) {
                        Ok(s) => s,
                        Err(_) => return Err(error("Failed to hash given password.")),
                    }
                }
                "6"  => {
                    match sha512_crypt::hash_with(salt.as_str(), password) {
                        Ok(s) => s,
                        Err(_) => return Err(error("Failed to hash given password.")),
                    }
                }
                _ => return Err(error(format!("Uncovered algorithm: `${}`, please open an issue on <https://github.com/wafelack/rad>", password_fields[1])))
            };

            return Ok(hashed
                == format!(
                    "${}${}${}",
                    password_fields[1], password_fields[2], password_fields[3]
                ));
        }
    }

    Err(error(format!("No user named `{}` found.", user)))
}
