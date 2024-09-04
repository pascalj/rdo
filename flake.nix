{
  description = "rdo-cli: a TUI radio client";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
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
              pkg-config
              gdb
              rust-bin.stable.latest.default
            ];

            shellHook = "";
          };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
