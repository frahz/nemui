{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
    ];
    forEachSystem = nixpkgs.lib.genAttrs systems;
    pkgsForEach = forEachSystem (system:
      import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      });
  in {
    devShells = forEachSystem (system: let
      pkgs = pkgsForEach.${system};
    in {
      default = pkgs.mkShell {
        buildInputs = [
          pkgs.rust-bin.stable.latest.default
        ];
      };
    });
  };
}
