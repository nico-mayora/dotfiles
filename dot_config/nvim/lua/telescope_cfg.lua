local action_state = require "telescope.actions.state"
local actions = require "telescope.actions"
local telescope = require "telescope"

telescope.setup {
    defaults = {
        vimgrep_arguments = {
            "rg",
            "--no-heading",
            "--with-filename",
            "--line-number",
            "--column",
            "--smart-case",
            "--hidden",
        },
        color_devicons = true,
        mappings = {
            n = {
                ['<c-d>'] = require('telescope.actions').delete_buffer
            },
            i = {
                ["<C-j>"] = actions.move_selection_next,
                ["<C-k>"] = actions.move_selection_previous,
            },
        },
    },
    extensions = {
        fzf = {
            fuzzy = true, -- false will only do exact matching
            override_generic_sorter = true,
            override_file_sorter = true,
            case_mode = "smart_case", -- this is default
        },
    },
}

require("telescope").load_extension "fzf"
