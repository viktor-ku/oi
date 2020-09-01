all: install

build:
	cargo build --release

install: build
	sudo cp -f target/release/oi /usr/local/bin
