" Maintainer: Allen
set background=dark
highlight clear
if exists("syntax_on")
  syntax reset
endif
let g:colors_name = "c4_neon"

hi Normal           guifg=#E0E2EE guibg=#0B0C10 ctermfg=254 ctermbg=232
hi CursorLine       guibg=#1F2833 ctermbg=235 gui=none cterm=none
hi LineNr           guifg=#45A29E guibg=#0B0C10 ctermfg=73 ctermbg=232
hi CursorLineNr     guifg=#66FCF1 guibg=#1F2833 ctermfg=81 ctermbg=235 gui=bold cterm=bold
hi Visual           guibg=#1F2833 ctermbg=235
hi Search           guifg=#0B0C10 guibg=#66FCF1 ctermfg=232 ctermbg=81
hi IncSearch        guifg=#0B0C10 guibg=#FF007F ctermfg=232 ctermbg=198
hi NonText          guifg=#1F2833 ctermfg=235
hi Pmenu            guifg=#E0E2EE guibg=#1F2833 ctermfg=254 ctermbg=235
hi PmenuSel         guifg=#66FCF1 guibg=#0B0C10 ctermfg=81 ctermbg=232
hi StatusLine       guifg=#66FCF1 guibg=#1F2833 ctermfg=81 ctermbg=235 gui=none
hi StatusLineNC     guifg=#45A29E guibg=#0B0C10 ctermfg=73 ctermbg=232 gui=none
hi VertSplit        guifg=#1F2833 guibg=#0B0C10 ctermfg=235 ctermbg=232

hi Keyword          guifg=#FF007F ctermfg=198 gui=bold cterm=bold
hi Conditional      guifg=#FF007F ctermfg=198 gui=bold cterm=bold
hi Repeat           guifg=#FF007F ctermfg=198 gui=bold cterm=bold
hi Operator         guifg=#66FCF1 ctermfg=81
hi Type             guifg=#00F0FF ctermfg=51  gui=bold cterm=bold
hi Typedef          guifg=#00F0FF ctermfg=51  gui=bold cterm=bold
hi Function         guifg=#FFE600 ctermfg=220
hi String           guifg=#39FF14 ctermfg=82
hi Character        guifg=#39FF14 ctermfg=82
hi Number           guifg=#FFE600 ctermfg=220
hi Float            guifg=#FFE600 ctermfg=220
hi Boolean          guifg=#00F0FF ctermfg=51
hi Constant         guifg=#00F0FF ctermfg=51
hi Special          guifg=#FF007F ctermfg=198
hi SpecialChar      guifg=#FFE600 ctermfg=220
hi Comment          guifg=#666666 ctermfg=242 gui=italic cterm=italic
hi Todo             guifg=#FF007F guibg=NONE    ctermfg=198 ctermbg=NONE gui=bold cterm=bold
hi Error            guifg=#FF0000 guibg=NONE    ctermfg=196 ctermbg=NONE gui=undercurl cterm=undercurl

hi c4PointerIntrinsic guifg=#FF007F ctermfg=198 gui=bold cterm=bold
hi c4Delimiter        guifg=#66FCF1 ctermfg=81

hi DiagnosticError           guifg=#FF0000 ctermfg=196 gui=none
hi DiagnosticUnderlineError  guisp=#FF0000 gui=undercurl cterm=undercurl
hi DiagnosticWarn            guifg=#FFE600 ctermfg=220 gui=none
hi DiagnosticUnderlineWarn   guisp=#FFE600 gui=undercurl cterm=undercurl
