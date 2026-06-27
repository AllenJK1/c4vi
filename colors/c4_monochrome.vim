" Maintainer: Allen
set background=dark
highlight clear
if exists("syntax_on")
  syntax reset
endif
let g:colors_name = "c4_monochrome"

hi Normal           guifg=#E0E0E0 guibg=#0A0A0A ctermfg=253 ctermbg=232
hi CursorLine       guibg=#1C1C1C ctermbg=234 gui=none cterm=none
hi LineNr           guifg=#666666 guibg=#0A0A0A ctermfg=242 ctermbg=232
hi CursorLineNr     guifg=#FFB000 guibg=#1C1C1C ctermfg=214 ctermbg=234 gui=bold cterm=bold
hi Visual           guibg=#2C2C2C ctermbg=236
hi Search           guifg=#0A0A0A guibg=#FFB000 ctermfg=232 ctermbg=214
hi IncSearch        guifg=#0A0A0A guibg=#FFFFFF ctermfg=232 ctermbg=15
hi NonText          guifg=#2C2C2C ctermfg=236
hi Pmenu            guifg=#E0E0E0 guibg=#1C1C1C ctermfg=253 ctermbg=234
hi PmenuSel         guifg=#FFB000 guibg=#2C2C2C ctermfg=214 ctermbg=236
hi StatusLine       guifg=#FFB000 guibg=#1C1C1C ctermfg=214 ctermbg=234 gui=none
hi StatusLineNC     guifg=#666666 guibg=#0A0A0A ctermfg=242 ctermbg=232 gui=none
hi VertSplit        guifg=#1C1C1C guibg=#0A0A0A ctermfg=234 ctermbg=232

hi Keyword          guifg=#FFFFFF ctermfg=15   gui=bold cterm=bold
hi Conditional      guifg=#FFFFFF ctermfg=15   gui=bold cterm=bold
hi Repeat           guifg=#FFFFFF ctermfg=15   gui=bold cterm=bold
hi Operator         guifg=#E0E0E0 ctermfg=253
hi Type             guifg=#FFB000 ctermfg=214  gui=bold cterm=bold
hi Typedef          guifg=#FFB000 ctermfg=214  gui=bold cterm=bold
hi Function         guifg=#E0E0E0 ctermfg=253
hi String           guifg=#888888 ctermfg=102
hi Character        guifg=#888888 ctermfg=102
hi Number           guifg=#FFB000 ctermfg=214
hi Float            guifg=#FFB000 ctermfg=214
hi Boolean          guifg=#FFB000 ctermfg=214
hi Constant         guifg=#FFB000 ctermfg=214
hi Special          guifg=#FFFFFF ctermfg=15
hi SpecialChar      guifg=#FFB000 ctermfg=214
hi Comment          guifg=#444444 ctermfg=238  gui=italic cterm=italic
hi Todo             guifg=#FFB000 guibg=NONE    ctermfg=214 ctermbg=NONE gui=bold cterm=bold
hi Error            guifg=#FF0000 guibg=NONE    ctermfg=196 ctermbg=NONE gui=undercurl cterm=undercurl

hi c4PointerIntrinsic guifg=#FFB000 ctermfg=214 gui=bold cterm=bold
hi c4Delimiter        guifg=#E0E0E0 ctermfg=253

hi DiagnosticError           guifg=#FF0000 ctermfg=196 gui=none
hi DiagnosticUnderlineError  guisp=#FF0000 gui=undercurl cterm=undercurl
hi DiagnosticWarn            guifg=#FFB000 ctermfg=214 gui=none
hi DiagnosticUnderlineWarn   guisp=#FFB000 gui=undercurl cterm=undercurl
