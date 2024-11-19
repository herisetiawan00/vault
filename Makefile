build:
	cargo build --release

install:
	cp target/release/vault /usr/bin/vault

uninstall:
	sudo rm /usr/bin/vault
