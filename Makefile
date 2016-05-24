CLIPPY_COMMAND=rustup run nightly cargo clippy --release
CLIPPY_ARGS=-Dclippy -Wclippy_pedantic

test:
	cargo test

check:
	cargo check

lint:
	$(CLIPPY_COMMAND) -- $(CLIPPY_ARGS)

outdated:
	cargo outdated -R

update:
	cargo update

install:
	@-cargo uninstall heartbeat
	cargo install

help:
	@echo "Available options:"
	@echo "  - check: Quickly validate all binaries compiles"
	@echo "  - install: Installs the project using cargo"
	@echo "  - lint: Lint all binaries against clippy"
	@echo "  - outdated: List outdated dependency information"
	@echo "  - test: Run cargo test"
	@echo "  - update: Update transitive dependencies versions"

PHONY: test check outdated update install help
