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

# ... existing code ...

.PHONY: build-windows
build-windows:
	@echo "Building Windows release..."
	# Install Windows target and dependencies
	rustup target add x86_64-pc-windows-gnu
	# Ensure MinGW is installed for Windows cross-compilation
	which x86_64-w64-mingw32-gcc || echo "Please install mingw-w64"
	# Build Windows binary with static linking
	RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-gnu
	# Create installer directory
	mkdir -p releases/windows
	# Copy binary and assets
	cp target/x86_64-pc-windows-gnu/release/ticket-tracker.exe releases/windows/
	cp -r assets releases/windows/
	@echo "Windows build complete. Check releases/windows/ directory"
	@ls -l releases/windows/

.PHONY: build-macos
build-macos:
	@echo "Note: macOS builds must be performed on a macOS system"
	@if [ "$$(uname)" = "Darwin" ]; then \
		echo "Building macOS release..."; \
		rustup target add x86_64-apple-darwin aarch64-apple-darwin; \
		cargo build --release --target x86_64-apple-darwin; \
		cargo build --release --target aarch64-apple-darwin; \
		mkdir -p releases/macos/TicketTracker.app/Contents/{MacOS,Resources}; \
		lipo "target/x86_64-apple-darwin/release/ticket-tracker" \
			"target/aarch64-apple-darwin/release/ticket-tracker" \
			-create -output "releases/macos/TicketTracker.app/Contents/MacOS/ticket-tracker"; \
		cp -r assets releases/macos/TicketTracker.app/Contents/Resources/; \
		cp packaging/macos/Info.plist releases/macos/TicketTracker.app/Contents/; \
		echo "macOS builds complete. Check releases/macos/ directory"; \
		ls -l releases/macos/TicketTracker.app/Contents/MacOS/; \
	else \
		echo "Skipping macOS build on non-macOS system..."; \
	fi

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