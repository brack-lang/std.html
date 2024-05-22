{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
        target = "wasm32-unknown-unknown";
        toolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [target];
        };
        rustPlatform = pkgs.makeRustPlatform {
          rustc = toolchain;
          cargo = toolchain;
        };
        extism-cli = import ./nix/extism-cli.nix {
          buildGoModule = pkgs.buildGoModule;
          fetchFromGitHub = pkgs.fetchFromGitHub;
        };
      in {
        packages.default = let
          name = "std-html";
          wasmPackage = rustPlatform.buildRustPackage {
            inherit name;
            src = ./.;
            copyLibs = true;
            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "brack-sdk-rs-0.1.0" = "sha256-brLwtHdMiJytb/yzA857c508ZnA6OcKhHfGA6sxNITE=";
              };
            };
            CARGO_BUILD_TARGET = target;
            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "${pkgs.llvmPackages.lld}/bin/lld";
            postBuild = ''
              for file in $(find $out -name "*.wasm"); do
                base=$(basename $file .wasm)
                renamed=$(echo $base | sed 's/_/./g').wasm
                mv $file $(dirname $file)/$renamed
              done
            '';
            buildPhase = ''
              cargo build --release -p ${name} --target=${target}
            '';
            installPhase = ''
              mkdir -p $out/lib
              cp target/${target}/release/*.wasm $out/lib/
            '';
          };
        in
          pkgs.stdenv.mkDerivation {
            name = "brack-std-html";
            buildInputs = [pkgs.rename];
            inherit (wasmPackage) src;
            installPhase = ''
              mkdir -p $out
              cp -r ${wasmPackage}/lib/* $out/
              for file in $out/*_*.wasm; do
                mv "$file" "$(echo $file | sed 's/_/./g')"
              done
            '';
          };
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            toolchain
            rust-analyzer
            nil
            alejandra
            extism-cli
          ];
        };
      }
    );
}
