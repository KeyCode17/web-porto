{
  description = "Rust + WASM Brutalist Portfolio";

  inputs = {
    clan-core.url = "https://git.clan.lol/clan/clan-core/archive/main.tar.gz";
    nixpkgs.follows = "clan-core/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, clan-core, nixpkgs, flake-utils, rust-overlay }:
    let
      clan = clan-core.lib.clan {
        inherit self;
        imports = [ ./clan.nix ];
      };
    in
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
            clan-core.packages.${system}.clan-cli
          ];
        };
      }
    ) // {
      inherit (clan.config) nixosConfigurations clanInternals;
    };
}
