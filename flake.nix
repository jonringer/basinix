{
  description = "Basinix flake";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      devEnv = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          rustfmt
          clippy
          elmPackages.elm
        ];
      };
    in rec {
        defaultPackage = devEnv;
      }
    );

}
