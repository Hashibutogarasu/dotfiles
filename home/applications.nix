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
    google-chrome
    unityhub
    vscode
    yt-dlp
    rpi-imager
    cf
  ];

  programs.firefox.enable = true;
}
