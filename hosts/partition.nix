{ ... }:

{
  fileSystems."/mnt/windows" = {
    device = "/dev/disk/by-uuid/98A6F476A6F455EA";
    fsType = "ntfs3";
    options = [
      "noauto"
      "nofail"
      "x-systemd.automount"
      "x-systemd.idle-timeout=60"
      "uid=1000"
      "gid=100"
    ];
  };

  services.udev.extraRules = ''
    ENV{ID_FS_UUID}=="98A6F476A6F455EA", ENV{UDISKS_IGNORE}="1"
  '';
}
