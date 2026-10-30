{ config, pkgs, ... }:

{
  hardware.graphics.enable = true;

  services.xserver = {
    videoDrivers = [ "modesetting" "nvidia" ];
    deviceSection = ''
      Option "Coolbits" "4"
    '';
  };

  hardware.nvidia = {
    modesetting.enable = true;
    powerManagement.enable = false;
    open = true;
    nvidiaSettings = true;
    package = config.boot.kernelPackages.nvidiaPackages.production;
    nvidiaPersistenced = true;
    prime = {
      intelBusId = "PCI:0@0:2:0";
      nvidiaBusId = "PCI:1@0:0:0";
    };
  };

  environment.systemPackages = [ pkgs.fw-fanctrl ];

  hardware.fw-fanctrl = {
    enable = true;
    config = {
      defaultStrategy = "school";
      strategyOnDischarging = "laziest";
      strategies = {
        "school" = {
          fanSpeedUpdateFrequency = 5;
          movingAverageInterval = 40;
          speedCurve = [
            { temp = 45; speed = 0; }
            { temp = 65; speed = 15; }
            { temp = 70; speed = 25; }
            { temp = 85; speed = 35; }
          ];
        };
        "lazy" = {
          fanSpeedUpdateFrequency = 5;
          movingAverageInterval = 30;
          speedCurve = [
            { temp = 0; speed = 15; }
            { temp = 50; speed = 15; }
            { temp = 65; speed = 25; }
            { temp = 70; speed = 35; }
            { temp = 75; speed = 50; }
            { temp = 85; speed = 100; }
          ];
        };
      };
    };
    disableBatteryTempCheck = false;
  };

  boot.extraModulePackages = [ config.boot.kernelPackages.msi-ec ];
  boot.kernelModules = [ "msi-ec" ];
}
