vim.g.mapleader = " "

vim.opt.number = true
vim.opt.relativenumber = true
vim.opt.expandtab = true
vim.opt.shiftwidth = 2
vim.opt.tabstop = 2
vim.opt.softtabstop = 2
vim.opt.undofile = true
vim.opt.mouse = ""
vim.opt.clipboard = ""

dofile(vim.fn.stdpath("config") .. "/lazy.lua")
