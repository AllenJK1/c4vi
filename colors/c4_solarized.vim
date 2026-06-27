" Maintainer: Allen
set background=dark
highlight clear
if exists("syntax_on")
  syntax reset
endif
let g:colors_name = "c4_solarized"

hi Normal           guifg=#839496 guibg=#002B36 ctermfg=66  ctermbg=233
hi CursorLine       guibg=#073642 ctermbg=234  gui=none cterm=none
hi LineNr           guifg=#586E75 guibg=#002B36 ctermfg=242 ctermbg=233
hi CursorLineNr     guifg=#93A1A1 guibg=#073642 ctermfg=109 ctermbg=234  gui=bold cterm=bold
hi Visual           guibg=#073642 ctermbg=234
hi Search           guifg=#002B36 guibg=#B58900 ctermfg=233 ctermbg=136
hi IncSearch        guifg=#002B36 guibg=#CB4B16 ctermfg=233 ctermbg=166
hi NonText          guifg=#073642 ctermfg=234
hi Pmenu            guifg=#839496 guibg=#073642 ctermfg=66  ctermbg=234
hi PmenuSel         guifg=#93A1A1 guibg=#002B36 ctermfg=109 ctermbg=233
hi StatusLine       guifg=#93A1A1 guibg=#073642 ctermfg=109 ctermbg=234  gui=none
hi StatusLineNC     guifg=#586E75 guibg=#002B36 ctermfg=242 ctermbg=233  gui=none
hi VertSplit        guifg=#073642 guibg=#002B36 ctermfg=234 ctermbg=233

hi Keyword          guifg=#859900 ctermfg=100  gui=bold cterm=bold
hi Conditional      guifg=#859900 ctermfg=100  gui=bold cterm=bold
hi Repeat           guifg=#859900 ctermfg=100  gui=bold cterm=bold
hi Operator         guifg=#93A1A1 ctermfg=109
hi Type             guifg=#268BD2 ctermfg=33   gui=bold cterm=bold
hi Typedef          guifg=#268BD2 ctermfg=33   gui=bold cterm=bold
hi Function         guifg=#2AA198 ctermfg=37
hi String           guifg=#2AA198 ctermfg=37
hi Character        guifg=#2AA198 ctermfg=37
hi Number           guifg=#D33682 ctermfg=125
hi Float            guifg=#D33682 ctermfg=125
hi Boolean          guifg=#D33682 ctermfg=125
hi Constant         guifg=#CB4B16 ctermfg=166
hi Special          guifg=#CB4B16 ctermfg=166
hi SpecialChar      guifg=#CB4B16 ctermfg=166
hi Comment          guifg=#586E75 ctermfg=242  gui=italic cterm=italic
hi Todo             guifg=#D33682 guibg=NONE    ctermfg=125 ctermbg=NONE gui=bold cterm=bold
hi Error            guifg=#DC322F guibg=NONE    ctermfg=160 ctermbg=NONE gui=undercurl cterm=undercurl

hi c4PointerIntrinsic guifg=#CB4B16 ctermfg=166 gui=bold cterm=bold
hi c4Delimiter        guifg=#93A1A1 ctermfg=109

hi DiagnosticError           guifg=#DC322F ctermfg=160 gui=none
hi DiagnosticUnderlineError  guisp=#DC322F gui=undercurl cterm=undercurl
hi DiagnosticWarn            guifg=#B58900 ctermfg=136 gui=none
hi DiagnosticUnderlineWarn   guisp=#B58900 gui=undercurl cterm=undercurl
