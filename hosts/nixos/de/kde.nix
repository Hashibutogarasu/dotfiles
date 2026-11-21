{ lib, pkgs, ... }:

let
  apps = [
    "dolphin"
    "konsole"
    "kate"
    "kwrite"
    "okular"
    "gwenview"
    "ark"
    "spectacle"
    "kcalc"
    "systemsettings"
    "kinfocenter"
    "plasma-systemmonitor"
    "elisa"
    "khelpcenter"
  ];
in
{
  imports = [ ../../applications/kwi.nix ];

  nixpkgs.overlays = [
    (final: prev: {
      kdePackages =
        prev.kdePackages
        // lib.genAttrs (builtins.filter (name: prev.kdePackages ? ${name}) apps) (
          name: import ./only-show-in.nix { pkgs = prev; } "KDE" prev.kdePackages.${name}
        );
    })
  ];

  services.desktopManager.plasma6.enable = true;

  environment.etc."xdg/kdeglobals".text = ''
    [General]
    ColorScheme=BreezeDark
    TerminalApplication=kwi
    TerminalService=org.kde.kwi.desktop
    fixed=JetBrains Mono,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
    font=JetBrains Mono,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
    menuFont=JetBrains Mono,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
    toolBarFont=JetBrains Mono,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
    smallestReadableFont=JetBrains Mono,8,-1,5,400,0,0,0,0,0,0,0,0,0,0,1

    [KDE]
    LookAndFeelPackage=org.kde.breezedark.desktop
  '';

  environment.etc."xdg/konsolerc".text = ''
    [Desktop Entry]
    DefaultProfile=Noto.profile
  '';

  environment.systemPackages = [
    (pkgs.runCommand "konsole-noto-profile" { } ''
      mkdir -p $out/share/konsole
      cat > $out/share/konsole/Noto.profile <<'EOF'
      [Appearance]
      Font=JetBrains Mono,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1

      [General]
      Name=Noto
      Parent=FALLBACK/
      EOF
    '')
  ];
}
