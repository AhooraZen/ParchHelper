PREFIX ?= /usr
BINDIR ?= $(PREFIX)/bin
SYSCONFDIR ?= /etc
DATADIR ?= $(PREFIX)/share
DESTDIR ?=

CARGO ?= cargo
CARGO_FLAGS ?= --release

all: build

build:
	$(CARGO) build $(CARGO_FLAGS)

check:
	$(CARGO) check
	$(CARGO) clippy -- -D warnings

test:
	$(CARGO) test

install:
	install -Dm755 target/release/parch-helper $(DESTDIR)$(BINDIR)/parch-helper
	ln -sf parch-helper $(DESTDIR)$(BINDIR)/parch-translate
	install -Dm644 config/helper.toml $(DESTDIR)$(SYSCONFDIR)/parch/helper.toml
	install -Dm644 shell/parch-helper.sh $(DESTDIR)$(SYSCONFDIR)/profile.d/parch-helper.sh
	install -Dm644 shell/parch-helper.zsh $(DESTDIR)$(DATADIR)/zsh/site-functions/parch-helper.zsh
	install -Dm644 shell/parch-helper.fish $(DESTDIR)$(DATADIR)/fish/vendor_conf.d/parch-helper.fish
	@for cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap; do \
		ln -sf parch-helper $(DESTDIR)$(BINDIR)/$$cmd; \
	done

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/parch-helper
	rm -f $(DESTDIR)$(BINDIR)/parch-translate
	rm -f $(DESTDIR)$(SYSCONFDIR)/parch/helper.toml
	rm -f $(DESTDIR)$(SYSCONFDIR)/profile.d/parch-helper.sh
	rm -f $(DESTDIR)$(DATADIR)/zsh/site-functions/parch-helper.zsh
	rm -f $(DESTDIR)$(DATADIR)/fish/vendor_conf.d/parch-helper.fish
	@for cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap; do \
		if [ -L "$(DESTDIR)$(BINDIR)/$$cmd" ]; then \
			rm -f "$(DESTDIR)$(BINDIR)/$$cmd"; \
		fi \
	done

clean:
	$(CARGO) clean

.PHONY: all build check test install uninstall clean
