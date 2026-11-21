# dotfiles

NixOS (hostname `nixos`) and Home Manager managed from a single flake.

```
dotfiles/
├── flake.nix          # nixosConfigurations."nixos" + homeConfigurations."crow"
├── hosts/nixos/       # NixOS system configuration
│   ├── hardware.nix   # partition UUIDs + initrd kernel modules (required to boot)
│   ├── core.nix       # boot / locale / user / nix settings
│   ├── graphics.nix   # NVIDIA Prime + msi-ec + fw-fanctrl
│   ├── desktop.nix    # X11 / sddm / Plasma6 / fcitx5-mozc
│   ├── fonts.nix      # Noto CJK and friends
│   ├── apps.nix       # steam / OBS / coolercontrol / git / nh
│   ├── services.nix   # openssh / fprintd / flatpak / pipewire / CUPS
│   ├── dev.nix        # openjdk / gcc
│   └── nix-ld.nix     # shared libraries for prebuilt binaries (nix-ld)
└── home/              # Home Manager configuration
    ├── default.nix    # user core + direnv
    ├── standalone.nix # only used by standalone `home-manager switch`
    ├── nvim/          # lazy.nvim based neovim configuration
    ├── git/           # git config + GitParser (Rust) config generator
    ├── cli.nix        # gh / just / gnupg and friends
    ├── applications.nix # GUI applications (1Password, zed, godot, ...)
    ├── develop.nix    # rustup / nixd / espflash and friends
    └── claude/        # ~/.claude/CLAUDE.md
```

`home-manager` is integrated as a NixOS module, so `nixos-rebuild switch` switches
the system and the `crow` home environment at the same time.

---

## Setup on a new machine

Prerequisites: NixOS installed, `nix-command` and `flakes` enabled,
hostname `nixos`. On a different host, regenerate `hosts/nixos/hardware.nix`
with `sudo nixos-generate-config` first.

### 1. Clone the repository

```sh
git clone git@github.com:Hashibutogarasu/nix-home.git ~/dotfiles
```

### 2. Prepare the git signing configuration

`config/git.json` contains personal information and is excluded from the repository.
Copy `config/git.example.json` to `config/git.json` and edit it. A switch fails
with an error if the file is missing or invalid.

Entries: `useGh` (the `gh` credential helper), `signing` (`commit` enables
commit signing, `format` selects the signature format), `user` (accounts keyed
by an identifier that must not contain whitespace, each with `name` — the git
user name — `email`, `signingKey` — a key ID, or an SSH public key such as
`ssh-ed25519 AAAA...` that is passed to git as `key::<key>` — `sshCommandArgs`
— an array of extra `ssh` arguments — and `useGh`), `defaultUser` (the account
used globally; none is applied if omitted), `directories` (per-directory `user`,
naming an account from `user`). Referencing an undefined account is an error.

`core.sshCommand` is generated from the account's `signingKey`: for an SSH public
key it is written to `~/.config/git/local.d/<user>.pub` and passed as
`-i <file> -o IdentitiesOnly=yes`, followed by `sshCommandArgs`.

Keys listed in `config/1Password.json` are imported on every switch. `keyId` is
either a GPG key ID or an SSH public key such as `ssh-ed25519 AAAA...`.

- GPG key ID: the 1Password item titled with that ID is used and its `private key`
  field is piped into `gpg --import`, replacing any existing key with the same ID.
  An optional `reference` (`op://...`) overrides the item lookup.
- SSH public key: the 1Password SSH Key item with that public key is found and its
  private key is piped into `ssh-add`, which hands it to gpg-agent.

The private key is never written to a file.

`config/ssh.json` defines the extra SSH hosts. Copy `config/ssh.example.json` to
`config/ssh.json` and edit it. Every switch renders `hosts` into `~/.ssh/hosts.conf`,
which `~/.ssh/config` includes. Each host takes `hostname`, `user`, `port`,
`identityFile`, `identitiesOnly`, `identityAgent` and `proxyCommand`; any other key is
an error. A switch fails if the file is missing. To use a key stored in 1Password, add
its public key to `config/1Password.json` so it is loaded into gpg-agent on every switch.

Every rebuild/switch runs GitParser, which regenerates `~/.config/git/local.conf`
and `local.d/*`. Every `core.sshCommand` passes `-F` with an SSH config in the
Nix store, so git never reads `~/.ssh/config`.
If the file is missing, GitParser exits with an error and no default values are
applied.

### 3. Apply the system and home environment

```sh
sudo nixos-rebuild switch --flake ~/dotfiles#nixos
```

This applies the system and the `crow` Home Manager environment at once.

### 4. (optional) Switch Home Manager independently

```sh
home-manager switch --flake ~/dotfiles#crow
```

To apply it with `sudo`:

```sh
sudo home-manager switch --flake ~/dotfiles#crow
```

The standalone configuration links `~/.config/home-manager` to `~/dotfiles`, so
after the first switch a bare `home-manager switch` uses this flake by default.

If existing files conflict, add `-b backup` (the configuration already sets
the `.backup` extension for moved files).

---

## Updating

```sh
cd ~/dotfiles
# update flake.lock to a newer revision
nix flake update

# edit, then apply
sudo nixos-rebuild switch --flake ~/dotfiles#nixos
```

Preview changes with `sudo nixos-rebuild dry-run --flake ~/dotfiles#nixos`.

To only change `git.json` (signing / account switching), edit
`~/dotfiles/config/git.json` and run a switch — GitParser regenerates the affected
config files.

If something breaks, select the previous generation from the systemd-boot menu
or run `sudo nixos-rebuild switch --rollback`.

---

## Secrets

`config/git.json` is excluded via `.gitignore`; only `config/git.example.json` is
tracked in the repository. `user.name` and `user.email` set the global identity,
and each `directories[].user` overrides them for repositories under that path.
