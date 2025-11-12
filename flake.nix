{
  description = "bootloader-experimentation";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs =
    { self, ... }@inputs:
    let
      inherit (inputs.nixpkgs) lib;

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forEachSupportedSystem =
        f:
        lib.genAttrs supportedSystems (
          system:
          f {
            inherit system;
            pkgs = import inputs.nixpkgs { inherit system; };
          }
        );
    in
    {
      devShell = forEachSupportedSystem (
        { system, pkgs }:
        pkgs.mkShell {
          name = "bootspec";

          packages = with pkgs; [
            cargo
            rustc
            clippy
            codespell
            nixpkgs-fmt
            rustfmt
            jsonschema # provides the jv tool
            json-schema-for-humans # provides the generate-schema-doc tool
          ];
        }
      );

      packages = forEachSupportedSystem (
        { system, pkgs }:
        {
          default = self.packages.${system}.bootspec;
          bootspec = pkgs.rustPlatform.buildRustPackage rec {
            pname = "bootspec";
            version = "unreleased";

            src = inputs.self;

            cargoLock.lockFile = ./Cargo.lock;
          };
        }
      );
    };
}
