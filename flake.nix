{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
      };
    };
    nix-filter = {
      url = "github:numtide/nix-filter";
    };
  };

  outputs = {...} @ inputs:
    with inputs;
      flake-parts.lib.mkFlake {inherit inputs;} {
        systems = ["x86_64-linux" "aarch64-linux"];
        perSystem = {system, ...}: let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [(import rust-overlay)];
          };

          rustToolchain = fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-yMuSb5eQPO/bHv+Bcf/US8LVMbf/G/0MSfiPwBhiPpk=";
          };

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        in {
          devShells = {
            default = craneLib.devShell {
              checks = self.checks.${system};
              packages = with pkgs; [
                rust-analyzer
                cmake
                postgresql
                postgresql.dev
                postgresql.lib
                diesel-cli
              ];
              RUST_SRC_PATH = "${craneLib.rustc}/lib/rustlib/src/rust/library";
              RUST_BACKTRACE = 1;
              shellHook = ''
                BONJOUR_SDK_HOME=$(pwd)/deps/bonjour-sdk-windows
                PQ_LIB_DIR=${pkgs.postgresql.lib}/lib
                echo "RUST_LOG=debug" > .env
                echo "DATABASE_URL=postgres://postgres:postgres@localhost/flo" >> .env
                echo "FLO_CONTROLLER_SECRET=1111" >> .env
                echo "FLO_NODE_SECRET=1111" >> .env
                echo "JWT_SECRET_BASE64=dGVzdHRlc3R0ZXN0dGVzdHRlc3R0ZXN0dGVzdHRlc3R0ZXN0dGVzdHRlc3Q=" >> .env
              '';
            };
          };
        };
      };
}
