all: install

build:
	cargo build --release

install: build
	sudo cp -f target/release/hey /usr/local/bin
