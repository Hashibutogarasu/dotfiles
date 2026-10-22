{
  config,
  lib,
  pkgs,
  ...
}:

let
  home = config.home.homeDirectory;
  jsonPath = "${home}/.config/home-manager/config/git.json";
  localConfig = "${home}/.config/git/local.conf";
  localDir = "${home}/.config/git/local.d";
  jq = "${pkgs.jq}/bin/jq";
  git = "${pkgs.git}/bin/git";
in
{
  programs.git = {
    enable = true;

    includes = [
      {
        path = localConfig;
      }
    ];
  };

  home.activation.gitLocalConfig = lib.hm.dag.entryAfter [ "writeBoundary" ] ''
    set -eu
    json=${lib.escapeShellArg jsonPath}
    main=${lib.escapeShellArg localConfig}
    local_dir=${lib.escapeShellArg localDir}
    mkdir -p "$(dirname "$main")" "$local_dir"
    rm -f "$local_dir"/*.conf
    : > "$main"
    if [ -f "$json" ]; then
      name=$(${jq} -r '.user.name // empty' "$json")
      email=$(${jq} -r '.user.email // empty' "$json")
      signing_key=$(${jq} -r --arg home "$HOME" '.user.signingKey // empty | sub("~"; $home)' "$json")
      commit_sign=$(${jq} -r 'if .signing.commit == null then empty else (.signing.commit | tostring) end' "$json")
      gpg_format=$(${jq} -r '.signing.format // empty' "$json")
      if [ -n "$name" ]; then ${git} config --file "$main" user.name "$name"; fi
      if [ -n "$email" ]; then ${git} config --file "$main" user.email "$email"; fi
      if [ -n "$signing_key" ]; then ${git} config --file "$main" user.signingKey "$signing_key"; fi
      if [ -n "$commit_sign" ]; then ${git} config --file "$main" commit.gpgsign "$commit_sign"; fi
      if [ -n "$gpg_format" ]; then ${git} config --file "$main" gpg.format "$gpg_format"; fi
      i=0
      ${jq} -c '.directories // [] | .[]' "$json" | while IFS= read -r entry; do
        dir_path=$(printf '%s' "$entry" | ${jq} -r --arg home "$HOME" '(.path // "") | sub("^~"; $home)')
        include_file="$local_dir/$i.conf"
        : > "$include_file"
        dir_name=$(printf '%s' "$entry" | ${jq} -r '.user.name // empty')
        dir_email=$(printf '%s' "$entry" | ${jq} -r '.user.email // empty')
        dir_signing_key=$(printf '%s' "$entry" | ${jq} -r --arg home "$HOME" '.user.signingKey // empty | sub("~"; $home)')
        dir_ssh_command=$(printf '%s' "$entry" | ${jq} -r --arg home "$HOME" '.sshCommand // empty | gsub("~"; $home)')
        if [ -n "$dir_name" ]; then ${git} config --file "$include_file" user.name "$dir_name"; fi
        if [ -n "$dir_email" ]; then ${git} config --file "$include_file" user.email "$dir_email"; fi
        if [ -n "$dir_signing_key" ]; then ${git} config --file "$include_file" user.signingKey "$dir_signing_key"; fi
        if [ -n "$dir_ssh_command" ]; then ${git} config --file "$include_file" core.sshCommand "$dir_ssh_command"; fi
        case "$dir_path" in
          */) ;;
          *) dir_path="$dir_path/" ;;
        esac
        ${git} config --file "$main" "includeIf.gitdir:$dir_path.path" "$include_file"
        i=$((i + 1))
      done
    fi
  '';
}
