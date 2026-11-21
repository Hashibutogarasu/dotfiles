{ config, lib, pkgs, ... }:

let
  sshUtils = pkgs.callPackage ../ssh/utils/package.nix { };
  home = config.home.homeDirectory;
  jsonPath = "${home}/dotfiles/config/ssh.json";
  hostsConfig = "${home}/.ssh/hosts.conf";
  renderHosts = pkgs.writeText "ssh-hosts.jq" ''
    def keywords: {
      hostname: "Hostname",
      user: "User",
      port: "Port",
      identityFile: "IdentityFile",
      identitiesOnly: "IdentitiesOnly",
      identityAgent: "IdentityAgent",
      proxyCommand: "ProxyCommand"
    };
    def render: if type == "boolean" then (if . then "yes" else "no" end) else tostring end;
    .hosts | to_entries[] |
      "Host \(.key)",
      (.value | to_entries[] |
        "  \(keywords[.key] // error("unknown ssh option: \(.key)")) \(.value | render)"),
      ""
  '';
in
{
  home.packages = [ sshUtils ];

  programs.ssh = {
    enable = true;
    enableDefaultConfig = false;
    includes = [
      "${config.home.homeDirectory}/.config/git/ssh.conf"
      hostsConfig
    ];
  };

  home.activation.sshHosts = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
    tmp=$(mktemp)
    ${pkgs.jq}/bin/jq -r -f ${renderHosts} ${lib.escapeShellArg jsonPath} > "$tmp" \
      && install -m 600 "$tmp" ${lib.escapeShellArg hostsConfig}
    rm -f "$tmp"
  '';

  home.activation.sshKeyGeneration = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
    if [ ! -e "${config.home.homeDirectory}/.ssh/generated/system_key" ]; then
      ${sshUtils}/bin/ssh-utils generate --output-dir "${config.home.homeDirectory}/.ssh/generated"
    fi
  '';
}
