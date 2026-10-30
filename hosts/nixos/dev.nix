{ pkgs, ... }:

{
  programs.java = {
    enable = true;
    package = pkgs.openjdk25;
  };

  environment.systemPackages = [ pkgs.gcc ];
}
