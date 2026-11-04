{
  config,
  pkgs,
  home-manager,
  ...
}:

{
  targets.genericLinux.enable = true;

  xdg.mime.enable = true;

  xdg.configFile."home-manager" = {
    source = config.lib.file.mkOutOfStoreSymlink "${config.home.homeDirectory}/dotfiles";
    force = true;
  };

  home.sessionPath = [ "${config.xdg.stateHome}/nix/profiles/home-manager/home-path/bin" ];

  news.display = "silent";

  nix.package = pkgs.nix;
  nix.nixPath = [ "home-manager=${home-manager}" ];
}
