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

" Add syntax directory to runtime path
let s:plugin_dir = fnamemodify(expand('<sfile>'), ':h:h')
execute 'set runtimepath+=' . s:plugin_dir

" Enable syntax highlighting for GritStatus buffer
augroup grit_syntax
  autocmd!
  autocmd BufEnter GritStatus setlocal syntax=grit
augroup END
