{
  description = "NixOS and Home Manager configuration";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixvim = {
      url = "github:nix-community/nixvim";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, home-manager, nixvim, ... }:
    let
      system = "x86_64-linux";

      users = {
        crow = {
          name = "crow";
          description = "crow";
          homeDirectory = "/home/crow";
        };
      };
    in
    {
      packages.${system}.ssh-utils =
        nixpkgs.legacyPackages.${system}.callPackage ./ssh/utils/package.nix { };

      nixosConfigurations."nixos" = nixpkgs.lib.nixosSystem {
        inherit system;
        specialArgs = { inherit users nixvim; };
        modules = [
          ./hosts/nixos
          home-manager.nixosModules.home-manager
          {
            home-manager = {
              useGlobalPkgs = true;
              useUserPackages = true;
              backupFileExtension = "backup";
              sharedModules = [ nixvim.homeModules.nixvim ];
              extraSpecialArgs = { inherit users; };
              users.${users.crow.name} = import ./home;
            };
          }
        ];
      };

      homeConfigurations.${users.crow.name} = home-manager.lib.homeManagerConfiguration {
        pkgs = nixpkgs.legacyPackages.${system};
        extraSpecialArgs = { inherit home-manager users; };
        modules = [
          nixvim.homeModules.nixvim
          ./home
          ./home/standalone.nix
          { nixpkgs.config.allowUnfree = true; }
        ];
      };
    };
}
