{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    nixd
    nixfmt
    espflash
    android-tools
    rustup
    cmake
    fvm
  ];
}
