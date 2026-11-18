{ pkgs, ... }:

let
  gtkSettings = ''
    [Settings]
    gtk-application-prefer-dark-theme=true
    gtk-theme-name=Breeze-Dark
    gtk-font-name=Noto Sans CJK JP 10
  '';
in
{
  qt = {
    enable = true;
    platformTheme = "kde";
    style = "breeze";
  };

  environment.sessionVariables.GTK_THEME = "Breeze-Dark";

  environment.systemPackages = with pkgs; [
    kdePackages.breeze
    kdePackages.breeze-gtk
    kdePackages.plasma-integration
  ];

  environment.etc."xdg/gtk-3.0/settings.ini".text = gtkSettings;
  environment.etc."xdg/gtk-4.0/settings.ini".text = gtkSettings;

  programs.dconf.profiles.user.databases = [
    {
      settings."org/gnome/desktop/interface" = {
        color-scheme = "prefer-dark";
        gtk-theme = "Breeze-Dark";
        font-name = "Noto Sans CJK JP 10";
        document-font-name = "Noto Sans CJK JP 10";
        monospace-font-name = "JetBrains Mono 10";
      };
    }
  ];
}
