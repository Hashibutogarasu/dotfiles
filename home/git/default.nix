{
  config,
  lib,
  pkgs,
  ...
}:

let
  home = config.home.homeDirectory;
  jsonPath = "${home}/dotfiles/config/git.json";
  localConfig = "${home}/.config/git/local.conf";
  localDir = "${home}/.config/git/local.d";
  sshConfig = pkgs.writeText "git-ssh-config" ''
    Host github.com
      HostName github.com
      User git
      IdentityAgent /run/user/%i/ssh-agent
      IdentitiesOnly yes
      UserKnownHostsFile ${home}/.ssh/github_known_hosts
      StrictHostKeyChecking yes
  '';
  sshKeygen = pkgs.writeShellScript "git-ssh-keygen" ''
    export SSH_AUTH_SOCK="/run/user/$(id -u)/ssh-agent"
    exec ${pkgs.openssh}/bin/ssh-keygen "$@"
  '';
  ghWrapper = pkgs.writeShellScriptBin "gh" ''
    if [ -z "''${GH_CONFIG_DIR:-}" ]; then
      helper=$(git config --get-all credential.https://github.com.helper 2>/dev/null | tail -n1)
      if [[ $helper =~ GH_CONFIG_DIR=\'([^\']*)\' ]]; then
        export GH_CONFIG_DIR="''${BASH_REMATCH[1]}"
      fi
    fi
    exec ${pkgs.gh}/bin/gh "$@"
  '';
  gitParser = pkgs.rustPlatform.buildRustPackage {
    pname = "GitParser";
    version = "0.1.0";
    src = ./utils;
    cargoLock.lockFile = ./utils/Cargo.lock;
    cargoBuildFlags = [
      "--package"
      "GitParser"
    ];
  };
in
{
  _module.args.gitParser = gitParser;

  home.packages = [
    pkgs.gh
    (lib.hiPrio ghWrapper)
  ];

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

  home.activation.gitLocalConfig = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
    if [ -e ${lib.escapeShellArg jsonPath} ]; then
      ${gitParser}/bin/GitParser git ${
        lib.escapeShellArgs [
          jsonPath
          localConfig
          localDir
          "${sshConfig}"
        ]
      }
    fi
  '';
}
