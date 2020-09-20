all: install

assets:
	sudo mkdir -p /usr/share/oi
	sudo cp assets/notification.wav /usr/share/oi

build:
	cargo build --release

install: build assets
	sudo cp -f target/release/oi /usr/local/bin

.PHONY: assets
