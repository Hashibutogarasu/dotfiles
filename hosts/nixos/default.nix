{ ... }:

{
  imports = [
    ./hardware.nix
    ./core.nix
    ./build.nix
    ./graphics.nix
    ./desktop.nix
    ./fonts.nix
    ./apps.nix
    ./services.nix
    ./dev.nix
    ./nix-ld.nix
    ../partition.nix
    ../1Password.nix
  ];
}
