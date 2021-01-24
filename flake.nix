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
          pkg-config
        ];
        buildInputs = with pkgs; [
          openssl
        ];
        shellHook = ''
          test -f ~/.bashrc && source ~/.bashrc
        '';
      };
    in rec {
        defaultPackage = devEnv;
        devShell = devEnv;
      }
    );

}
