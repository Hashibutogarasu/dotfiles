{ pkgs, ... }:

{
  services.displayManager.cosmic-greeter.enable = true;
  services.desktopManager.cosmic.enable = true;

  environment.systemPackages = [
    (pkgs.runCommand "cosmic-term-noto-font" { } ''
      dir=$out/share/cosmic/com.system76.CosmicTerm/v1
      mkdir -p $dir
      echo -n '"JetBrains Mono"' > $dir/font_name
    '')
  ];
}
