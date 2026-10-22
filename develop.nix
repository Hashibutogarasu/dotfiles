{
  config,
  lib,
  pkgs,
  ...
}:

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

  home.activation.rustupInit = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
    ${pkgs.rustup}/bin/rustup default stable
  '';
}
