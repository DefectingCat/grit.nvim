.PHONY: all build test test-plugin clean

all: build

build:
	cargo build --release
	cp target/release/libgrit.so lua/grit.so

test:
	cargo test

test-plugin: build
	nvim --headless -c "luafile tests/test_plugin.lua"

clean:
	cargo clean
