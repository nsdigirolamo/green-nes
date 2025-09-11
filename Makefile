.PHONY: build-dev build-release dev release test

build-dev:
	cargo build

build-release:
	cargo build -r

dev:
	target/debug/green-nes -d low run tests/nestest.nes > tests/nestest.out

release:
	target/release/green-nes -d low run tests/nestest.nes > tests/nestest.out

test:
	bash download-tests.sh
	cargo test
