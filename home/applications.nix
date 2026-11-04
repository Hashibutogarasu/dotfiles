{ config, pkgs, ... }:

{
  imports = [ ./claude.nix ];

  home.packages = with pkgs; [
    zed-editor
    godot
    ffmpeg
    (callPackage ./gamemaker.nix { })
    android-studio
    vlc
    piper
    _1password-cli
    _1password-gui
    google-chrome
  ];

  programs.firefox.enable = true;
}
