{ ... }:

{
  home.username = "crow";
  home.homeDirectory = "/home/crow";
  home.stateVersion = "26.05";

  imports = [
    ./applications.nix
    ./claude.nix
    ./cli.nix
    ./develop.nix
    ./git
    ./nvim
    ./ssh.nix
  ];

  programs.home-manager.enable = true;

  programs.direnv = {
    enable = true;
    enableBashIntegration = true;
    nix-direnv.enable = true;
  };
}
