{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    zed-editor
    (callPackage ./gamemaker.nix { })
    android-studio
    vlc
    piper
    _1password-cli
    _1password-gui
  ];

  programs.firefox.enable = true;
}
