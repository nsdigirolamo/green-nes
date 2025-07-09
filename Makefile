.PHONY: clean

build:
	cargo build

nestest:
	target/debug/green-nes run tests/nestest.nes 2>&1 | tee out.txt
