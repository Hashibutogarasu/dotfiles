{ config, lib, pkgs, ... }:

let
  configFile = "${config.home.homeDirectory}/dotfiles/config/1Password.json";

  exportSshKeys = pkgs.writeShellApplication {
    name = "export-ssh-keys";
    runtimeInputs = [
      pkgs.coreutils
      pkgs.jq
      pkgs.openssh
      config.programs.gpg.package
    ];
    text = ''
      export PATH="/run/wrappers/bin:$PATH"
      SSH_AUTH_SOCK=$(gpgconf --list-dirs agent-ssh-socket)
      export SSH_AUTH_SOCK
      gpgconf --launch gpg-agent
      gpg-connect-agent updatestartuptty /bye >/dev/null
      loaded=$(ssh-add -L 2>/dev/null | cut -d' ' -f1,2 || true)

      config_file="$1"
      if [ ! -f "$config_file" ]; then
        echo "1Password: config not found: $config_file" >&2
        exit 0
      fi

      items=$(op item list --categories "SSH Key" --format json)

      jq -c '.keys[]' "$config_file" | while read -r entry; do
        public_key=$(jq -r '.publicKey' <<<"$entry" | cut -d' ' -f1,2)
        dest=$(jq -r '.path' <<<"$entry")
        dest="''${dest/#\~/$HOME}"
        dest="''${dest%.pub}.pub"

        found=""
        while read -r item; do
          id=$(jq -r '.id' <<<"$item")
          vault=$(jq -r '.vault.id' <<<"$item")
          item_key=$(op item get "$id" --fields "public key" --format json | jq -r '.value' | cut -d' ' -f1,2)
          if [ "$item_key" = "$public_key" ]; then
            found="op://$vault/$id/private_key?ssh-format=openssh"
            break
          fi
        done < <(jq -c '.[]' <<<"$items")

        if [ -z "$found" ]; then
          echo "1Password: no SSH key matches the public key for $dest" >&2
          continue
        fi

        mkdir -p "$(dirname "$dest")"
        tmp=$(mktemp "$dest.XXXXXX")
        printf '%s\n' "$public_key" >"$tmp"
        chmod 644 "$tmp"
        mv -f "$tmp" "$dest"

        if ! grep -qxF "$public_key" <<<"$loaded"; then
          op read "$found" | ssh-add -
        fi
      done
    '';
  };
in
{
  home.activation.onePasswordSshKeys = lib.hm.dag.entryAfter [ "linkGeneration" ] ''
    ${lib.getExe exportSshKeys} ${lib.escapeShellArg configFile} \
      || echo "1Password: SSH key export failed (is 1Password unlocked?)" >&2
  '';
}
