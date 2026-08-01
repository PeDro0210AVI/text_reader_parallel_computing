# Rust Base Dev Flake

A nix flake for setting up nix devshels and package with Naersk flake

## Requirements

- Nix flake enabled on any nix system or system with nix.

- direnv (needed if you want to setup auto startup for flake on entering by the shell).

## How to setup

### Vanilla

- init rust project

```fish
nix run nixpkgs#cargo init
nix run nixpkgs#cargo generate-lockfile
```

- Init flake template.

```fish
nix flake init -t github:PeDro0210/rust-base-dev-flake#default
```

- You are all set!

### With direnv

- Install direnv.

- Init flake template.

```fish
nix flake init -t github:PeDro0210/rust-base-dev-flake#default
```
