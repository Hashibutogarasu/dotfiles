{
  config,
  pkgs,
  home-manager,
  ...
}:

{
  targets.genericLinux.enable = true;

  xdg.mime.enable = true;

  nix.package = pkgs.nix;
  nix.nixPath = [ "home-manager=${home-manager}" ];
}
