{ config, pkgs, users, ... }:

{
  programs._1password.enable = true;
  programs._1password-gui = {
    enable = true;
    polkitPolicyOwners = [ users.crow.name ];
  };

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
    home-manager
  ];
}
