{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    zed-editor
    android-studio
    vlc
    piper
  ];

  programs.firefox.enable = true;
}
