# Grit Neovim Plugin

A simple Hello World plugin for Neovim written in Rust using the nvim-oxi library.

## Features

- Provides a `:GritHello` command that prints "Hello World from Grit plugin!" in the Neovim command line

## Installation

### Requirements

- Neovim 0.9+ (built with LuaJIT support)
- Rust toolchain (for building the plugin)

### Build and Install

1. Clone or download this repository
2. Build the plugin in release mode:
   ```bash
   cargo build --release
   ```
3. Install using your favorite plugin manager:

#### Using Lazy.nvim

```lua
{
  'your-username/grit',
  build = 'cargo build --release',
  config = function()
    require('grit')
  end,
}
```

#### Using Packer.nvim

```lua
use {
  'your-username/grit',
  run = 'cargo build --release',
  config = function()
    require('grit')
  end,
}
```

## Usage

After installing, run the following command in Neovim:

```vim
:GritHello
```

This will print "Hello World from Grit plugin!" in the Neovim command line.

## Development

### Project Structure

- `src/lib.rs`: Rust implementation of the plugin
- `lua/grit/init.lua`: Lua module entry point
- `plugin/grit.vim`: Vim script initialization
- `Cargo.toml`: Cargo manifest file

### Building

```bash
cargo build --release
```

### Testing

Run the tests:
```bash
cargo test
```

## License

MIT