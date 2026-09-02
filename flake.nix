{
  description = "pathtracer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src" "rustfmt" "clippy"];
      };
    in {
      formatter = pkgs.alejandra;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = [pkgs.pkg-config];

        buildInputs = [
          rustToolchain
          pkgs.openimagedenoise
          pkgs.openssl
        ];

        env = {
          OIDN_DIR = "${pkgs.openimagedenoise}/lib";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.openimagedenoise
            pkgs.stdenv.cc.cc.lib
          ];
        };
      };
    });
}
