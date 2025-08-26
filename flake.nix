{
  description = "chos";
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
    fenix = {
      url = "github:nix-community/fenix/main";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils/main";
    };
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        limine = pkgs.limine.override {
          enableAll = true;
          buildCDs = true;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo-about
            fenix.packages.${system}.complete.toolchain
            pkgs.qemu
            limine
            pkgs.gum
            pkgs.cz-cli
            (import nixpkgs {
              localSystem = system;
              targetSystem = {
                config = "riscv64-unknown-none-elf";
                gcc.arch = "rv64gc";
                gcc.abi = "lp64d";
              };
            }).gdb
          ];
          shellHook = ''
            export LIMINE_PATH=${limine}/share/limine
          '';
        };
      }
    );
}
