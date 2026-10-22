{ pkgs, home-manager, ... }:

let
  username = builtins.getEnv "USER";
  homeDirectory = builtins.getEnv "HOME";
in
{
  nix.package = pkgs.nix;
  nix.nixPath = [ "home-manager=${home-manager}" ];

  imports = [
    ./applications.nix
    ./cli.nix
    ./develop.nix
    ./git.nix
    ./nvim
  ];

  home.username = username;
  home.homeDirectory = homeDirectory;
  home.stateVersion = "26.05";

  home.packages = [
  ];

  home.file = {
  };

  home.sessionVariables = {
  };

  programs.home-manager.enable = true;
}
