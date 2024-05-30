.PHONY: test wf ast lint lint-fix clean clean-win

default: test

test: lint-fix
	cargo test -- --nocapture

wf: lint-fix
	cargo test --package verilog_hdl --features write_test

ast:
	cargo test --package verilog_hdl --lib -- tests::output_ast --exact --show-output  -- --nocapture

lint:
	@cargo clippy --all-targets --all-features
	@cargo fmt --all

lint-fix:
	cargo clippy --workspace --fix --allow-staged --allow-dirty

clean:
	cd ./verilog_hdl && rm results/*.v

clean-win:
	cd ./verilog_hdl && del results
