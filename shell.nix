{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = (with pkgs; [
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.rustc
    pkgs.cargo
    pkgs.rustup
    pkgs.libxkbcommon
    pkgs.wayland
    pkgs.wayland-protocols
    pkgs.valgrind # used in cli tests, see cli/tests/cli_run.rs
    pkgs.vulkan-headers # here and below is all graphics stuff for examples/gui
    pkgs.vulkan-loader
    pkgs.vulkan-tools
    pkgs.vulkan-validation-layers
    pkgs.xorg.libXi
    pkgs.xorg.libxcb
  ]);
}