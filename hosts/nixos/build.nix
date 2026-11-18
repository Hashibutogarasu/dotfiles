{ ... }:

{
  nix.settings = {
    max-jobs = 1;
    cores = 2;
  };

  nix.daemonCPUSchedPolicy = "idle";
  nix.daemonIOSchedClass = "idle";

  systemd.services.nix-daemon.serviceConfig = {
    MemoryHigh = "60%";
    MemoryMax = "75%";
    OOMScoreAdjust = 500;
  };

  zramSwap.enable = true;
  services.earlyoom.enable = true;
}
