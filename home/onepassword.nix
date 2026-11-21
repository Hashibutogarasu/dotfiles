{
  config,
  gitParser,
  lib,
  pkgs,
  ...
}:

let
  configFile = "${config.home.homeDirectory}/dotfiles/config/1Password.json";
in
{
  home.packages = [ pkgs._1password-cli ];

  home.activation.onePasswordKeys = lib.hm.dag.entryAfter [ "linkGeneration" ] ''
    if [ -e ${lib.escapeShellArg configFile} ]; then
      export PATH=${
        lib.makeBinPath [
          pkgs.openssh
          pkgs._1password-cli
          config.programs.gpg.package
        ]
      }:/run/wrappers/bin:$PATH
      if op whoami </dev/null >/dev/null 2>&1; then
        ${gitParser}/bin/GitParser keys ${lib.escapeShellArg configFile} </dev/null || true
      fi
    fi
  '';
}
