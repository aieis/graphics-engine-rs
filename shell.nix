{ pkgs ? import <nixpkgs> {} }:

let
  # Libraries needed at link time (-lvulkan) and/or dlopen'd at runtime (winit, ash)
  libs = with pkgs; [
    vulkan-loader
    wayland
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
  ];
in
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # Sets LIBCLANG_PATH and BINDGEN_EXTRA_CLANG_ARGS for bindgen (crates/stb-truetype)
    rustPlatform.bindgenHook
    # Provides glslc, used by build.rs to compile shaders
    shaderc
  ];

  buildInputs = libs;

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
}
