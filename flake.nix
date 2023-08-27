{
  description = "Schizofox Assets Flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    systems = ["x86_64-linux" "aarch64-linux"];
    forEachSystem = nixpkgs.lib.genAttrs systems;

    pkgsForEach = nixpkgs.legacyPackages;
  in {
    packages = forEachSystem (system: {
      # we don't provide a default package, everything should be called explicitly
      transmute-rs = pkgsForEach.${system}.callPackage ./utils/transmute-rs {};
    });

    devShells = forEachSystem (system: {
      default = with pkgsForEach.${system};
        mkShell {
          name = "transmute-dev";
          RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
          packages = with pkgsForEach.${system};
            [
              cargo
              rustc
              rustfmt
              clippy
              rust-analyzer-unwrapped
            ]
            ++ [self.packages.${system}.transmute];
        };
    });
  };
}
