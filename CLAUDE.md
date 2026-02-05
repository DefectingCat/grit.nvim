# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Grit Neovim Plugin

A Neovim plugin written in Rust using the nvim-oxi library. Provides:

- `:GritHello` command: Prints "Hello World from Grit plugin!" in the Neovim command line
- `:Grit` command: Opens a new tab with a GritStatus buffer showing Git status and commands

## Key Commands

Prefer using make for building

### Build

```bash
make build              # Build release version
make build-dev          # Build debug version
```

### Test

```bash
make test              # Run Rust unit tests
make test-plugin       # Run plugin integration tests in Neovim
```

### Clean

```bash
make clean             # Clean all build artifacts
```

## Project Architecture

### Structure

- `src/lib.rs`: Rust implementation (main plugin logic)
- `syntax/grit.vim`: Syntax highlighting for GritStatus buffer
- `plugin/grit.vim`: Vim script initialization
- `lua/grit.so`: Compiled Rust shared library (generated)
- `Cargo.toml`: Cargo manifest (nvim-oxi dependency)
- `tests/test_plugin.lua`: Integration tests for the plugin

### Key Technologies

- **Rust**: Plugin implementation language
- **nvim-oxi**: Rust bindings for Neovim API (version 0.6.0, Neovim 0.11)
- **Vim script**: Plugin initialization and syntax highlighting
- **Lua**: Integration testing and module loading

If you need to access nvim-oxi files, use `cargo vendor`

### Plugin Functionality

1. Plugin initializes via `grit()` function in `src/lib.rs:6`
2. Registers `:GritHello` and `:Grit` commands using Neovim API
3. `:Grit` command creates a read-only GritStatus buffer
4. Displays Git status information and command hints
5. Supports syntax highlighting for command prefixes

### GritStatus Buffer

- **Name**: GritStatus
- **Type**: nofile (not associated with a real file)
- **Modifiability**: Read-only
- **Display**: No line numbers or relative line numbers
- **File Type**: grit (for syntax highlighting)

### Syntax Highlighting

Located in `syntax/grit.vim`:

- `GritHint`: Highlights "Hint: " as comment
- `GritTab`: Highlights "<tab>" as keyword
- `GritCommand`: Highlights command prefixes (za, s, u, x, c, ?) at word boundaries

**Commands**:

- `za`: Toggle
- `s`: Stage
- `u`: Unstage
- `x`: Discard
- `c`: Commit
- `?`: Help

## Compilation and Installation

### Platform-Specific Notes

#### macOS

- Requires special linker configuration (`-undefined dynamic_lookup`)
- Compiled as `.dylib` but renamed to `.so` for Lua compatibility

#### Linux

- Standard shared library compilation as `.so`

#### Windows

- Compiles as `.dll`

### Makefile Automation

The Makefile automatically handles:

1. Compilation with Cargo
2. Copying the compiled library to `lua/grit.so`
3. Platform-specific library renaming

## Development Workflow

### Using lazy.nvim for Development

```lua
return {
  dir = '/home/xfy/Developer/grit',
  build = 'make build',
  config = function()
    require('grit')
  end,
}
```

### Manual Testing

```bash
nvim -c "Grit" -c "q"              # Test :Grit
```

## Requirements

- Neovim 0.9+ (built with LuaJIT support)
- Rust toolchain (2024 edition)
