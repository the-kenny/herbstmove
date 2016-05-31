let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
in rec {
  rustEnv = stdenv.mkDerivation rec {
    name = "rust-env";
    version = "1.2.3.4";
    src = ./.;
    buildInputs = with pkgs; [ pkgconfig xlibs.libX11 xlibs.libXi ];
  };
 }
