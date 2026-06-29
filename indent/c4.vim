if exists("b:did_indent")
  finish
endif
let b:did_indent = 1

setlocal indentexpr=GetC4Indent()
setlocal indentkeys=0{,0},0),0],:,!^F,o,O,e

function! GetC4Indent()
  let lnum = prevnonblank(v:lnum - 1)
  if lnum == 0
    return 0
  endif

  let ind = indent(lnum)
  let line = getline(lnum)

  let clean_line = substitute(line, '//.*$', '', '')
  let clean_line = substitute(clean_line, '\s\+$', '', '')

  if clean_line =~ '[[{(]$'
    let ind = ind + shiftwidth()
  endif

  let cur_line = getline(v:lnum)
  let clean_cur = substitute(cur_line, '//.*$', '', '')
  let clean_cur = substitute(clean_cur, '^\s\+', '', '')
  let clean_cur = substitute(clean_cur, '\s\+$', '', '')

  if clean_cur =~ '^[]})]'
    let ind = ind - shiftwidth()
  endif

  if ind < 0
    let ind = 0
  endif

  return ind
endfunction
