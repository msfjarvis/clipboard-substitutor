{
  description = "clipboard-substitutor";

  inputs = {
    nixpkgs = {url = "github:NixOS/nixpkgs/nixpkgs-unstable";};

    flake-utils = {url = "github:numtide/flake-utils";};

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    advisory-db,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustStable =
        pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
      commonArgs = {
        src = craneLib.cleanCargoSource ./.;
        buildInputs = with pkgs;
          [xorg.libxcb python39]
          ++ pkgs.lib.optionals stdenv.isDarwin
          [pkgs.darwin.apple_sdk.frameworks.AppKit];
        nativeBuildInputs = [];
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      };
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {doCheck = false;});

      clipboard-substitutor = craneLib.buildPackage (commonArgs // {doCheck = false;});
      clipboard-substitutor-clippy = craneLib.cargoClippy (commonArgs
        // {
          inherit cargoArtifacts;
        });
      clipboard-substitutor-fmt = craneLib.cargoFmt (commonArgs // {});
      clipboard-substitutor-audit = craneLib.cargoAudit (commonArgs // {inherit advisory-db;});
      clipboard-substitutor-nextest = craneLib.cargoNextest (commonArgs
        // {
          inherit cargoArtifacts;
          src = ./.;
          partitions = 1;
          partitionType = "count";
        });
    in {
      checks = {
        inherit clipboard-substitutor clipboard-substitutor-audit clipboard-substitutor-clippy clipboard-substitutor-fmt clipboard-substitutor-nextest;
      };

      packages.default = clipboard-substitutor;

      apps.default = flake-utils.lib.mkApp {drv = clipboard-substitutor;};

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        nativeBuildInputs = with pkgs; [
          cargo-nextest
          cargo-release
          rustStable
        ];
      };
    });
}
