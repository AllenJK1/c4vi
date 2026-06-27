" Maintainer: Allen
set background=light
highlight clear
if exists("syntax_on")
  syntax reset
endif
let g:colors_name = "c4_light"

hi Normal           guifg=#2D3748 guibg=#F7FAFC ctermfg=236 ctermbg=255
hi CursorLine       guibg=#EDF2F7 ctermbg=254 gui=none cterm=none
hi LineNr           guifg=#A0AEC0 guibg=#F7FAFC ctermfg=248 ctermbg=255
hi CursorLineNr     guifg=#4A5568 guibg=#EDF2F7 ctermfg=240 ctermbg=254 gui=bold cterm=bold
hi Visual           guibg=#E2E8F0 ctermbg=253
hi Search           guifg=#FFFFFF guibg=#DD6B20 ctermfg=15 ctermbg=208
hi IncSearch        guifg=#FFFFFF guibg=#3182CE ctermfg=15 ctermbg=75
hi NonText          guifg=#E2E8F0 ctermfg=253
hi Pmenu            guifg=#2D3748 guibg=#EDF2F7 ctermfg=236 ctermbg=254
hi PmenuSel         guifg=#FFFFFF guibg=#3182CE ctermfg=15 ctermbg=75
hi StatusLine       guifg=#4A5568 guibg=#E2E8F0 ctermfg=240 ctermbg=253 gui=none
hi StatusLineNC     guifg=#A0AEC0 guibg=#F7FAFC ctermfg=248 ctermbg=255 gui=none
hi VertSplit        guifg=#E2E8F0 guibg=#F7FAFC ctermfg=253 ctermbg=255

hi Keyword          guifg=#0056B3 ctermfg=25  gui=bold cterm=bold
hi Conditional      guifg=#805AD5 ctermfg=92  gui=bold cterm=bold
hi Repeat           guifg=#805AD5 ctermfg=92  gui=bold cterm=bold
hi Operator         guifg=#2D3748 ctermfg=236
hi Type             guifg=#00A3C4 ctermfg=37  gui=bold cterm=bold
hi Typedef          guifg=#00A3C4 ctermfg=37  gui=bold cterm=bold
hi Function         guifg=#2B6CB0 ctermfg=26
hi String           guifg=#38A169 ctermfg=71
hi Character        guifg=#38A169 ctermfg=71
hi Number           guifg=#DD6B20 ctermfg=208
hi Float            guifg=#DD6B20 ctermfg=208
hi Boolean          guifg=#DD6B20 ctermfg=208
hi Constant         guifg=#3182CE ctermfg=75
hi Special          guifg=#DD6B20 ctermfg=208
hi SpecialChar      guifg=#805AD5 ctermfg=92
hi Comment          guifg=#A0AEC0 ctermfg=248 gui=italic cterm=italic
hi Todo             guifg=#DD6B20 guibg=NONE    ctermfg=208 ctermbg=NONE gui=bold cterm=bold
hi Error            guifg=#E53E3E guibg=NONE    ctermfg=167 ctermbg=NONE gui=undercurl cterm=undercurl

hi c4PointerIntrinsic guifg=#D69E2E ctermfg=178 gui=bold cterm=bold
hi c4Delimiter        guifg=#2D3748 ctermfg=236

hi DiagnosticError           guifg=#E53E3E ctermfg=167 gui=none
hi DiagnosticUnderlineError  guisp=#E53E3E gui=undercurl cterm=undercurl
hi DiagnosticWarn            guifg=#DD6B20 ctermfg=208 gui=none
hi DiagnosticUnderlineWarn   guisp=#DD6B20 gui=undercurl cterm=undercurl
