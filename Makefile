PREFIX ?= /usr
DESTDIR ?=

all: build

build:
	cargo build --release

test:
	cargo test

install: build
	install -Dm755 target/release/parch-helper $(DESTDIR)$(PREFIX)/bin/parch-helper
	ln -sf $(PREFIX)/bin/parch-helper $(DESTDIR)$(PREFIX)/bin/parch-translate
	install -Dm644 config/helper.toml $(DESTDIR)/etc/parch/helper.toml
	install -Dm644 shell/parch-helper.sh $(DESTDIR)/etc/profile.d/parch-helper.sh
	install -Dm644 shell/parch-helper.zsh $(DESTDIR)$(PREFIX)/share/zsh/site-functions/parch-helper.zsh
	install -Dm644 shell/parch-helper.fish $(DESTDIR)$(PREFIX)/share/fish/vendor_conf.d/parch-helper.fish

clean:
	cargo clean

.PHONY: all build test install clean
