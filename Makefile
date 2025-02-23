PREFIX ?= /usr/local
VERSION := $(shell grep '^version = ' Cargo.toml | cut -d '"' -f 2)

.PHONY: all
all: build-all

.PHONY: build-all
build-all: build-linux build-windows build-macos

.PHONY: build-linux
build-linux:
	@echo "Building Linux releases..."
	# Create build directories
	mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
	mkdir -p /tmp/ticket-tracker-$(VERSION)
	
	# Create source tarball
	cp -r src/ Cargo.toml Cargo.lock assets/ ticket-tracker.1 /tmp/ticket-tracker-$(VERSION)/
	cd /tmp && tar czf ~/rpmbuild/SOURCES/ticket-tracker-$(VERSION).tar.gz ticket-tracker-$(VERSION)/
	rm -rf /tmp/ticket-tracker-$(VERSION)
	
	# Build RPM
	cp .rpm/ticket-tracker.spec ~/rpmbuild/SPECS/
	rpmbuild -ba ~/rpmbuild/SPECS/ticket-tracker.spec
	
	# Build DEB (optional on Fedora)
	-cargo deb
	
	# Copy results to releases directory
	mkdir -p releases/linux
	cp target/release/ticket-tracker releases/linux/
	-cp ~/rpmbuild/RPMS/x86_64/ticket-tracker-$(VERSION)-*.rpm releases/linux/
	-cp target/debian/ticket-tracker_$(VERSION)*.deb releases/linux/
	@echo "Linux builds complete. Check releases/linux/ directory"
	@ls -l releases/linux/

.PHONY: build-windows
build-windows:
	@echo "Building Windows release..."
	# Install Windows target
	rustup target add x86_64-pc-windows-gnu
	# Build Windows binary
	cargo build --release --target x86_64-pc-windows-gnu
	# Copy results
	mkdir -p releases/windows
	cp target/x86_64-pc-windows-gnu/release/ticket-tracker.exe releases/windows/
	@echo "Windows build complete. Check releases/windows/ directory"
	@ls -l releases/windows/

.PHONY: build-macos
build-macos:
	@echo "Building macOS release..."
	# Install macOS target
	rustup target add x86_64-apple-darwin
	# Build macOS binary
	cargo build --release --target x86_64-apple-darwin
	# Create directory
	mkdir -p releases/macos
	# Copy binary
	-cp target/x86_64-apple-darwin/release/ticket-tracker releases/macos/
	@echo "macOS builds complete. Check releases/macos/ directory"
	@ls -l releases/macos/

.PHONY: install
install:
	install -d $(DESTDIR)$(PREFIX)/bin
	install -m 755 target/release/ticket-tracker $(DESTDIR)$(PREFIX)/bin/
	install -d $(DESTDIR)$(PREFIX)/share/man/man1
	install -m 644 ticket-tracker.1 $(DESTDIR)$(PREFIX)/share/man/man1/

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/ticket-tracker
	rm -f $(DESTDIR)$(PREFIX)/share/man/man1/ticket-tracker.1

.PHONY: clean-linux
clean-linux:
	rm -rf target/release/
	rm -rf releases/linux/
	rm -rf ~/rpmbuild/SOURCES/ticket-tracker-$(VERSION).tar.gz
	rm -rf ~/rpmbuild/RPMS/x86_64/ticket-tracker-$(VERSION)-*.rpm
	rm -rf target/debian/

.PHONY: clean-windows
clean-windows:
	rm -rf target/x86_64-pc-windows-gnu/
	rm -rf releases/windows/

.PHONY: clean-macos
clean-macos:
	rm -rf target/x86_64-apple-darwin/
	rm -rf releases/macos/

.PHONY: clean-all
clean-all: clean-linux clean-windows clean-macos
	cargo clean

.PHONY: clean
clean: clean-all 