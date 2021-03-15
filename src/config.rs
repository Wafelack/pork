use crate::*;
use std::fs;
use toml::Value;

pub fn can_run_program(program: &str, user: &str, config_file: &str) -> Result<(bool, bool)> {
    let content = match fs::read_to_string(config_file)?.parse::<Value>() {
        Ok(v) => v,
        Err(_) => return Err(error("Invalid configuration file.")),
    };

    if !content.as_table().unwrap().contains_key(user) {
        Err(error(format!("You are not in `{}` file !", config_file)))
    } else {
        let user_perms = &content.as_table().unwrap()[user];

        if user_perms.as_table().is_some() {
            let mapped = user_perms.as_table().unwrap();

            let mut authorized = false;
            let mut no_pass = false;

            if mapped.contains_key("no_password") {
                if let Value::String(s) = &mapped["no_password"] {
                    if s.as_str() == "ALL" {
                        authorized = true;
                        no_pass = true;
                    }
                } else if let Value::Array(v) = &mapped["no_password"] {
                    for element in v {
                        if let Value::String(s) = element {
                            if s.as_str() == program {
                                authorized = true;
                                no_pass = true;
                                break;
                            }
                        }
                    }
                }
            }

            if mapped.contains_key("programs") {
                if let Value::String(s) = &mapped["programs"] {
                    if s.as_str() == "ALL" {
                        authorized = true;
                    }
                } else if let Value::Array(v) = &mapped["programs"] {
                    for element in v {
                        if let Value::String(s) = element {
                            if s.as_str() == program {
                                authorized = true;
                                break;
                            }
                        }
                    }
                }
            }

            Ok((authorized, no_pass))
        } else {
            Ok((false, false))
        }
    }
}
