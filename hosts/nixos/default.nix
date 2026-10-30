{ ... }:

{
  imports = [
    ./hardware.nix
    ./core.nix
    ./graphics.nix
    ./desktop.nix
    ./fonts.nix
    ./apps.nix
    ./services.nix
    ./dev.nix
    ./nix-ld.nix
  ];
}
