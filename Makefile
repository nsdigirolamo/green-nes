.PHONY: clean

build:
	cargo build

nestest:
	target/debug/green-nes run tests/nestest.nes 2>&1 | tee out.txt

6502func:
	target/debug/green-nes run tests/6502_functionaL-test 2>&1 | tee out.txt
