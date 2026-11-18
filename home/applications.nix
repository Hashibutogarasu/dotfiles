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
  ];

  programs.firefox.enable = true;
}
