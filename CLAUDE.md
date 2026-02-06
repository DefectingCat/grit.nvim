# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Grit Neovim Plugin

A Neovim plugin written in Rust using the nvim-oxi library. Provides Git status and management commands within Neovim.

## Key Files and Directories

- `src/lib.rs`: Rust implementation (main plugin logic)
- `syntax/grit.vim`: Syntax highlighting for GritStatus buffer
- `plugin/grit.vim`: Vim script initialization
- `lua/grit.so`: Compiled Rust shared library (generated)
- `Cargo.toml`: Cargo manifest file
- `tests/test_plugin.lua`: Integration tests for the plugin
- `Makefile`: Build automation
- `DEVELOPMENT.md`: Comprehensive development documentation
- `README.md`: User-facing documentation

## Build Commands

```bash
make build              # Build release version
make build-dev          # Build debug version
make test              # Run Rust unit tests
make test-plugin       # Run plugin integration tests in Neovim
make clean             # Clean all build artifacts
```

## Key Technologies

- **Rust**: Plugin implementation language (2024 edition)
- **nvim-oxi**: Rust bindings for Neovim API (version 0.6.0, Neovim 0.11)
- **Vim script**: Plugin initialization and syntax highlighting
- **Lua**: Integration testing and module loading

## Plugin Functionality

1. Plugin initializes via `grit()` function in `src/lib.rs:8`
2. Registers `:Grit` command using Neovim API
3. `:Grit` command creates a read-only GritStatus buffer
4. Displays Git status information and command hints
5. Supports syntax highlighting for command prefixes

## GritStatus Buffer

- **Name**: GritStatus
- **Type**: nofile (not associated with a real file)
- **Modifiability**: Read-only
- **Display**: No line numbers or relative line numbers
- **File Type**: grit (for syntax highlighting)

## Syntax Highlighting

Located in `syntax/grit.vim`:

- `GritHint`: Highlights "Hint: " as comment
- `GritTab`: Highlights "<tab>" as keyword
- `GritCommand`: Highlights command prefixes (za, s, u, x, c, ?) at word boundaries

**Commands**:
- `za`: Toggle (expand/collapse)
- `s`: Stage changes
- `u`: Unstage changes
- `x`: Discard changes
- `c`: Commit changes
- `?`: Show help

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
nvim -c "Grit" -c "q"              # Test :Grit command
```

## Requirements

- Neovim 0.9+ (built with LuaJIT support)
- Rust toolchain (2024 edition)

## Important Notes

- For comprehensive development documentation, see `DEVELOPMENT.md`
- For user-facing documentation, see `README.md`
- The plugin is configured to work with Neovim 0.11 via nvim-oxi 0.6.0
- macOS requires special linker configuration (`-undefined dynamic_lookup`) which is automatically handled by the build system
