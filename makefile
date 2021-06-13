FILES := $(shell find src/ -name '*.rs')
PROGRAM := pork
BINARY := target/release/$(PROGRAM)
PREFIX ?= /usr
EDITOR ?= vim

$(BINARY): $(FILES)
	$(EDITOR) src/config.rs
	cargo build --release
install: $(BINARY)
	chown root:root $(BINARY)
	cp $(BINARY) $(PREFIX)/bin/$(PROGRAM)
	chmod 4711 $(PREFIX)/bin/$(PROGRAM)
	cp $(PROGRAM).pam /etc/pam.d/$(PROGRAM)
	cp $(PROGRAM).1 $(PREFIX)/share/man/man1/
uninstall:
	rm $(PREFIX)/bin/$(PROGRAM)
	rm $(PREFIX)/share/man/man1/$(PROGRAM).1
	rm /etc/pam.d/$(PROGRAM)
