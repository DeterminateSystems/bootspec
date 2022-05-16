let
  src = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/21.11.tar.gz";
    sha256 = "sha256:162dywda2dvfj1248afxc45kcrg83appjd0nmdb541hl7rnncf02";
  };
in
(import "${src}/nixos" {
  configuration = {
    imports = [
      "${src}/nixos/modules/virtualisation/qemu-vm.nix"
      ({ pkgs, ... }: {
        specialisation.example.configuration = {
          environment.systemPackages = [ pkgs.hello ];
        };
      })
    ];
  };
}).config.system.build.toplevel
