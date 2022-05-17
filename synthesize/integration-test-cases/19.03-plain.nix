let
  src = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/19.03.tar.gz";
    sha256 = "sha256:0q2m2qhyga9yq29yz90ywgjbn9hdahs7i8wwlq7b55rdbyiwa5dy";
  };
in
(import "${src}/nixos" {
  configuration = {
    imports = [
      "${src}/nixos/modules/virtualisation/qemu-vm.nix"
    ];
  };
}).config.system.build.toplevel
