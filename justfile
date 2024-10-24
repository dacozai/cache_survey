set shell := ["zsh", "-cu"]

cr := "cargo run"

setup:
	bash ./scripts/setup/dev_setup.sh

fmt:
	cargo fmt --all

lint:
	cargo fmt --all
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test

unit-test:
	ulimit -n 10000; ulimit -s 16384; RUST_LOG="ERROR" bash ./scripts/ci/ci-run-unit-tests.sh

profile:
	bash ./scripts/ci/ci-run-profile.sh


clean:
	cargo clean
