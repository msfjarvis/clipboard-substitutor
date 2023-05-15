{
  description = "clipboard-substitutor";

  inputs = {
    nixpkgs = {url = "github:NixOS/nixpkgs/nixpkgs-unstable";};

    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    flake-utils = {url = "github:numtide/flake-utils";};

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
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
    fenix,
    crane,
    flake-utils,
    advisory-db,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      rustStable = (import fenix {inherit pkgs;}).fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-eMJethw5ZLrJHmoN2/l0bIyQjoTX1NsvalWSscTixpI=";
      };

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

        CARGO_REGISTRIES_CRATES_IO_PROTOCOL = "sparse";
      };
    });
}
