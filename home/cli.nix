{ config, lib, pkgs, ... }:

let
  ghWrapper = pkgs.writeShellScriptBin "gh" ''
    if [ -z "''${GH_CONFIG_DIR:-}" ]; then
      helper=$(git config --get-all credential.https://github.com.helper 2>/dev/null | tail -n1)
      if [[ $helper =~ GH_CONFIG_DIR=\'([^\']*)\' ]]; then
        export GH_CONFIG_DIR="''${BASH_REMATCH[1]}"
      fi
    fi
    exec ${pkgs.gh}/bin/gh "$@"
  '';
in
{
  home.packages = with pkgs; [
    gh
    (lib.hiPrio ghWrapper)
    just
    screenfetch
    unzip
    usbutils
    lm_sensors
    nvtopPackages.nvidia
  ];

  programs.gpg.enable = true;

  services.gpg-agent = {
    enable = true;
    enableSshSupport = true;
    pinentry.package = pkgs.pinentry-gnome3;
  };

  systemd.user.sessionVariables.SSH_AUTH_SOCK = "\${XDG_RUNTIME_DIR}/gnupg/S.gpg-agent.ssh";
}
