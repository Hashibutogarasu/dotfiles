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
  sshConfig = "${home}/.config/git/ssh.conf";
  userSshConfig = "${home}/.ssh/config";
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
  home.packages = [ pkgs._1password-cli ];

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
    ${gitParser}/bin/GitParser ${lib.escapeShellArgs [ jsonPath localConfig localDir opSshSign sshConfig ]}

    if ! grep -qsF ${lib.escapeShellArg "Include ${sshConfig}"} ${lib.escapeShellArg userSshConfig}; then
      mkdir -p "$(dirname ${lib.escapeShellArg userSshConfig})"
      { echo ${lib.escapeShellArg "Include ${sshConfig}"}; echo; cat ${lib.escapeShellArg userSshConfig} 2>/dev/null; } > ${lib.escapeShellArg userSshConfig}.tmp
      mv ${lib.escapeShellArg userSshConfig}.tmp ${lib.escapeShellArg userSshConfig}
    fi
  '';
}
