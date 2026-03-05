{
  description = "Rust + WASM Brutalist Portfolio";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.wasm-pack
            pkgs.wasm-bindgen-cli
            pkgs.binaryen
            pkgs.cargo-watch
            pkgs.simple-http-server
            pkgs.dioxus-cli
          ];
        };

        # NOTE: This derivation requires network access for cargo to fetch crates.
        # In a sandboxed Nix build, this will fail unless:
        #   1. You build with `--option sandbox false`, or
        #   2. You vendor dependencies with `cargo vendor`, or
        #   3. You replace this with a crane-based build (recommended for production).
        # A proper crane-based build should replace this derivation for CI/CD.
        packages.default = pkgs.stdenv.mkDerivation {
          name = "web-porto";
          src = pkgs.lib.cleanSource ./.;

          nativeBuildInputs = [
            rustToolchain
            pkgs.wasm-pack
            pkgs.wasm-bindgen-cli
            pkgs.binaryen
            pkgs.cacert
          ];

          buildPhase = ''
            # Set HOME for cargo
            export HOME=$TMPDIR
            export CARGO_HOME=$TMPDIR/.cargo

            # Build the main app crate to WASM
            wasm-pack build crates/app --target web --out-dir $TMPDIR/pkg

            # Optimize WASM binary size
            wasm-opt -Oz $TMPDIR/pkg/web_porto_app_bg.wasm -o $TMPDIR/pkg/web_porto_app_bg.wasm
          '';

          installPhase = ''
            mkdir -p $out

            # Copy WASM output
            cp -r $TMPDIR/pkg $out/pkg

            # Copy static assets
            cp -r static $out/static || true

            # Copy index.html
            cp index.html $out/index.html
          '';
        };
      }
    ) // {
      nixosModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.services.web-porto;
        in {
          options.services.web-porto = {
            enable = lib.mkEnableOption "web-porto portfolio site";
            domain = lib.mkOption {
              type = lib.types.str;
              default = "localhost";
              description = "Domain name for the portfolio";
            };
          };

          config = lib.mkIf cfg.enable {
            services.nginx = {
              enable = true;
              recommendedGzipSettings = true;
              recommendedOptimisation = true;
              recommendedProxySettings = true;

              virtualHosts."${cfg.domain}" = {
                root = "${self.packages.${pkgs.system}.default}";

                locations."/" = {
                  tryFiles = "$uri $uri/ /index.html";
                };

                locations."~* \\.wasm$" = {
                  extraConfig = ''
                    add_header Content-Type application/wasm;
                    add_header Cache-Control "public, max-age=31536000, immutable";
                  '';
                };

                locations."/static/" = {
                  extraConfig = ''
                    add_header Cache-Control "public, max-age=31536000, immutable";
                  '';
                };

                locations."/pkg/" = {
                  extraConfig = ''
                    add_header Cache-Control "public, max-age=31536000, immutable";
                  '';
                };
              };
            };

            networking.firewall.allowedTCPPorts = [ 80 443 ];
          };
        };
    };
}
