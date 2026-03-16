{
  description = "Bootspec: an implementation of RFC-0125's data type and synthesis tooling";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1";
  };

  outputs =
    { self, ... }@inputs:
    let
      inherit (inputs.nixpkgs) lib;

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
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
      devShells = forEachSupportedSystem (
        { pkgs, system }:
        {
          default = pkgs.mkShell {
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
          };
        }
      );

      packages = forEachSupportedSystem (
        { pkgs, system }:
        {
          default = self.packages.${system}.bootspec;
          bootspec = pkgs.rustPlatform.buildRustPackage {
            pname = "bootspec";
            version = "unreleased";

            src = self;

            cargoLock.lockFile = ./Cargo.lock;
          };
        }
      );
    };
}
