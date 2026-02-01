.PHONY: build-dev build-release dev release

build-dev:
	cargo build

build-release:
	cargo build -r

nestest:
	target/release/green-nes -d low run tests/nestest.nes 49152 > tests/nestest.out

colortest:
	target/release/green-nes -d low run tests/color_test.nes > tests/color_test.out

pacman:
	target/release/green-nes -d low run tests/pacman.nes > tests/pacman.out
