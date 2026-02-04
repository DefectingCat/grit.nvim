" Grit Neovim Plugin - Vim Script Initialization

" This file ensures the plugin is properly loaded
" The main functionality is implemented in Rust and exposed through Lua

" Check if Neovim version supports Lua (which it does for 0.5+)
if !has('nvim')
  echoerr "Grit plugin requires Neovim"
  finish
endif

" Load the Lua module
lua require('grit')