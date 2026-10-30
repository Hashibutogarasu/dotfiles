{
  lib,
  rustPlatform,
  installShellFiles,
}:

rustPlatform.buildRustPackage {
  pname = "ssh-utils";
  version = "0.1.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ installShellFiles ];

  postInstall = ''
    installShellCompletion --cmd ssh-utils \
      --bash <($out/bin/ssh-utils completions bash) \
      --fish <($out/bin/ssh-utils completions fish) \
      --zsh <($out/bin/ssh-utils completions zsh)
  '';

  meta = {
    description = "SSH key generation and client setup guide utility";
    mainProgram = "ssh-utils";
  };
}
