{
  description = "clipboard-substitutor";

  inputs.nixpkgs.url = "github:msfjarvis/nixpkgs/nixpkgs-unstable";

  inputs.systems.url = "github:msfjarvis/flake-systems";

  inputs.advisory-db.url = "github:rustsec/advisory-db";
  inputs.advisory-db.flake = false;

  inputs.crane.url = "github:ipetkov/crane";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.devshell.inputs.nixpkgs.follows = "nixpkgs";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.flake-utils.inputs.systems.follows = "systems";

  inputs.flake-compat.url = "git+https://git.lix.systems/lix-project/flake-compat";
  inputs.flake-compat.flake = false;

  outputs =
    {
      self,
      nixpkgs,
      advisory-db,
      crane,
      devshell,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlays.default ];
        };

        rustStable = (import fenix { inherit pkgs; }).fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-Qxt8XAuaUR2OMdKbN4u8dBJOhSHxS+uS06Wl9+flVEk=";
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          buildInputs = [ ];
          nativeBuildInputs =
            with pkgs;
            [
              xorg.libxcb
              python312
            ]
            ++ pkgs.lib.optionals stdenv.isDarwin [ pkgs.darwin.apple_sdk.frameworks.AppKit ];
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };
        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { doCheck = false; });

        clipboard-substitutor = craneLib.buildPackage (commonArgs // { doCheck = false; });
        clipboard-substitutor-clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );
        clipboard-substitutor-fmt = craneLib.cargoFmt (commonArgs // { });
        clipboard-substitutor-audit = craneLib.cargoAudit (commonArgs // { inherit advisory-db; });
        clipboard-substitutor-nextest = craneLib.cargoNextest (
          commonArgs
          // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          }
        );
      in
      {
        checks = {
          inherit
            clipboard-substitutor
            clipboard-substitutor-audit
            clipboard-substitutor-clippy
            clipboard-substitutor-fmt
            clipboard-substitutor-nextest
            ;
        };

        packages.default = clipboard-substitutor;

        apps.default = flake-utils.lib.mkApp { drv = clipboard-substitutor; };

        devShells.default = pkgs.devshell.mkShell {
          imports = [
            "${devshell}/extra/language/c.nix"
            "${devshell}/extra/language/rust.nix"
          ];

          env = [
            {
              name = "DEVSHELL_NO_MOTD";
              value = 1;
            }
          ];

          packages = with pkgs; [
            cargo-dist
            cargo-nextest
            cargo-release
            rustStable
          ];

          language.c.libraries = commonArgs.nativeBuildInputs;
          language.rust.enableDefaultToolchain = false;
        };
      }
    );
}
