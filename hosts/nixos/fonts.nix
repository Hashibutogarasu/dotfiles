{ pkgs, ... }:

{
  fonts = {
    packages = with pkgs; [
      noto-fonts
      noto-fonts-cjk-sans
      noto-fonts-cjk-serif
      noto-fonts-color-emoji
      jetbrains-mono
      mplus-outline-fonts.githubRelease
      ipafont
      ipaexfont
      kochi-substitute
      terminus_font
      cantarell-fonts
    ];
    fontDir.enable = true;
    fontconfig.defaultFonts = {
      serif = [
        "Noto Serif"
        "Noto Serif CJK JP"
      ];
      sansSerif = [
        "Noto Sans"
        "Noto Sans CJK JP"
      ];
      monospace = [
        "JetBrains Mono"
        "Noto Sans Mono CJK JP"
      ];
      emoji = [ "Noto Color Emoji" ];
    };
  };
}
