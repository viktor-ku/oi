all: test

build:
	cargo build --all

test: build
	../target/debug/oid --sandbox test \
		& cargo t -- --nocapture \
		&& bash ./scripts/clean.sh || bash ./scripts/clean.sh

