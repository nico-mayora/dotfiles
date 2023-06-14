" Neovim configuration.
" Huge thank you to every FOSS contributor who made this editor a reality.
" Nicolás Mayora 2021-2022.

""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
" => Basic configuration
""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""

set number
set cursorline
highlight clear CursorLine
hi CursorLineNr term=bold cterm=bold ctermfg=010 gui=bold
set mouse=a
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

""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
" => Vim Plug for Plugin Management
""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""

call plug#begin()

" {{ Basic utilities }}
    Plug 'nvim-telescope/telescope.nvim'                        " Fuzzy finder
    Plug 'nvim-treesitter/nvim-treesitter', {'do': ':TSUpdate'} " Better syntax highlighting
    Plug 'ludovicchabant/vim-gutentags'                         " Tag generation and maintainance
    Plug 'nvim-lua/plenary.nvim'                                " Lua functions
    Plug 'tpope/vim-commentary'                                 " For Commenting gcc
    Plug 'tpope/vim-surround'                                   " Surround commands
    Plug 'tpope/vim-endwise'                                    " Autoappend do, then, end, etc
    Plug 'easymotion/vim-easymotion'                            " Better motion
    Plug 'kevinhwang91/nvim-ufo'                                " Improve folding
    Plug 'kevinhwang91/promise-async'                           " Required for ^^^
" {{ Productivity }}
    Plug 'akinsho/git-conflict.nvim'                            " Resolve git conflicts
    Plug 'lewis6991/gitsigns.nvim'
    Plug 'nvim-neo-tree/neo-tree.nvim'                          " Fs explorer
    Plug 'MunifTanjim/nui.nvim'
    Plug 'tpope/vim-fugitive'                                   " Git integration
    Plug 'tpope/vim-obsession'                                  " Better session management
    Plug 'nvim-telescope/telescope-fzf-native.nvim', { 'do': 'make' }
    Plug 'stevearc/aerial.nvim'                                 " Outliner
" {{ LSP & autocompletion }}
    Plug 'neovim/nvim-lspconfig'                                " LSP config
    Plug 'williamboman/mason.nvim'                              " LSP/format/dap installer
    Plug 'williamboman/mason-lspconfig.nvim'                    " Mason LSP
    Plug 'mfussenegger/nvim-dap'                                " Debug adapter protocol
    Plug 'rcarriga/nvim-dap-ui'                                 " In-editor interface for nvim-dap
    " Misc. autocomplete sources
    Plug 'hrsh7th/cmp-nvim-lsp'                         
    Plug 'hrsh7th/cmp-buffer'                                  
    Plug 'hrsh7th/cmp-path'                                     
    Plug 'hrsh7th/nvim-cmp' 
    Plug 'hrsh7th/cmp-vsnip'
    Plug 'hrsh7th/vim-vsnip'
    Plug 'rafamadriz/friendly-snippets'                         " Snippets support
    Plug 'github/copilot.vim'                                   " Copilot integration
    Plug 'sbdchd/neoformat'                                     " Autoformatting
" {{ Work-specific }}
    Plug 'antoinemadec/FixCursorHold.nvim'                      " Workaround fix
    Plug 'nvim-neotest/neotest'                                 " Testing framework
    Plug 'olimorris/neotest-rspec'                              " Rspec adapter
    Plug 'tpope/vim-rails'                                      " rails utilities
    Plug 'vim-ruby/vim-ruby'                                    " Ruby integration
" {{ Appearance }}
    Plug 'folke/todo-comments.nvim'                             " Fancy comments
    Plug 'goolord/alpha-nvim'                                   " Fancy startup
    Plug 'akinsho/bufferline.nvim'                              " Better buffline
    Plug 'nvim-lualine/lualine.nvim'                            " Bottom bar
    Plug 'kyazdani42/nvim-web-devicons'                         " Better icons
    Plug 'lukas-reineke/indent-blankline.nvim'                  " Indentation guides
    Plug 'sainnhe/gruvbox-material'                             " Gruvbox with treesitter support
    Plug 'loctvl842/monokai-pro.nvim'                           " Monokai pro theme
    Plug 'navarasu/onedark.nvim'                                " Onedark theme
    Plug 'folke/lsp-colors.nvim'                                " LSP colours
    Plug 'catppuccin/nvim', { 'as': 'catppuccin' }              " Catpucchin theme

