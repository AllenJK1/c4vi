" Maintainer: Allen
set background=dark
highlight clear
if exists("syntax_on")
  syntax reset
endif
let g:colors_name = "c4_theme"

hi Normal           guifg=#D4D4D4 guibg=#1E1E1E ctermfg=252 ctermbg=234
hi CursorLine       guibg=#2D2D2D ctermbg=236 gui=none cterm=none
hi LineNr           guifg=#858585 guibg=#1E1E1E ctermfg=244 ctermbg=234
hi CursorLineNr     guifg=#C6C6C6 guibg=#2D2D2D ctermfg=251 ctermbg=236 gui=bold cterm=bold
hi Visual           guibg=#264F78 ctermbg=24
hi Search           guifg=#FFFFFF guibg=#613214 ctermfg=15 ctermbg=95
hi IncSearch        guifg=#000000 guibg=#E9A700 ctermfg=0 ctermbg=214
hi NonText          guifg=#3C3C3C ctermfg=237
hi Pmenu            guifg=#D4D4D4 guibg=#252526 ctermfg=252 ctermbg=235
hi PmenuSel         guifg=#FFFFFF guibg=#094771 ctermfg=15 ctermbg=24
hi StatusLine       guifg=#E7E7E7 guibg=#323232 ctermfg=254 ctermbg=236 gui=none
hi StatusLineNC     guifg=#858585 guibg=#2D2D2D ctermfg=244 ctermbg=236 gui=none
hi VertSplit        guifg=#3C3C3C guibg=#1E1E1E ctermfg=237 ctermbg=234

hi Keyword          guifg=#569CD6 ctermfg=75  gui=bold cterm=bold
hi Conditional      guifg=#C586C0 ctermfg=176 gui=bold cterm=bold
hi Repeat           guifg=#C586C0 ctermfg=176 gui=bold cterm=bold
hi Operator         guifg=#D4D4D4 ctermfg=252
hi Type             guifg=#4EC9B0 ctermfg=79  gui=bold cterm=bold
hi Typedef          guifg=#4EC9B0 ctermfg=79  gui=bold cterm=bold
hi Function         guifg=#DCDCAA ctermfg=187
hi String           guifg=#CE9178 ctermfg=173
hi Character        guifg=#CE9178 ctermfg=173
hi Number           guifg=#B5CEA8 ctermfg=151
hi Float            guifg=#B5CEA8 ctermfg=151
hi Boolean          guifg=#569CD6 ctermfg=75
hi Constant         guifg=#569CD6 ctermfg=75
hi Special          guifg=#C586C0 ctermfg=176
hi SpecialChar      guifg=#D7BA7D ctermfg=180
hi Comment          guifg=#6A9955 ctermfg=71  gui=italic cterm=italic
hi Todo             guifg=#FF8C00 guibg=NONE    ctermfg=208 ctermbg=NONE gui=bold cterm=bold
hi Error            guifg=#F44747 guibg=NONE    ctermfg=203 ctermbg=NONE gui=undercurl cterm=undercurl

hi c4PointerIntrinsic guifg=#D16969 ctermfg=167 gui=bold cterm=bold
hi c4Delimiter        guifg=#9CDCFE ctermfg=117

hi DiagnosticError           guifg=#F44747 ctermfg=203 gui=none
hi DiagnosticUnderlineError  guisp=#F44747 gui=undercurl cterm=undercurl
hi DiagnosticWarn            guifg=#FF8C00 ctermfg=208 gui=none
hi DiagnosticUnderlineWarn   guisp=#FF8C00 gui=undercurl cterm=undercurl
