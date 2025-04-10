drop:
	rm -rf assets/main.db
reload:
	rm -rf target Cargo.lock && $(MAKE) drop && $(MAKE) build
lint:
	cargo fmt --all
build:
	cargo build --release
update:
	cargo update
play:
	target/release/rs-library
hook:
	@echo "#!/bin/sh" > .git/hooks/pre-commit
	@echo "make lint && make build && git add --all" >> .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
