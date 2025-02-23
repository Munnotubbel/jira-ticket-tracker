PREFIX ?= /usr/local

.PHONY: install
install:
	install -d $(DESTDIR)$(PREFIX)/bin
	install -m 755 target/release/ticket-ticker $(DESTDIR)$(PREFIX)/bin/
	install -d $(DESTDIR)$(PREFIX)/share/man/man1
	install -m 644 ticket-ticker.1 $(DESTDIR)$(PREFIX)/share/man/man1/

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/ticket-ticker
	rm -f $(DESTDIR)$(PREFIX)/share/man/man1/ticket-ticker.1 