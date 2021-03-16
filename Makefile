CONF=/etc/rad.toml
FILES=src/main.rs src/config.rs src/errors.rs
BINARY=target/release/rad

$(BINARY) : $(FILES)
	cargo build --release

install : $(BINARY)
	chown root:root $(BINARY)
	cp $(BINARY) /usr/bin/
	chmod 4711 /usr/bin/rad
	touch $(CONF)
	chown root:root $(CONF)
	chmod 600 $(CONF)
	cp rad.1 /usr/share/man/man1/
	cp rad.conf.5 /usr/share/man/man5/
	cp rad.pam /etc/pam.d/rad

uninstall :
	rm /usr/bin/rad
	rm /usr/share/man/man1/rad.1
	rm /usr/share/man/man5/rad.conf.5
	rm /etc/pam.d/rad
