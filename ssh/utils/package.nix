{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "ssh-utils";
  version = "0.1.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "SSH key generation and client setup guide utility";
    mainProgram = "ssh-utils";
  };
}