call plug#end()

""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""
" => Run configuration for lua plugins
""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""

runtime! lua/*.lua

lua << EOF
require('todo-comments').setup()
require('aerial').setup {} 
require('gitsigns').setup()
require('git-conflict').setup()
EOF


" => VimL configuration "

" Snippets "
if (&ft=='rb')
    :let g:vsnip_filetypes.ruby = ['rails']
endif

" LSP & formatter "
autocmd FileType ruby setlocal omnifunc=LanguageClient#complete

hi DiagnosticError guifg=#88088F
hi DiagnosticWarn guifg=DarkOrange
hi DiagnosticInfo guifg=Blue
hi DiagnosticHint guifg=Green

sign define DiagnosticSignError text= texthl=DiagnosticError linehl= numhl=
sign define DiagnosticSignWarn  text= texthl=DiagnosticWarn  linehl= numhl=
sign define DiagnosticSignInfo  text= texthl=DiagnosticInfo  linehl= numhl=
sign define DiagnosticSignHint  text= texthl=DiagnosticHint  linehl= numhl=

let g:neoformat_enabled_ruby = ['rubocop']
let g:neoformat_enabled_haskell = ['stylishhaskell']

" Vim-cmp "
    "see lua file for more "
set completeopt=menu,menuone,noselect

" GitHub Copilot "
let g:copilot_filetypes = {
      \ '*': v:false,
      \ 'ruby': v:true,
      \ 'haskell': v:true,
      \ }

" Gutentags "
let g:gutentags_enabled = 1
let g:gutentags_project_root = ['Gemfile'] 

" Easymotion config "
let g:EasyMotion_do_mapping = 1

" -------------------- "
" => Set current theme "
" -------------------- "
set termguicolors

" {{ Onedark }}
" let g:onedark_config = {
"     \ 'style': 'cool',
" \}
" colorscheme onedark 

" {{ Monokai Pro }}
" let g:monokaipro_filter = "spectrum"
" let g:monokaipro_italic_functions = 1
" let g:monokaipro_sidebars = [ "nvim-tree" ]
" colorscheme monokaipro

colorscheme catppuccin-macchiato

" ---------------------------- "
" => Configure custom mappings "
" ---------------------------- "

nnoremap k gk
nnoremap j gj

imap jk <Esc>
imap kj <Esc>
tnoremap <Esc> <C-\><C-n>
nnoremap <space> :
nnoremap <C-Space> za
nnoremap <C-n> :Neotree<CR>

" Go to definition "
nnoremap gD <cmd>lua vim.lsp.buf.declaration()<CR>
nnoremap gd <cmd>lua vim.lsp.buf.definition()<CR>
nnoremap ^] gd
nnoremap K <cmd>lua vim.lsp.buf.hover()<CR>

" Navigate open buffers "
nnoremap <leader>d :bp<CR>
nnoremap <leader>f :bn<CR>

map <F5> :source Session.vim<CR>
map <F9> :nohl<CR>
map <F10> :wa<CR>
nnoremap <F8> <cmd>lua require('neotest').summary.toggle()<CR>

nnoremap <leader>ss :lua require('telescope.builtin').treesitter()<cr>
nnoremap <leader>r :Telescope registers<cr>
nnoremap <leader>b :Telescope buffers<cr>
nnoremap <leader>g :Telescope live_grep<CR>
nnoremap <leader>a :AerialToggle!<CR>
nnoremap <C-P> :Telescope find_files<CR>

" Custom commands "
command Nt :Neotree
command Fmt :Neoformat

" Use ctrl-[hjkl] to select the active split "
nmap <silent> <c-k> :wincmd k<CR>
nmap <silent> <c-j> :wincmd j<CR>
nmap <silent> <c-h> :wincmd h<CR>
nmap <silent> <c-l> :wincmd l<CR>
