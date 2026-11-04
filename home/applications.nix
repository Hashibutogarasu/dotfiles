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
  ];

  programs.firefox.enable = true;

  xdg.configFile."autostart/1password.desktop".text = ''
    [Desktop Entry]
    Type=Application
    Name=1Password
    Exec=/run/current-system/sw/bin/1password --silent
  '';
}
