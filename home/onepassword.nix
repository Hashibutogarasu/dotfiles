{ config, gitParser, lib, pkgs, ... }:

let
  configFile = "${config.home.homeDirectory}/dotfiles/config/1Password.json";
in
{
  home.activation.onePasswordKeys = lib.hm.dag.entryAfter [ "linkGeneration" ] ''
    PATH=${lib.makeBinPath [ pkgs.openssh config.programs.gpg.package ]}:/run/wrappers/bin:$PATH \
      ${gitParser}/bin/GitParser keys ${lib.escapeShellArg configFile} \
      || echo "1Password: key import failed (is 1Password unlocked?)" >&2
  '';
}
