{ pkgs, ... }:

let
  version = "2026.0.0.16";

  unpacked = pkgs.stdenvNoCC.mkDerivation {
    pname = "gamemaker-unwrapped";
    inherit version;

    src = pkgs.fetchurl {
      url = "https://gms.yoyogames.com/GameMaker-LTS2026-${version}.deb";
      sha256 = "1c8j2ymp50r5vj7akq6bpfi978my4iqrh1bhi6kvl5isd45b67aj";
    };

    nativeBuildInputs = [ pkgs.dpkg ];
    dontConfigure = true;
    dontBuild = true;
    dontFixup = true;

    unpackPhase = "dpkg-deb -x $src .";

    installPhase = ''
      mkdir -p $out
      cp -r opt/GameMaker-LTS2026 $out/opt
      cp -r usr/share $out/share
      rm -rf $out/opt/aarch64
      substituteInPlace $out/share/applications/GameMaker-LTS2026.desktop \
        --replace-fail "Exec=/opt/GameMaker-LTS2026/GameMaker" "Exec=gamemaker" \
        --replace-fail "Icon=/opt/GameMaker-LTS2026/GameMaker.png" "Icon=gamemaker"
      install -Dm644 $out/opt/GameMaker.png $out/share/icons/hicolor/256x256/apps/gamemaker.png
    '';
  };

  linuxdeploy = pkgs.stdenvNoCC.mkDerivation {
    pname = "linuxdeploy";
    version = "1-alpha-20250213-2";

    src = pkgs.fetchurl {
      url = "https://github.com/linuxdeploy/linuxdeploy/releases/download/1-alpha-20250213-2/linuxdeploy-x86_64.AppImage";
      sha256 = "0ajjnk89zbgjwvbkfxm7cm9hwr32yi80vhv7ks0izwrymdwg4j26";
    };

    dontUnpack = true;
    dontFixup = true;

    installPhase = ''
      install -Dm755 $src $out/bin/linuxdeploy
    '';
  };

  appimagetool = pkgs.stdenvNoCC.mkDerivation {
    pname = "appimagetool";
    version = "1.9.0";

    src = pkgs.fetchurl {
      url = "https://github.com/AppImage/appimagetool/releases/download/1.9.0/appimagetool-x86_64.AppImage";
      sha256 = "1lc3c38033392x5lnr1z4jmqx3fryfqczbv1bda6wzsc162xgza6";
    };

    dontUnpack = true;
    dontFixup = true;

    installPhase = ''
      install -Dm755 $src $out/bin/appimagetool
    '';
  };

  steamRuntime =pkgs.stdenvNoCC.mkDerivation {
    pname = "steam-runtime-sdk";
    version = "1.0.20261021.266054";

    src = pkgs.fetchurl {
      name = "steamrt-sdk-sysroot.tar.gz";
      url = "https://repo.steampowered.com/steamrt-images-scout/snapshots/1.0.20261021.266054/com.valvesoftware.SteamRuntime.Sdk-amd64,i386-scout-sysroot.tar.gz";
      sha256 = "c1a3ff99348908e82e845d83a5a689f3e9b95c815ea896df9923d068b4c5c5cb";
    };

    sourceRoot = ".";
    dontConfigure = true;
    dontBuild = true;
    dontFixup = true;

    unpackPhase = ''
      mkdir sysroot
      tar -xzf $src -C sysroot --no-same-owner --no-same-permissions --exclude='./dev/*'
    '';

    installPhase = ''
      mkdir -p $out
      cp -a sysroot/. $out/
      mkdir -p $out/tmp
    '';
  };
in
pkgs.buildFHSEnv {
  pname = "gamemaker";
  inherit version;

  targetPkgs =
    p: with p; [
      unpacked
      linuxdeploy
      appimagetool
      zlib
      bzip2.out
      libpng
      brotli
      openssl
      icu
      krb5
      libGL
      libGLU
      libx11
      libxcursor
      libxext
      libxi
      libxrandr
      libxinerama
      fontconfig
      freetype
      libxkbcommon
      gtk3
      glib
      alsa-lib
      libpulseaudio
      ffmpeg
      zip
      unzip
      zenity
      file
      desktop-file-utils
      e2fsprogs
      stdenv.cc.cc.lib
      libxcb
      libdrm
      gmp
      libgpg-error
      mesa
    ];

  profile = ''
    export LD_LIBRARY_PATH=${
      pkgs.lib.makeLibraryPath [
        pkgs.zlib
        pkgs.bzip2
        pkgs.libpng
        pkgs.brotli
      ]
    }:$LD_LIBRARY_PATH
    export PULSE_PROP="state.restore-props=false"
  '';

  runScript = "${unpacked}/opt/GameMaker";

  extraBuildCommands = ''
    mkdir -p $out/opt
    ln -s ${steamRuntime} $out/opt/steam-runtime
  '';

  extraInstallCommands = ''
    mkdir -p $out/share
    ln -s ${unpacked}/share/applications $out/share/applications
    ln -s ${unpacked}/share/icons $out/share/icons
    ln -s ${unpacked}/share/mime $out/share/mime
  '';
}
