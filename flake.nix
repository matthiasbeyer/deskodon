{
  description = "The deskodon app";
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:matthiasbeyer/crane/0.8.0-master-fix-use-unsafe";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        tauriBuildInputs = with pkgs; [
          appimagekit
          cargo-tauri
          cmake
          dbus
          glib
          gtk3
          libayatana-appindicator-gtk3
          librsvg
          libsoup
          openssl
          pkg-config
          webkitgtk
          libsoup
        ];

        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;

        tomlInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
        inherit (tomlInfo) version;
        pname = "deskodon";

        src =
          let
            markdownFilter = path: _type: pkgs.lib.hasSuffix ".md" path;
            filterPath = path: type: builtins.any (f: f path type) [
              markdownFilter
              craneLib.filterCargoSources
              pkgs.lib.cleanSourceFilter
            ];
          in
          pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = filterPath;
          };

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src pname;

          buildInputs = tauriBuildInputs ++ [];
        };

        elmArtifacts = import ./nix/elm2nix.nix { inherit pkgs; };

        deskodon = craneLib.buildPackage {
          inherit cargoArtifacts src pname version;

          cargoExtraArgs = "--all-features";

          preBuild = ''
            mkdir ./ui-out
            cp -rv ${elmArtifacts}/* ./ui-out/
            cp ${src}/ui/index.html ./ui-out/index.html
          '';

          buildInputs = tauriBuildInputs ++ [
            elmArtifacts
          ];
        };

        cargo-check-everything = pkgs.writeScriptBin "cargo-check-everything" ''
          #!${pkgs.runtimeShell}
          ${rustTarget}/bin/cargo check --all --tests --examples --benches
        '';

      in
      rec {
        checks = {
          inherit deskodon;
          inherit cargo-check-everything;
        };

        packages.deskodon = deskodon;
        packages.default = packages.deskodon;

        apps.deskodon = flake-utils.lib.mkApp {
          name = "deskodon";
          drv = deskodon;
        };
        apps.default = apps.deskodon;

        devShells.default = devShells.deskodon;
        devShells.deskodon = pkgs.mkShell {
          buildInputs = with pkgs; [
            appimagekit
            dbus
            glib
            gtk3
            libayatana-appindicator-gtk3
            librsvg
            libsoup
            openssl
            webkitgtk
          ];

          nativeBuildInputs = with pkgs; [
            rustTarget

            cargo-check-everything

            cargo-tauri
            cargo-deny
            gitlint
            pkg-config
            cmake
            elmPackages.elm
          ];
        };
      }
    );
}
