{
  globals.mapleader = " ";

  opts = {
    number = true;
    relativenumber = true;
    expandtab = true;
    shiftwidth = 2;
    tabstop = 2;
    softtabstop = 2;
    undofile = true;
    mouse = "a";
    clipboard = "unnamedplus";
    mousemodel = "extend";
  };

  viAlias = true;
  vimAlias = true;

  lsp = {
    servers = {
      ts_ls.enable = true;
      nixd.enable = true;
    };

    keymaps = [
      {
        key = "gd";
        lspBufAction = "definition";
      }
      {
        key = "K";
        lspBufAction = "hover";
      }
      {
        key = "gr";
        lspBufAction = "references";
      }
      {
        key = "<leader>rn";
        lspBufAction = "rename";
      }
      {
        key = "<leader>ca";
        lspBufAction = "code_action";
      }
    ];
  };

  plugins.blink-cmp = {
    enable = true;
    setupLspCapabilities = true;

    settings = {
      keymap = {
        preset = "none";
        "<Tab>" = [
          "select_and_accept"
          "select_next"
          "snippet_forward"
          "fallback"
        ];
        "<S-Tab>" = [
          "select_prev"
          "snippet_backward"
          "fallback"
        ];
        "<CR>" = [
          "accept"
          "fallback"
        ];
        "<C-Space>" = [
          "show"
          "show_documentation"
          "hide_documentation"
        ];
      };

      sources.default = [
        "lsp"
        "path"
        "snippets"
        "buffer"
      ];
    };
  };
}
