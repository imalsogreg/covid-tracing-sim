{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    pkgs.openssl
    pkgs.pkg-config
    pkgs.dhall
    pkgs.dhall-json
    (pkgs.callPackage ./default.nix {}).cargoDeps
  ];
}
