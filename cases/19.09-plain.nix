let
  src = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/19.09.tar.gz";
    sha256 = "sha256:0mhqhq21y5vrr1f30qd2bvydv4bbbslvyzclhw0kdxmkgg3z4c92";
  };
in
(import "${src}/nixos" {
  configuration = {
    imports = [
      "${src}/nixos/modules/virtualisation/qemu-vm.nix"
    ];
  };
}).config.system.build.toplevel
