.PHONY: test, lint

default: test

test:
	cargo test -- --nocapture > result.txt

lint:
	@cargo clippy --all-targets --all-features
	@cargo fmt --all
