{ config, pkgs, ... }:

{
  programs.coolercontrol.enable = true;
  programs.steam.enable = true;

  programs.obs-studio = {
    enable = true;
    package = pkgs.obs-studio.override { cudaSupport = true; };
    plugins = with pkgs.obs-studio-plugins; [
      wlrobs
      obs-backgroundremoval
      obs-pipewire-audio-capture
      obs-vaapi
      obs-gstreamer
      obs-vkcapture
    ];
  };

  environment.systemPackages = with pkgs; [
    git
    udev
    coolercontrol.coolercontrold
    openssl
    nh
  ];
}
