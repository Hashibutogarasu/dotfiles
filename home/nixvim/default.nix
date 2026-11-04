{ ... }:

{
  programs.nixvim = (import ./settings.nix) // {
    enable = true;
    defaultEditor = true;
  };
}
