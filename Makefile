CONF=/etc/rad.toml
FILES=src/main.rs src/password.rs src/config.rs src/errors.rs
BINARY=target/release/rad

$(BINARY) : $(FILES)
	cargo build --release

install : $(BINARY)
	chown root:root $(BINARY)
	chmod 4751 $(BINARY)
	cp $(BINARY) /usr/bin/
	touch $(CONF)
	chown root:root $(CONF)
	chmod 600 $(CONF)
	cp rad.1 /usr/share/man/man1/
	cp rad.conf.5 /usr/share/man/man5/
