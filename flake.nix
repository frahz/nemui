{
  inputs.nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems =
        function: nixpkgs.lib.genAttrs systems (system: function nixpkgs.legacyPackages.${system});

    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = builtins.attrValues {
            inherit (pkgs)
              cargo
              clippy
              rustc
              rustfmt
              ;
          };
        };
      });

    };
}
