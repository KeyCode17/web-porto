{
  description = "Daffa Karyudi - Portfolio (Rust + WASM / Dioxus)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";
  };

  nixConfig = {
    extra-substituters = [ "https://porto.cachix.org" ];
    extra-trusted-public-keys = [ "porto.cachix.org-1:+LLDAFA3ZLtOWuVBRn5V/hNjbDKW1ZYqhxQShPQBTPc=" ];
  };

  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = pkgs.lib.cleanSource ./.;
        cargoVendorDir = craneLib.vendorCargoDeps { inherit src; };
      in {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "web-porto";
          version = "0.1.0";
          inherit src;

          nativeBuildInputs = [
            rustToolchain
            pkgs.dioxus-cli
            pkgs.wasm-bindgen-cli
            pkgs.binaryen
          ];

          configurePhase = ''
            export HOME=$TMPDIR
            export CARGO_HOME=$TMPDIR/.cargo
            mkdir -p .cargo
            cp ${cargoVendorDir}/config.toml .cargo/config.toml
          '';

          buildPhase = ''
            dx bundle --package porto-app --release
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
