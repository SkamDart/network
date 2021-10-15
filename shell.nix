with import <nixpkgs> {};
pkgs.stdenv.mkDerivation {
    name = "network";
    version = "0.0.0.1";
    buildInputs = [
      rustc
      cargo
      cmake
    ];
}
