{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
    pkg-config
    rustup
    gcc
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH";
    export LD_LIBRARY_PATH=./oidn/oidn/lib:$LD_LIBRARY_PATH
    export OPENSSL_DIR=${pkgs.openssl.dev}
    export OPENSSL_LIB_DIR=${pkgs.openssl.out}/lib
    export OPENSSL_INCLUDE_DIR=${pkgs.openssl.dev}/include
    export PKG_CONFIG_PATH=${pkgs.openssl.out}/lib/pkgconfig
  '';
}
