" Vim syntax file for KERN language
" Language: KERN
" Maintainer: KERN Development Team

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword kernKeyword entity rule flow constraint
syn keyword kernControl if then else loop break halt
syn keyword kernBoolean true false

" Identifiers
syn match kernIdentifier "\<[a-zA-Z_][a-zA-Z0-9_]*\>"

" Comments
syn match kernComment "//.*$"
syn region kernComment start="/\*" end="\*/"

" Strings
syn region kernString start=+"+ end=+"+ skip=+\\\\\|\\"+

" Numbers
syn match kernNumber "\<\d\+\>"

" Operators
syn match kernOperator "=\|==\|!=\|>\|<\|>=\|<=\|+\|-\|\*\|/"

" Define highlighting
hi def link kernKeyword Keyword
hi def link kernControl Conditional
hi def link kernBoolean Boolean
hi def link kernIdentifier Identifier
hi def link kernComment Comment
hi def link kernString String
hi def link kernNumber Number
hi def link kernOperator Operator

let b:current_syntax = "kern"