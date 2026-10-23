{
  config,
  lib,
  pkgs,
  ...
}:

let
  home = config.home.homeDirectory;
  jsonPath = "${home}/.config/home-manager/config/git.json";
  localConfig = "${home}/.config/git/local.conf";
  localDir = "${home}/.config/git/local.d";
  opSshSign = "${config.home.profileDirectory}/bin/op-ssh-sign";
  gitParser = pkgs.rustPlatform.buildRustPackage {
    pname = "GitParser";
    version = "0.1.0";
    src = ./utils;
    cargoLock.lockFile = ./utils/Cargo.lock;
    cargoBuildFlags = [ "--package" "GitParser" ];
  };
in
{
  programs.git = {
    enable = true;

    settings.gpg.ssh.program = lib.mkDefault opSshSign;

    includes = [
      {
        path = localConfig;
      }
    ];
  };

  home.activation.gitLocalConfig = lib.hm.dag.entryAfter [ "writeBoundary" "rustupInit" ] ''
    ${gitParser}/bin/GitParser ${lib.escapeShellArgs [ jsonPath localConfig localDir opSshSign ]}
  '';
}
