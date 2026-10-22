{ config, lib, ... }:

let
  home = config.home.homeDirectory;
  gitConfigPath = "${home}/.config/home-manager/config/git.json";

  gitConfig =
    if builtins.pathExists gitConfigPath then
      builtins.fromJSON (builtins.readFile gitConfigPath)
    else
      { };

  signing = gitConfig.signing or { };

  expandHome = path: builtins.replaceStrings [ "~" ] [ home ] path;

  mkUser =
    user:
    {
      name = user.name;
      email = user.email;
    }
    // lib.optionalAttrs ((user.signingKey or null) != null) {
      signingKey = expandHome user.signingKey;
    };

  mkInclude = directory: {
    condition = "gitdir:${lib.removeSuffix "/" (expandHome directory.path)}/";
    contents =
      lib.optionalAttrs (directory ? user) {
        user = mkUser directory.user;
      }
      // lib.optionalAttrs (directory ? sshCommand) {
        core.sshCommand = expandHome directory.sshCommand;
      }
      // lib.optionalAttrs (signing ? commit) {
        commit.gpgsign = signing.commit;
      };
  };
in
{
  programs.git = {
    enable = true;

    settings =
      lib.optionalAttrs (gitConfig ? user) {
        user = mkUser gitConfig.user;
      }
      // lib.optionalAttrs (signing ? commit) {
        commit.gpgsign = signing.commit;
      }
      // lib.optionalAttrs (signing ? format) {
        gpg.format = signing.format;
      };

    includes = map mkInclude (gitConfig.directories or [ ]);
  };
}
