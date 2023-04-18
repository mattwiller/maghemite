.PHONY: test build clean nodejs

test:
	cargo check && cargo test

build: test
	cargo build --release

clean:
	rm -rf target dist

nodejs: build
	mkdir -p dist
	cp target/release/libmaghemite.so dist/index.node
