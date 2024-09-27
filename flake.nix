{
  description = "Brainfuck interpreter and compilier";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in
        with pkgs; { 
          packages = {
            default = stdenv.mkDerivation {
              name = "bfic";
              src = ./.;

              buildInputs = [
                gnumake
                glibc
              ];
              nativeBuildInputs = [
                gcc
              ];

              buildPhase = ''
                make -k
              '';
              installPhase = ''
                mkdir -p $out/bin
                cp bfic $out/bin
              '';

            };
          };
        }
    );
}
