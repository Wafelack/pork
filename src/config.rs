#[derive(Copy, Clone)]
pub enum Perms<'a> {
    Some(&'a [&'a str]), /* The user can only run some programs.
                         * Each element must me the absolute path of the program
                         * in order to avoid potentially malicious programs
                         * with the same name placed in path.
                         */
    All, /* The user can run anything, be careful while giving this permission. */
}
#[derive(Copy, Clone)]
pub struct Config<'a> {
    pub uid: u32, /* Authorized user id, e.g., 0 for root. */
    pub programs: Perms<'a>, /* The programs the user is allowed to run.
                  Set to `Perms::Some(&[])` if the user is not allowed
                  to run anything and set to `Perms::All` if he can run everything.
                  Write the programs names as explained in the `Perms` enumeration. */
    pub no_password: Perms<'a>, /* Same as above, but without password */
}

/*
 * This function returns the config for all users.
 * Configure the users that can run programs as explained above.
 * You don't need to configure for every user, if a user is not
 * configured, he cannot run anything.
 */
pub fn gen_config<'a>() -> Vec<Config<'a>> {
    /* Example:
     *
     * vec![Config {
     *     uid: 123, /* Sample user */
     *     programs: Perms::All, /* Can run anything */
     *     no_password: Perms::Some(&["/sbin/poweroff"]), /* Can run `/sbin/poweroff` without specifying a password. */
     * }]
     */
     vec![]
}
