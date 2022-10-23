all: install

assets:
	sudo mkdir -p /usr/share/oi
	sudo cp assets/notification.wav /usr/share/oi

build:
	cargo build --release

install: assets build
	sudo cp -f target/release/oi /usr/local/bin
	sudo cp -f target/release/oid /usr/local/bin

uninstall:
	sudo rm -rf /usr/local/bin/oi /usr/local/bin/oid /usr/share/oi

generate-openapi:
	cargo run --bin oid -- --generate-openapi

.PHONY: assets
