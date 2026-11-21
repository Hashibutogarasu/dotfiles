{
  config,
  lib,
  pkgs,
  ...
}:

let
  jsonPath = "${config.home.homeDirectory}/dotfiles/config/ssh.json";
  keyFile = "${config.home.homeDirectory}/.ssh/github.pub";
  knownHostsFile = "${config.home.homeDirectory}/.ssh/github_known_hosts";
in
{
  home.activation.sshGithubKey = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
    if [ -e ${lib.escapeShellArg jsonPath} ]; then
      mkdir -p "$(dirname ${lib.escapeShellArg keyFile})"
      ${pkgs.jq}/bin/jq -r '.github.publicKey' ${lib.escapeShellArg jsonPath} > ${lib.escapeShellArg keyFile}
      ${pkgs.jq}/bin/jq -r '"github.com " + .github.hostKey' ${lib.escapeShellArg jsonPath} > ${lib.escapeShellArg knownHostsFile}
    fi
  '';

  programs.ssh = {
    enable = true;
    enableDefaultConfig = false;
    settings."github.com" = {
      HostName = "github.com";
      User = "git";
      IdentityFile = "~/.ssh/github.pub";
      IdentitiesOnly = "yes";
      IdentityAgent = "/run/user/%i/ssh-agent";
      UserKnownHostsFile = knownHostsFile;
      StrictHostKeyChecking = "yes";
    };
  };
}
