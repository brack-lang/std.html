{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/23.11";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = { self, nixpkgs, fenix, naersk, flake-utils, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (_: super: let pkgs = fenix.inputs.nixpkgs.legacyPackages.${super.system}; in fenix.overlays.default pkgs pkgs)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        naersk' = pkgs.callPackage naersk {};
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
        packages.default =
          let
            pkgs = nixpkgs.legacyPackages.${system};
            target = "wasm32-unknown-unknown";
            toolchain = with fenix.packages.${system}; combine [
              minimal.cargo
              minimal.rustc
              targets.${target}.latest.rust-std
            ];
            wasmPackage = (naersk.lib.${system}.override {
              cargo = toolchain;
              rustc = toolchain;
            }).buildPackage {
              src = ./.;
              copyLibs = true;
              CARGO_BUILD_TARGET = target;
              CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "${pkgs.llvmPackages.lld}/bin/lld";
              postBuil = ''
                for file in $(find $out -name "*.wasm"); do
                  base=$(basename $file .wasm)
                  renamed=$(echo $base | sed 's/_/./g').wasm
                  mv $file $(dirname $file)/$renamed
                done
              '';
            };
          in pkgs.stdenv.mkDerivation {
            name = "brack-std-html";
            buildInputs = [ pkgs.rename ];
            
            inherit (wasmPackage) src;
            
            installPhase = ''
              mkdir -p $out
              cp -r ${wasmPackage}/lib/* $out/
              for file in $out/*_*.wasm; do
                mv "$file" "$(echo $file | sed 's/_/./g')"
              done
            '';
          };
      }
    );
}
