{ config, lib, pkgs, ... }:

{
  services.openssh = {
    enable = true;
    openFirewall = true;
    authorizedKeysFiles = [
      ".ssh/authorized_keys"
      ".ssh/generated/system_key.pub"
    ];
    settings = {
      PasswordAuthentication = false;
      KbdInteractiveAuthentication = false;
      AuthenticationMethods = "publickey";
      PermitRootLogin = "no";
      AllowUsers = [ "crow" ];
      MaxAuthTries = 3;
      PerSourcePenalties = "crash:3600s authfail:3600s max:86400s";
    };
  };

  services.fprintd = {
    enable = true;
    package = pkgs.fprintd-tod;
    tod = {
      enable = true;
      driver = pkgs.libfprint-2-tod1-broadcom-cv3plus;
    };
  };

  services.ratbagd.enable = true;

  services.printing.enable = true;

  environment.systemPackages = [
    pkgs.pkg-config
    pkgs.flatpak
    pkgs.cloudflared
  ];
  environment.variables.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  services.flatpak.enable = true;

  services.pulseaudio.enable = false;
  security.rtkit.enable = true;
  services.pipewire = {
    enable = true;
    alsa.enable = true;
    alsa.support32Bit = true;
    pulse.enable = true;
  };
}
