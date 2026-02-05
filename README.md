# Grit Neovim Plugin

A Neovim plugin written in Rust using the nvim-oxi library. Provides Git status and management commands within Neovim.

## Features

- `:GritHello`: Prints "Hello World from Grit plugin!" in the Neovim command line
- `:Grit`: Opens a new tab with a GritStatus buffer showing Git status and command hints
- Syntax highlighting for command prefixes in the GritStatus buffer

## Installation

### Requirements

- Neovim 0.9+ (built with LuaJIT support)
- Rust toolchain (for building the plugin)

### Build and Install

1. Clone or download this repository
2. Build the plugin in release mode:
   ```bash
   make build
   ```
3. Install using your favorite plugin manager:

#### Using Lazy.nvim

```lua
{
  'your-username/grit',
  build = 'make build',
  config = function()
    require('grit')
  end,
}
```

#### Using Packer.nvim

```lua
use {
  'your-username/grit',
  run = 'make build',
  config = function()
    require('grit')
  end,
}
```

## Usage

### Basic Commands

```vim
:GritHello
```
Prints "Hello World from Grit plugin!" in the Neovim command line.

```vim
:Grit
```
Opens a new tab with a GritStatus buffer that shows:
- Git status information
- Command hints: `<tab> za toggle | s stage | u unstage | x discard | c commit | ? help`

### GritStatus Buffer Commands

The GritStatus buffer supports the following commands:

- `za`: Toggle (expand/collapse)
- `s`: Stage changes
- `u`: Unstage changes
- `x`: Discard changes
- `c`: Commit changes
- `?`: Show help

## Development

### Project Structure

- `src/lib.rs`: Rust implementation (main plugin logic)
- `syntax/grit.vim`: Syntax highlighting for GritStatus buffer
- `plugin/grit.vim`: Vim script initialization
- `lua/grit.so`: Compiled Rust shared library (generated)
- `Cargo.toml`: Cargo manifest file
- `tests/test_plugin.lua`: Integration tests for the plugin
- `Makefile`: Build automation

### Build Commands

```bash
make build              # Build release version
make build-dev          # Build debug version
cargo build --release   # Direct Cargo command
```

### Testing

```bash
make test              # Run Rust unit tests
make test-plugin       # Run plugin integration tests in Neovim
cargo test             # Direct Cargo test command
```

### Cleanup

```bash
make clean             # Clean all build artifacts
cargo clean            # Direct Cargo clean command
```

## License

MIT