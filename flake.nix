{
  description = "rdo: a TUI internet radio client";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (final: prev: { rdo = prev.callPackage ./nix/package.nix { }; })
            (import rust-overlay)
          ];
        };
      in
      {
        packages.default = pkgs.rdo;

        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              mpv
              rust-bin.stable.latest.default
            ];
          };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
