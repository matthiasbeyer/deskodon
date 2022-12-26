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

        cargo-tauri = unstable.callPackage ./nix/cargo-tauri.nix {};

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
          pkgconfig
          which
          zlib

          freetype
          expat
          protobuf
        ];

        guiBuildInputs = (with pkgs; [
          alejandra
          appimagekit
          atk
          cairo
          dbus.lib
          dbus
          dprint
          gdk-pixbuf
          glib.out
          gtk3
          harfbuzz
          libsoup
          nodejs-16_x
          openssl.out
          pango
          pkg-config
          treefmt
          webkitgtk
          zlib
          pkg-config
          gobject-introspection
          glib-networking
        ]) ++ (with pkgs.xorg; [
          libX11
          libXcomposite
          libXcursor
          libXext
          libXfont
          libXfont2
          libXft
          libXi
          libXinerama
          libXmu
          libXpm
          libXpresent
          libXrandr
          libXrender
          libXt
          libXtst
          libXxf86misc
          libXxf86vm
          libxcb
          libxkbfile
          libxshmfence

          pkgs.libGL
          pkgs.pkgconfig
        ]);

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
            LIBCLANG_PATH   = "${pkgs.llvmPackages.libclang}/lib";
            PROTOC          = "${pkgs.protobuf}/bin/protoc";
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath guiBuildInputs;

            XDG_DATA_DIRS = let
              base = pkgs.lib.concatMapStringsSep ":" (x: "${x}/share") [
                pkgs.gnome.adwaita-icon-theme
                pkgs.shared-mime-info
              ];

              gsettings_schema = pkgs.lib.concatMapStringsSep ":" (x: "${x}/share/gsettings-schemas/${x.name}") [
                pkgs.glib
                pkgs.gsettings-desktop-schemas
                pkgs.gtk3
              ];
            in "${base}:${gsettings_schema}";

            GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/";

            buildInputs = guiBuildInputs;

            nativeBuildInputs = nativeBuildInputs ++ [
              rustTarget
              cargo-tauri

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

