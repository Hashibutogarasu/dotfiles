{ pkgs, ... }:

{
  programs.nixvim = (import ./settings.nix) // {
    enable = true;
    defaultEditor = true;
    nixpkgs.source = pkgs.path;
  };
}
