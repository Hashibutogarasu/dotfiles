{ config, pkgs, ... }:

{
  imports = [
    ./applications.nix
    ./cli.nix
    ./develop.nix
    ./git.nix
    ./nvim
  ];

  home.username = "crow";
  home.homeDirectory = "/home/crow";
  home.stateVersion = "26.05"; # Please read the comment before changing.

  home.packages = [
  ];

  home.file = {
  };

  home.sessionVariables = {
    # EDITOR = "emacs";
  };

  programs.home-manager.enable = true;
}
