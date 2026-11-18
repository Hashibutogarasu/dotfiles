{
  pkgs,
  kwi,
  ghostty,
  ...
}:

let
  ghosttyLib = ghostty.packages.${pkgs.stdenv.hostPlatform.system}.libghostty-vt;

  onlyShowIn = import ../nixos/de/only-show-in.nix { inherit pkgs; };

  kwiPackage = onlyShowIn "KDE" (
    pkgs.stdenv.mkDerivation {
      pname = "kwi";
      version = "0.1.0-nightly";

      src = kwi;

      nativeBuildInputs = with pkgs; [
        cmake
        ninja
        pkg-config
        qt6.wrapQtAppsHook
      ];

      buildInputs = with pkgs; [
        qt6.qtbase
        qt6.qtdeclarative
        qt6.qtshadertools
        qt6.qtsvg

        kdePackages.extra-cmake-modules
        kdePackages.kcoreaddons
        kdePackages.kconfig
        kdePackages.kdbusaddons
        kdePackages.kguiaddons
        kdePackages.ki18n
        kdePackages.kirigami
        kdePackages.kpty
        kdePackages.kio
        kdePackages.knotifications
        kdePackages.kwindowsystem

        kdePackages.kirigami-addons
        kdePackages.kirigami-app-components

        ghosttyLib
        ghosttyLib.dev
      ];

      cmakeFlags = [
        "-DCMAKE_BUILD_TYPE=Release"
        "-DBUILD_TESTING=OFF"
        "-DBUILD_KPART=OFF"
      ];

      meta = {
        description = "Modern and fast terminal for KDE Plasma";
        homepage = "https://github.com/KDE/kwi";
        license = pkgs.lib.licenses.gpl3Plus;
        mainProgram = "kwi";
        platforms = [
          "x86_64-linux"
          "aarch64-linux"
        ];
      };
    }
  );
in
{
  environment.systemPackages = [ kwiPackage ];

  xdg.terminal-exec = {
    enable = true;
    settings.KDE = [ "org.kde.kwi.desktop" ];
  };
}
