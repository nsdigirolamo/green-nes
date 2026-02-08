.PHONY: build-dev build-release dev release

build:
	cargo build -r

test:
	bash download-tests.sh
	cargo test

nestest:
	target/release/green-nes -d low run tests/nestest/nestest.nes 49152 > tests/nestest/nestest.out

colortest:
	target/release/green-nes -d low run tests/color_test/color_test.nes > tests/color_test/out.txt

pacman:
	target/release/green-nes -d low run tests/pacman.nes > tests/pacman.out
