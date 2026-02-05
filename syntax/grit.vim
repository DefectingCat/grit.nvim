" Grit buffer syntax highlighting file
" This file provides syntax highlighting for the GritStatus buffer

" Set up syntax highlighting
if exists("b:current_syntax")
  finish
endif

" Highlight "Hint: " as comment
syn match GritHint "Hint: " containedin=ALL
hi def link GritHint Comment

" Highlight <tab> in the first line as keyword (using Keyword color)
syn match GritTab "<tab>" containedin=ALL
hi def link GritTab Keyword

" Mark syntax as loaded
let b:current_syntax = "grit"
