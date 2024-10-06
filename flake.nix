{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    {
      self,
      nixpkgs,
      treefmt-nix,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forEachSupportedSystem =
        f: nixpkgs.lib.genAttrs supportedSystems (system: f { pkgs = import nixpkgs { inherit system; }; });

      treefmtEval = pkgs: treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
    in
    {
      formatter = forEachSupportedSystem ({ pkgs }: (treefmtEval pkgs).config.build.wrapper);

      checks = forEachSupportedSystem (
        { pkgs }:
        {
          formatting = (treefmtEval pkgs).config.build.check self;
        }
      );

      packages = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.callPackage ./. { };
        }
      );

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              SDL2
            ];
          };
        }
      );
    };
}
