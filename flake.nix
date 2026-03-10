{
  description = "Daffa Karyudi - Portfolio (Rust + WASM / Dioxus)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "web-porto";
          version = "0.1.0";
          src = pkgs.lib.cleanSource ./.;

          nativeBuildInputs = [
            rustToolchain
            pkgs.dioxus-cli
            pkgs.wasm-bindgen-cli
            pkgs.binaryen
            pkgs.cacert
          ];

          buildPhase = ''
            export HOME=$TMPDIR
            export CARGO_HOME=$TMPDIR/.cargo
            dx bundle --package porto-app
          '';

          installPhase = ''
            mkdir -p $out
            cp -r target/dx/porto-app/release/web/public/* $out/
          '';
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.dioxus-cli
            pkgs.wasm-bindgen-cli
            pkgs.binaryen
            pkgs.cargo-watch
            pkgs.simple-http-server
          ];
        };
      }
    ) // {
      nixosModules.default = import ./nix/module.nix self;
    };
}
