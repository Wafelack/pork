pork
====

Permission OverRide Kontroller

Pork is a minimalistic alternative to `sudo` and `doas` that is meant to be simple and secure, therefore, its codebase fits under 120 SLOC to keep a clear and understandable code.

**Disclaimer: This hasn't received any security audit yet, the author is not responsible of any potential security problems occuring because of this program. You use it at your own risk.**

Configuration
-----------

The configuration of `pork` is done by editing the `src/config.rs` file and (re)compiling the source code.

Installing
----------

You will need a `make` program and the Rust toolchain v1.50+.
```bash
$ git clone https://github.com/miniutils/pork.git
$ cd pork/
$ make # This will make you configure and build pork using your $EDITOR.
$ make install [PREFIX=/somewhere/in/your/path]
```

License
-------

This program is licensed under the GNU General Public License version 3.0 and later.
