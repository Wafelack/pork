# rad

rad is a program which allows a user to run a command as an administrator.
Its name is the acronym of **R**un as **Ad**ministrator.

It has only been tested on GNU/Linux (Calculate Linux) but should work on all \*nix OSs.

## Installing

### From source (recommended)

You will need Cargo and GNU Make to perform this installation.

```
$ git clone git@github.com:Wafelack/rad.git
$ cd rad/
$ chmod +x configure
$ ./configure
$ make
# make install
```

### From the release page

* Download the latest archive from the releases page.
* Execute the following script as root (You will need to have bash and tar installed):

```bash
#!/usr/bin/env bash
set -euo pipefail

KERNEL=$(uname -a | awk '{print $1}')

LOCAL=""
GROUP="root"

if [[ $KERNEL == "Darwin" ]]
then
	GROUP="wheel"
	LOCAL="local/"
fi

if [[ ! -f rad.tar.bz2 ]]
then
	echo "No archive in the working directory."
	exit 1
fi
tar -xjf rad.tar.bz2
chown root:$(GROUP) rad
mv rad /usr/$(LOCAL)bin
chmod 4751 /usr/$(LOCAL)bin/rad
mv rad.1 /usr/$(LOCAL)share/man/man1/
mv rad.conf.5 /usr/$(LOCAL)share/man/man5/
mv rad.pam /etc/pam.d/rad
```

## Getting help

If you notice a bug, report it in [an issue](https://github.com/wafelack/rad/issues/new).

To learn how to configure and use rad, read rad(1) and rad.conf(5).

## License

This program is licensed under the GNU General Public License version 3.0.
