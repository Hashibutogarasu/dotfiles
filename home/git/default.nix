{
  config,
  lib,
  pkgs,
  ...
}:

let
  home = config.home.homeDirectory;
  jsonPath = "${home}/.config/git/git.json";
  localConfig = "${home}/.config/git/local.conf";
  localDir = "${home}/.config/git/local.d";
  sshConfig = pkgs.writeText "git-ssh-config" ''
    Host github.com
      HostName github.com
      User git
      IdentityAgent /run/user/%i/gnupg/S.gpg-agent.ssh
      IdentitiesOnly yes
  '';
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

    settings.core.askPass = "";

    includes = [
      {
        path = localConfig;
      }
    ];
  };

  home.activation.gitLocalConfig = lib.hm.dag.entryAfter [ "writeBoundary" "rustupInit" ] ''
    ${gitParser}/bin/GitParser ${lib.escapeShellArgs [ jsonPath localConfig localDir "${sshConfig}" ]}
  '';
}
