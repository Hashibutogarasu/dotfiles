{ pkgs, ... }:

{
  programs.nix-ld.enable = true;
  programs.nix-ld.libraries = with pkgs; [
    stdenv.cc.cc
    stdenv.cc.cc.lib
    zlib
    zstd
    xz
    bzip2
    curl
    openssl
    attr
    acl
    libssh
    libsodium
    libxml2
    util-linux
    udev
    dbus
    systemd

    mesa
    libglvnd
    libGL
    libGLU
    vulkan-loader
    libgbm
    libdrm
    libva
    libvdpau
    libxkbcommon
    fontconfig
    freetype
    pixman
    libpng
    libjpeg
    libtiff

    libX11
    libXcomposite
    libXcursor
    libXdamage
    libXext
    libXfixes
    libXft
    libXi
    libXinerama
    libXmu
    libXrandr
    libXrender
    libXScrnSaver
    libXt
    libXtst
    libXxf86vm
    libxcb
    libxcb-cursor
    xcbutil
    xcbutilimage
    xcbutilkeysyms
    xcbutilrenderutil
    xcbutilwm

    glib
    gtk2
    gtk3
    gdk-pixbuf
    pango
    cairo
    atk
    harfbuzz
    librsvg
    icu
    libnotify
    gsettings-desktop-schemas
    dbus-glib
    libayatana-appindicator
    nspr
    nss

    alsa-lib
    libpulseaudio
    pipewire
    flac
    libogg
    libvorbis
    libtheora
    libvpx

    cups
    libcap
    libgcrypt
    libgpg-error
    gmp
    libelf
    libidn
    tbb
    libxcrypt
    libxcrypt-legacy
    expat
    speex
    libsamplerate
    libmikmod
    libcaca
    libcanberra

    SDL
    SDL2
    SDL_image
    SDL2_image
    SDL_ttf
    SDL2_ttf
    SDL_mixer
    SDL2_mixer
    freeglut
    glew_1_10

    networkmanager
    coreutils
    pciutils
    zenity
    fuse
    e2fsprogs
    sane-backends
    pkcs11helper
    krb5

    (runCommand "librsvg-loader" { } ''
      mkdir -p $out/lib/gdk-pixbuf-2.0/2.10.0/loaders
      ln -s "${librsvg}/lib/gdk-pixbuf-2.0/2.10.0/loaders/libpixbufloader_svg.so" "$out/lib/libpixbufloader-svg.so"
    '')
  ];
}
