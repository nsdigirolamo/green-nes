build-dev:
	cargo build

build-release:
	cargo build -r

run-dev:
	target/debug/green-nes -d low run tests/nestest.nes > tests/nestest.out

run-release:
	target/release/green-nes -d low run tests/nestest.nes > tests/nestest.out

run-tests:
	bash download-tests.sh
	cargo test

.PHONY: build-dev build-release run-dev
