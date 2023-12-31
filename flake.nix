{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/23.11";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, fenix, flake-utils, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (_: super: let pkgs = fenix.inputs.nixpkgs.legacyPackages.${super.system}; in fenix.overlays.default pkgs pkgs)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        extism-cli = (import ./nix/extism-cli.nix {
          buildGoModule = pkgs.buildGoModule;
          fetchFromGitHub = pkgs.fetchFromGitHub;
        });
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            nil
            openssl
            glib
            pkg-config
            rustup
            cargo
            gcc
            extism-cli
          ];
        };
      }
    );
}
