CARGO=cargo
CARGOFLAGS="--release"

BINARIES=target/release
SOURCES=src
ASSETS=assets
DICT=/usr/dict

PREFIX?=/usr/local

.PHONY: install clean uninstall


$(BINARIES)/wordle-rs: $(SOURCES)/main.rs
	$(CARGO) build $(CARGOFLAGS)

install: $(BINARIES)/wordle-rs $(ASSETS)/words.sorted
	mv $(ASSETS)/words.sorted $(DICT)
	mv $(BINARIES)/wordle-rs $(PREFIX)/bin

clean:
	$(CARGO) clean

uninstall:
	rm $(DICT)/words.sorted
	rm $(PREFIX)/bin/wordle-rs
