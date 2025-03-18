{
  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = {
      nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs =
    {
      self,
      nixpkgs,
      devenv,
      systems,
      fenix,
      ...
    }@inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
      musl-overlay = final: prev: {
        musl = prev.musl.overrideAttrs (old: {
          patches = (old.patches or []) ++ (prev.lib.optional (prev.stdenv.buildPlatform.isDarwin) (builtins.fetchurl {
              url = "https://github.com/timbertson/musl/compare/f314e133929b6379eccc632bef32eaebb66a7335...05b89f783fd1873ce9ec1127fa76d002921caa23.patch";
              sha256 = "1n17lawfpd551707nh3pr6ilyh0qh7rh0vdb522ijdygggh49rhd";
            })
          );
        });
      };
    in
    {
      packages = forEachSystem (system: let 
        pkgs = import nixpkgs { inherit system; overlays = [ musl-overlay ]; };
        lib = pkgs.lib;
        linuxPkgs = {
          "x86_64" = pkgs.pkgsCross.musl64.pkgsStatic;
          "aarch64" = pkgs.pkgsCross.aarch64-multiplatform-musl.pkgsStatic;
        };
        curlspeed-gen = linuxPkgs: let 
          target = (lib.systems.parse.tripleFromSystem linuxPkgs.stdenv.hostPlatform.parsed);
          toolchain = with fenix.packages.${system}; combine [
            stable.cargo
            stable.rustc
            targets.${target}.stable.rust-std
          ];
        in (linuxPkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        }).buildRustPackage {
          pname = "curlspeed";
          version = "0.1.0";

          src = with lib.fileset; toSource {
            root = ./.;
            fileset = unions [
              ./src
              ./Cargo.lock
              ./Cargo.toml
            ];
          };
          cargoLock.lockFile = ./Cargo.lock;
          logLevel = "info";
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ linuxPkgs.curl ];
        };
      in {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
        devenv-test = self.devShells.${system}.default.config.test;
        curlspeed = (lib.mapAttrs' (name: value: lib.nameValuePair ("${name}-linux") (curlspeed-gen linuxPkgs."${name}")) linuxPkgs);
      });

      devShells = forEachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              {
                # https://devenv.sh/reference/options/
                packages = [ pkgs.curl ];

                languages.rust = {
                  enable = true;
                  channel = "stable";
                };
              }
            ];
          };
        }
      );
    };
}
