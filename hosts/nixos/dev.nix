{ pkgs, nixvim, ... }:

{
  programs.java = {
    enable = true;
    package = pkgs.openjdk25;
  };

  environment.systemPackages = [
    pkgs.gcc
    (nixvim.legacyPackages.${pkgs.stdenv.hostPlatform.system}.makeNixvim (import ../../home/nixvim/settings.nix))
  ];
}
