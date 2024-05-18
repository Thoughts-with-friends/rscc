.PHONY: test, lint

default: test

test: lint
	cargo test -- --nocapture > result.txt

lint:
	@cargo clippy --all-targets --all-features
	@cargo fmt --all
