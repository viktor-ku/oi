all: install

build:
	cargo build --release

install: build
	sudo cp -f target/release/oi /usr/local/bin
	sudo cp -f target/release/oid /usr/local/bin

uninstall:
	sudo rm -f /usr/local/bin/oi /usr/local/bin/oid

generate-openapi:
	cargo run --bin oid -- --generate-openapi

.PHONY: assets
