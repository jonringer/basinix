with import <nixpkgs> { };

mkShell rec {
  buildInputs = [
    elmPackages.elm
    nodejs
  ];

  # shell commands to be ran upon entering shell
  shellHook = ''
  '';
}
