.PHONY: all build test test-plugin clean

all: build

build:
	cargo build --release
	@# Copy the correct library file based on OS
	@if [ "$(shell uname -s)" = "Darwin" ]; then \
		cp target/release/libgrit.dylib lua/grit.so; \
	elif [ "$(shell uname -s)" = "Linux" ]; then \
		cp target/release/libgrit.so lua/grit.so; \
	elif [ "$(shell uname -s)" = "Windows_NT" ]; then \
		cp target/release/grit.dll lua/grit.dll; \
	else \
		echo "Unsupported OS: $(shell uname -s)"; \
		exit 1; \
	fi

test:
	cargo test

test-plugin: build
	nvim --headless -c "set rtp+=$(shell pwd)" -c "luafile tests/test_plugin.lua"

clean:
	cargo clean
