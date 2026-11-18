{ users, ... }:

{
  programs._1password.enable = true;
  programs._1password-gui = {
    enable = true;
    polkitPolicyOwners = [ users.crow.name ];
  };

  environment.etc."xdg/autostart/1password.desktop".text = ''
    [Desktop Entry]
    Type=Application
    Name=1Password
    Hidden=true
  '';
}
