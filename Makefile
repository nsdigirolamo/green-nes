.PHONY: build test nestest colortest pacman donkey-kong

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
	target/release/green-nes run tests/pacman/pacman.nes > tests/pacman/pacman.out

donkey-kong:
	target/release/green-nes -d low run tests/donkey-kong/donkey-kong.nes > tests/donkey-kong/donkey-kong.out
