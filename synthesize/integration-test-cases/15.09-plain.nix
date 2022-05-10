let
  src = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/15.09.tar.gz";
    sha256 = "sha256:0pn142js99ncn7f53bw7hcp99ldjzb2m7xhjrax00xp72zswzv2n";
  };
in
(import "${src}/nixos" {
  configuration = {
    imports = [
      "${src}/nixos/modules/virtualisation/qemu-vm.nix"
    ];
  };
}).config.system.build.toplevel
