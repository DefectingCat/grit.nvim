" Grit buffer syntax highlighting file
" This file provides syntax highlighting for the GritStatus buffer

" Set up syntax highlighting
if exists("b:current_syntax")
  finish
endif

" Highlight "Hint: ", "/" and "|" as comment only on first line
syn match GritHint "Hint: " containedin=GritFirstLine
syn match GritHint "/" containedin=GritFirstLine
syn match GritHint "|" containedin=GritFirstLine
syn region GritFirstLine start="\%1l" end="\%1l$" contains=GritHint,GritTab,GritCommand
hi def GritHint ctermfg=green cterm=italic gui=italic
hi def link GritHint Comment

" Highlight <tab> in the first line as keyword (using Keyword color)
syn match GritTab "<tab>" containedin=ALL
hi def link GritTab Keyword

" Highlight command prefixes as keywords only on first line
" 匹配命令前缀，只在单词边界或特定上下文中高亮
" za - toggle，s - stage，u - unstage，x - discard，c - commit，? - help
syn match GritCommand "\<za\>" containedin=GritFirstLine
syn match GritCommand "\<s\>" containedin=GritFirstLine
syn match GritCommand "\<u\>" containedin=GritFirstLine
syn match GritCommand "\<x\>" containedin=GritFirstLine
syn match GritCommand "\<c\>" containedin=GritFirstLine
syn match GritCommand "?" containedin=GritFirstLine
hi def link GritCommand Keyword

" Mark syntax as loaded
let b:current_syntax = "grit"
