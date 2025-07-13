.PHONY: clean n nestest nh nestest-high

build:
	cargo build

n: nestest

nh: nestest-high

nestest:
	target/debug/green-nes -d low run tests/nestest.nes 2>&1 | tee out.txt

nestest-high:
	target/debug/green-nes -d high run tests/nestest.nes 2>&1 | tee out.txt
