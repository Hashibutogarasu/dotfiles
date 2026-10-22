{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    zed-editor
    android-studio
    vlc
    piper
    _1password-cli
    _1password-gui
  ];

  programs.firefox.enable = true;
}
