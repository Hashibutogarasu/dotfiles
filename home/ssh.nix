{ config, lib, pkgs, ... }:

let
  sshUtils = pkgs.callPackage ../ssh/utils/package.nix { };
in
{
  home.packages = [ sshUtils ];

  home.activation.sshKeyGeneration = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
    if [ ! -e "${config.home.homeDirectory}/.ssh/generated/system_key" ]; then
      ${sshUtils}/bin/ssh-utils generate --output-dir "${config.home.homeDirectory}/.ssh/generated"
    fi
  '';
}
