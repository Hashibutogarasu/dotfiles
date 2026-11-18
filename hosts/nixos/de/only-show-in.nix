{ pkgs }:

desktop: pkg:

pkgs.symlinkJoin {
  name = "${pkg.name}-only-${desktop}";
  paths = [ pkg ];
  inherit (pkg) meta;
  postBuild = ''
    if [ -d "$out/share/applications" ]; then
      for f in "$out"/share/applications/*.desktop; do
        cp --remove-destination "$(readlink -f "$f")" "$f"
        chmod u+w "$f"
        sed -i \
          -e '/^OnlyShowIn=/d' \
          -e '/^NotShowIn=/d' \
          -e '/^\[Desktop Entry\]/a OnlyShowIn=${desktop};' \
          "$f"
      done
    fi
  '';
}
