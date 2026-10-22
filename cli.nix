{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    gh
    just
    screenfetch
    unzip
    gnupg
    usbutils
    lm_sensors
    nvtopPackages.nvidia
  ];
}
