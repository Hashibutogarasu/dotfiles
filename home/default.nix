{ users, ... }:

{
  home.username = users.crow.name;
  home.homeDirectory = users.crow.homeDirectory;
  home.stateVersion = "26.05";

  imports = [
    ./applications.nix
    ./cli.nix
    ./develop.nix
    ./git
    ./nixvim
    ./ssh.nix
  ];

  programs.home-manager.enable = true;

  programs.bash = {
    enable = true;
    initExtra = ''
      export PATH="$HOME/.local/bin:$PATH"

      export PNPM_HOME="$HOME/.local/share/pnpm"
      case ":$PATH:" in
        ":$PNPM_HOME/bin:"*) ;;
        *) export PATH="$PNPM_HOME/bin:$PATH" ;;
      esac

      export NVM_DIR="$HOME/.nvm"
      [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
      [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
    '';
  };

  programs.direnv = {
    enable = true;
    enableBashIntegration = true;
    nix-direnv.enable = true;
  };
}
