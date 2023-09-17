{
  description = "The deskodon app";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.05";
    unstable-nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
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

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, unstable-nixpkgs, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        unstable = import unstable-nixpkgs {
          inherit system;
        };

        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        unstableRustTarget = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "miri" ];
        });
        craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;

        tomlInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };

        nativeBuildInputs = with pkgs; [
          curl
          gcc
          openssl
          zlib
          pkg-config
        ];

        guiBuildInputs = with pkgs; [
          xorg.libX11
          xorg.libXcomposite
          xorg.libXcursor
          xorg.libXext
          xorg.libXfont
          xorg.libXfont2
          xorg.libXft
          xorg.libXi
          xorg.libXinerama
          xorg.libXmu
          xorg.libXpm
          xorg.libXpresent
          xorg.libXrandr
          xorg.libXrender
          xorg.libXt
          xorg.libXtst
          xorg.libXxf86misc
          xorg.libXxf86vm
          xorg.libxcb
          xorg.libxkbfile
          xorg.libxshmfence


          libGL
          pkg-config
          fontconfig
        ];

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

        deskodonFrontendArtifacts = craneLib.buildDepsOnly {
          pname = "deskodon-frontend";
          inherit src;

          doCheck = false;
          cargoExtraArgs = "--all-features -p deskodon-frontend --target wasm32-unknown-unknown";
        };

        deskodonArtifacts = craneLib.buildDepsOnly {
          inherit (tomlInfo) pname;
          inherit src;
          inherit nativeBuildInputs;
          buildInputs = guiBuildInputs;
        };

        deskodon-frontend = craneLib.buildPackage {
          inherit (tomlInfo) version;
          inherit src;
          inherit nativeBuildInputs;
          pname = "deskodon-frontend";

          # Override crane's use of --workspace, which tries to build everything.
          cargoCheckCommand = "cargo check --release";
          cargoBuildCommand = "cargo build --release";
          cargoTestCommand = "cargo test --profile release -p deskodon-frontend --lib";

          doCheck = false;
          cargoArtifacts = deskodonFrontendArtifacts;
          cargoExtraArgs = "--all-features -p deskodon-frontend --target wasm32-unknown-unknown";
        };

        deskodon = craneLib.buildPackage {
          inherit (tomlInfo) pname version;
          inherit src;
          inherit nativeBuildInputs;

          cargoArtifacts = deskodonArtifacts;
          cargoExtraArgs = "--all-features";
          buildInputs = guiBuildInputs;
        };
      in
      rec {
        checks = {
          inherit deskodon;
          inherit deskodon-frontend;

          deskodon-clippy = craneLib.cargoClippy {
            inherit (tomlInfo) pname;
            inherit src;
            inherit nativeBuildInputs;
            buildInputs = guiBuildInputs;

            cargoArtifacts = deskodonArtifacts;
            cargoClippyExtraArgs = "--tests --all-features -- --deny warnings";
          };

          deskodon-fmt = craneLib.cargoFmt {
            inherit (tomlInfo) pname;
            inherit src;
            inherit nativeBuildInputs;
            buildInputs = guiBuildInputs;
          };
        };

        packages = {
          inherit deskodon;
          inherit deskodon-frontend;
          default = packages.deskodon;
        };

        apps = {
          deskodon = flake-utils.lib.mkApp {
            name = "deskodon";
            drv = deskodon;
          };
          default = apps.deskodon;
        };

        devShells = {
          deskodon = pkgs.mkShell {
            buildInputs = guiBuildInputs;

            nativeBuildInputs = nativeBuildInputs ++ [
              rustTarget

              pkgs.wasm-bindgen-cli
              pkgs.cargo-msrv
              pkgs.cargo-deny
              pkgs.cargo-expand
              pkgs.cargo-bloat
              pkgs.cargo-fuzz
              pkgs.cargo-outdated
              pkgs.trunk

              unstable.gitlint
            ];
          };

          default = devShells.deskodon;
        };
      }
    );
}

