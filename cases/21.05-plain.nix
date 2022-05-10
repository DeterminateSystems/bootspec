let
  src = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/21.05.tar.gz";
    sha256 = "sha256:1ckzhh24mgz6jd1xhfgx0i9mijk6xjqxwsshnvq789xsavrmsc36";
  };
in
(import "${src}/nixos" {
  configuration = {
    imports = [
      "${src}/nixos/modules/virtualisation/qemu-vm.nix"
    ];
  };
}).config.system.build.toplevel
