{ ... }:
{
  home.username = "crow";
  home.homeDirectory = "/home/crow";
  home.stateVersion = "25.05";

  imports = [
    ./home/git
    ./home/gpg.nix
    ./home/onepassword.nix
    ./home/ssh.nix
  ];

  programs.home-manager.enable = true;

  news.display = "silent";
}
