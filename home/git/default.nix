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
  sshKeygen = pkgs.writeShellScript "git-ssh-keygen" ''
    SSH_AUTH_SOCK=$(${config.programs.gpg.package}/bin/gpgconf --list-dirs agent-ssh-socket)
    export SSH_AUTH_SOCK
    exec ${pkgs.openssh}/bin/ssh-keygen "$@"
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

    settings = {
      core.askPass = "";
      gpg.ssh.program = "${sshKeygen}";
    };

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
