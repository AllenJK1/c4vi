set nocompatible
filetype off

let s:plugin_dir = expand('<sfile>:p:h')

execute 'set runtimepath+=' . fnameescape(s:plugin_dir)

filetype plugin indent on
syntax enable

set number
set tabstop=4
set shiftwidth=4
set expandtab
set autoindent
set smartindent

set hlsearch
set incsearch
set ignorecase
set smartcase

set backspace=indent,eol,start
set laststatus=2
set ruler

if has('termguicolors')
    set termguicolors
endif

silent! colorscheme c4_theme
