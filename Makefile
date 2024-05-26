.PHONY: test, lint

default: test

test: lint-fix
	cargo test -- --nocapture > result.txt

lint:
	@cargo clippy --all-targets --all-features
	@cargo fmt --all

lint-fix:
	cargo clippy --workspace --fix --allow-staged --allow-dirty
