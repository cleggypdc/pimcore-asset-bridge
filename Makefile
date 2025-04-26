build-daemon:
	cd daemon && cargo build --release

install-daemon:
	cp daemon/target/release/asset-bridge-daemon pimcore-bundle/bin/asset-bridge-daemon

build: build-daemon install-daemon
