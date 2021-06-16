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
                              * Set to `Perms::Some(&[])` if the user is not allowed
                              * to run anything and set to `Perms::All` if he can run everything.
                              * Write the programs names as explained in the `Perms` enumeration. 
                              */
    pub no_password: Perms<'a>, /* Same as above, but without password */
}

/*
 * This static variable contains the pork configuration for all users.
 * Configure the users that can run programs as explained above.
 *
 * You don't need to configure that for every user, if a user is not configured, he won't be able
 * to run anything.
 *
 * E.g.,
 *
 * pub static CONFIG: [Config; 1] = [Config {
 *   uid: 1000,
 *   programs: Perms::All,
 *   no_password: Perms::Some(&["/sbin/poweroff"]),
 * }];
 */
pub static CONFIG: [Config; 0] = [];
