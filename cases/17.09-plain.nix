let
  src = builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/17.09.tar.gz";
    sha256 = "sha256:0kpx4h9p1lhjbn1gsil111swa62hmjs9g93xmsavfiki910s73sh";
  };
in
(import "${src}/nixos" {
  configuration = {
    imports = [
      "${src}/nixos/modules/virtualisation/qemu-vm.nix"
    ];
  };
}).config.system.build.toplevel
