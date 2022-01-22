{
  description = "Basinix flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      inherit (pkgs.darwin.apple_sdk.frameworks) Security;
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
          sqlite
        ] ++ lib.optionals stdenv.isDarwin [ Security ];
        shellHook = ''
          test -f ~/.bashrc && source ~/.bashrc
          # needed for sqlx to do query!
          export DATABASE_URL=sqlite::memory:
        '';
        DATABASE_URL="/var/run/user/1000/basinix/test.db";
      };
    in rec {
        defaultPackage = devEnv;
        devShell = devEnv;
      }
    );

}
