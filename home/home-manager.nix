{ config, ... }:

{
  xdg.configFile."home-manager" = {
    source = config.lib.file.mkOutOfStoreSymlink "${config.home.homeDirectory}/dotfiles";
    force = true;
  };
}
