pork
====

Permission OverRide Kontroller

Pork is a minimalistic alternative to `sudo` and `doas` that is meant to be simple and secure, therefore, its codebase fits under 120 SLOC to keep a clear and understandable code.

Configuring
-----------

`pork` is configured by editing `src/config.rs`. This file contains explanations and detailed examples on how to configure it.

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
