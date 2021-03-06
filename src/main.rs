mod config;
use config::{Config, Perms};
use libc::{c_char, getpwuid, getuid, setuid};
use std::{
    env,
    ffi::CString,
    path::Path,
    process::{exit, Command},
};

fn main() {
    match try_main() {
        Ok(()) => exit(0),
        Err(e) => eprintln!("pork: {}", e.0),
    }
    exit(1);
}
fn try_main() -> Result<()> {
    let mut args = env::args().skip(1);
    let cfg = config::CONFIG;
    let uid = unsafe { getuid() };
    let command = get_path(
        args.nth(0)
            .map_or(Err(Error("Missing operand.".to_string())), |v| Ok(v))?,
    );

    let ucfg = match cfg.iter().position(|p| p.uid == uid) {
        Some(idx) => Ok(cfg[idx]),
        None => Err(Error(format!("No configuration available for {}.", uid))),
    }?;

    let no_password = match allowed(&command, ucfg) {
        0 => Err(Error(format!(
            "{} is not allowed to perform this command.",
            uid
        ))),
        x => Ok(x == 2),
    }?;

    if !no_password {
        let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
        let mut auth = pam::Authenticator::with_password("pork").unwrap();
        auth.get_handler().set_credentials(
            unsafe {
                CString::from_raw(((*getpwuid(uid)).pw_name) as *mut c_char)
                    .into_string()
                    .unwrap()
                    .as_str()
            },
            pass,
        );
        auth.authenticate()
            .map_err(|_| Error(format!("Invalid password.")))?;
    }

    unsafe {
        if setuid(0) != 0 {
            /* Set identity to root */
            Err(Error("Failed to change user id.".to_string()))
        } else {
            Ok(())
        }
    }?;

    let status = Command::new(&command)
        .args(args)
        .status()
        .map_err(|e| Error(format!("Failed to run command: {}: {}", command, e)))?;

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }

    Ok(())
}
fn get_path(program: String) -> String {
    env::var("PATH")
        .iter()
        .flat_map(|v| v.split(':').map(ToString::to_string))
        .find_map(|directory| {
            let full = format!("{}/{}", directory, program);
            if Path::new(&full).exists() {
                Some(full.replace("//", "/"))
            } else {
                None
            }
        })
        .unwrap_or_else(|| program.to_string())
}
fn allowed(program: &str, config: Config) -> u8 {
    /* 0: Cannot ; 1: Password; 2: No password */
    match config.no_password {
        Perms::All => Some(2),
        Perms::Some(programs) => {
            if programs.contains(&program) {
                Some(2)
            } else {
                None
            }
        }
    }
    .unwrap_or(
        match config.programs {
            Perms::All => Some(1),
            Perms::Some(programs) => {
                if programs.contains(&program) {
                    Some(1)
                } else {
                    None
                }
            }
        }
        .unwrap_or(0),
    )
}
#[derive(Debug)]
pub struct Error(pub String);
pub type Result<T> = std::result::Result<T, Error>;
