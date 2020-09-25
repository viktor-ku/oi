all: test

build:
	cargo build --all

test: build
	../target/debug/oid --sandbox test --detach \
		&& cargo t -- --nocapture \
		&& bash ./scripts/clean.sh || bash ./scripts/clean.sh

