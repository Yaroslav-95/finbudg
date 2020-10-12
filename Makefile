VERSION=0.2.0

BUILDDIR=target
RELEASEDIR=$(BUILDDIR)/release
DOCSOUTPUTDIR=$(BUILDDIR)/man
PREFIX?=/usr
_INSTDIR=$(DESTDIR)$(PREFIX)
BINDIR?=$(_INSTDIR)/bin
MANDIR?=$(PREFIX)/share/man
CARGO?=cargo
CARGOFLAGS?=--release

all: build docs

build: build-unstripped
	strip $(RELEASEDIR)/finbudg

build-unstripped:
	$(CARGO) build $(CARGOFLAGS)

docs:
	mkdir -p $(DOCSOUTPUTDIR)
	scdoc < docs/finbudg.1.scd > $(DOCSOUTPUTDIR)/finbudg.1
	scdoc < docs/finbudg.5.scd > $(DOCSOUTPUTDIR)/finbudg.5

clean:
	rm -rf target/

install:
	mkdir -p $(BINDIR) $(MANDIR)/man1 $(MANDIR)/man5
	install -m755 $(RELEASEDIR)/finbudg $(BINDIR)/finbudg
	install -m644 $(DOCSOUTPUTDIR)/finbudg.1 $(MANDIR)/man1/finbudg.1
	install -m644 $(DOCSOUTPUTDIR)/finbudg.5 $(MANDIR)/man5/finbudg.5

uninstall:
	rm -f $(BINDIR)/finbudg
	rm -f $(MANDIR)/man1/finbudg.1
	rm -f $(MANDIR)/man5/finbudg.5

.PHONY: all build build-unstripped docs clean install uninstall
