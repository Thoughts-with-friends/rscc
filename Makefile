.PHONY: test, lint

default: test

test: lint-fix
	cargo test -- --nocapture > result.txt

ast:
	cargo test --package verilog_hdl --lib -- tests::output_ast --exact --show-output  -- --nocapture > ast.txt

lint:
	@cargo clippy --all-targets --all-features
	@cargo fmt --all

lint-fix:
	cargo clippy --workspace --fix --allow-staged --allow-dirty
