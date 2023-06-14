-- Neovim configuration.
-- Huge thank you to every FOSS contributor who made this editor a reality.
-- Nicolás Mayora 2021-2022.

------------------------------
-- => Basic configuration
-----------------------------

vim.opt.number = true
vim.opt.cursorline = true
-- highlight clear CursorLine
-- hi CursorLineNr term=bold cterm=bold ctermfg=010 gui=bold
vim.mouse=a
set softtabstop=4
set tabstop=4
set shiftwidth=4
set expandtab
set noshowmode
set signcolumn=yes
set updatetime=100
let mapleader = ","
set iskeyword-=_

" Highlight yanked text
augroup highlight_yank
  autocmd!
  autocmd TextYankPost * silent! lua require'vim.highlight'.on_yank()
augroup END

" Use persistent history
set undodir=~/.config/nvim/undodir
set undofile
