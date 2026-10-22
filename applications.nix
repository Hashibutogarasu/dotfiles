{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    zed-editor
    android-studio
    kdePackages.kate
    vlc
    piper
  ];

  programs.firefox.enable = true;
}
