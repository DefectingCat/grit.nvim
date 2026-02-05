```
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
```

## Grit Neovim Plugin

A simple Neovim plugin written in Rust using the nvim-oxi library. Provides a `:GritHello` command that prints "Hello World from Grit plugin!" in the Neovim command line, and a `:Grit` command that opens a new tab.

## Key Commands

### Build

```bash
make build
```

### Test

```bash
make test              # Same as above via Makefile
make test-plugin       # Run plugin integration tests in Neovim
```

### Clean

```bash
make clean             # Same as above via Makefile
```

## Project Architecture

### Structure

- `src/lib.rs`: Rust implementation (main plugin logic)
- `lua/grit/init.lua`: Lua module entry point (loads Rust library)
- `plugin/grit.vim`: Vim script initialization
- `Cargo.toml`: Cargo manifest (nvim-oxi dependency)
- `tests/test_plugin.lua`: Integration tests for the plugin

### Key Technologies

- **Rust**: Plugin implementation language
- **nvim-oxi**: Rust bindings for Neovim API
- **Lua**: Neovim integration layer
- **Vim script**: Plugin initialization

### Plugin Functionality

1. Plugin initializes via `grit()` function in `src/lib.rs:6`
2. Registers `:GritHello` command using Neovim API
3. Command executes Vim `echo` to display hello message
4. Returns a dictionary with `{ hello = "world" }` to Lua

### Testing

- Rust tests: Standard cargo test framework
- Integration tests: Neovim headless mode running Lua test script (`tests/test_plugin.lua`)
- Tests verify: Plugin loading, command registration, and command execution

## Requirements

- Neovim 0.9+ (built with LuaJIT support)
- Rust toolchain (for building the plugin)
