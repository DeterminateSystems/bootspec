let
  src = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/18.03.tar.gz";
    sha256 = "sha256:0hk4y2vkgm1qadpsm4b0q1vxq889jhxzjx3ragybrlwwg54mzp4f";
  };
in
(import "${src}/nixos" {
  configuration = {
    imports = [
      "${src}/nixos/modules/virtualisation/qemu-vm.nix"
    ];
  };
}).config.system.build.toplevel
