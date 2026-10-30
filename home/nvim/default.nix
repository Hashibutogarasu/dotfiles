{ pkgs, ... }:

{
  programs.neovim = {
    enable = true;
    defaultEditor = true;
    viAlias = true;
    vimAlias = true;

    extraPackages = with pkgs; [
      typescript-language-server
    ];
  };

  xdg.configFile."nvim".source = ./.nvim;
}
