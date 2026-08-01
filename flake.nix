{
  description = "A nix flake for working with vanilla rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    crane.url = "github:ipetkov/crane";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    {
      crane,
      flake-parts,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {

      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];

      flake = {
        templates.default.path = ./.;
      };

      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        let

          craneLib = crane.mkLib pkgs;

          buildInputs = with pkgs; [
            expat
            fontconfig
            freetype
            freetype.dev
            libGL
            pkg-config
          ];

          nativeBuildInputs = with pkgs; [
            glfw
            cmake
            clang

            rustc
            rust-analyzer
            clippy
            rustfmt

          ];

          packages = with pkgs; [
            cargo
            bacon
            taplo # lsp for cargo.toml
            rustc
            rust-analyzer
            clippy
            rustfmt

          ];

        in

        {

          # declaring the build with the naerskLib flake
          packages.default = craneLib.buildPackage {
            inherit nativeBuildInputs buildInputs;
            src = ./.;
          };

          devShells.default = pkgs.mkShell {
            inherit nativeBuildInputs buildInputs packages;
          };

        };
    };

}
