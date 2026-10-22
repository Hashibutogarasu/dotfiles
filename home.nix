{ config, pkgs, ... }:

{
  home.username = "crow";
  home.homeDirectory = "/home/crow";
  home.stateVersion = "26.05"; # Please read the comment before changing.

  home.packages = [

  home.file = {
  };

  home.sessionVariables = {
    # EDITOR = "emacs";
  };

  programs.home-manager.enable = true;
}
