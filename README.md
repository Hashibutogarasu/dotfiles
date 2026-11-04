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

`git.json` contains personal information and is excluded from the repository.
Place the template and edit it:

```sh
cp ~/dotfiles/home/git/git.example.json ~/.config/git/git.json
```

Entries: `use1Password` (SSH commit signing and the `gh` credential helper via
the 1Password CLI plugin), `user` (global name, email and `signingKey`, a `.pub`
path whose private key is held by gpg-agent), `directories` (per-directory user
switching and SSH settings).

SSH keys listed in `config/1Password.json` are exported from 1Password on every
switch: the public key is written to the `.pub` path and the private key is
imported into gpg-agent without being written to `~/.ssh`.

Every rebuild/switch runs GitParser, which regenerates `~/.config/git/local.conf`
and `local.d/*`. Every `core.sshCommand` passes `-F` with an SSH config in the
Nix store, so git never reads `~/.ssh/config`.
If the file is missing, only the signing configuration is skipped — the switch
itself still succeeds.

### 3. Apply the system and home environment

```sh
sudo nixos-rebuild switch --flake ~/dotfiles#nixos
```

This applies the system and the `crow` Home Manager environment at once.

### 4. (optional) Switch Home Manager independently

```sh
home-manager switch --flake ~/dotfiles#crow
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
`~/.config/git/git.json` and run a switch — GitParser regenerates the affected
config files.

If something breaks, select the previous generation from the systemd-boot menu
or run `sudo nixos-rebuild switch --rollback`.

---

## Secrets

`home/git/git.json` is excluded via `.gitignore`; only `git.example.json` is
tracked in the repository. The live file lives outside the repository at
`~/.config/git/git.json`.
