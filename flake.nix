{
  description = "Basinix flake";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      inherit (pkgs.darwin.apple_sdk.frameworks) Security;
      pkgs = nixpkgs.legacyPackages.${system};
      devEnv = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          diesel-cli
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
        '';
        DATABASE_URL="/var/run/user/1000/basinix/test.db";
      };
    in rec {
        defaultPackage = devEnv;
        devShell = devEnv;
      }
    );

}
