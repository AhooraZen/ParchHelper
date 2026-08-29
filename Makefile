PREFIX ?= /usr
DESTDIR ?=

# Auto-detect sudo requirement for installation phase only
SUDO := $(shell if [ "$$(id -u)" -ne 0 ]; then which sudo 2>/dev/null; fi)

all: build

build:
	cargo build --release

test:
	cargo test

# install target does NOT depend on build rule to prevent root from re-invoking cargo
install:
	@if [ ! -f target/release/parch-helper ]; then \
		echo "Binary target/release/parch-helper not found! Running build as current user..."; \
		cargo build --release; \
	fi
	$(SUDO) install -Dm755 target/release/parch-helper $(DESTDIR)$(PREFIX)/bin/parch-helper
	$(SUDO) ln -sf $(PREFIX)/bin/parch-helper $(DESTDIR)$(PREFIX)/bin/parch-translate
	$(SUDO) mkdir -p $(DESTDIR)/etc/parch
	@if [ ! -f $(DESTDIR)/etc/parch/helper.toml ]; then \
		$(SUDO) install -Dm644 config/helper.toml $(DESTDIR)/etc/parch/helper.toml; \
	fi
	$(SUDO) install -Dm644 shell/parch-helper.sh $(DESTDIR)/etc/profile.d/parch-helper.sh
	$(SUDO) install -Dm644 shell/parch-helper.zsh $(DESTDIR)$(PREFIX)/share/zsh/site-functions/parch-helper.zsh
	$(SUDO) install -Dm644 shell/parch-helper.fish $(DESTDIR)$(PREFIX)/share/fish/vendor_conf.d/parch-helper.fish
	@echo "Creating symlinks for foreign package managers in $(DESTDIR)$(PREFIX)/bin/..."
	@for cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap; do \
		if [ ! -f "$(DESTDIR)$(PREFIX)/bin/$$cmd" ] || [ -L "$(DESTDIR)$(PREFIX)/bin/$$cmd" ]; then \
			$(SUDO) ln -sf $(PREFIX)/bin/parch-helper $(DESTDIR)$(PREFIX)/bin/$$cmd; \
		fi \
	done
	@echo "Installation complete!"

clean:
	cargo clean

.PHONY: all build test install clean
