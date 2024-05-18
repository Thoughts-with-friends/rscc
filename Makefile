.PHONY: test, lint

default: test

test:
	cargo test

lint:
	@cargo clippy --all-targets --all-features
	@cargo fmt --all
