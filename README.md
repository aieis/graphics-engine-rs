# Overview

Rust project for graphics and rendering in vulkan. This project is spun off from a purely experimental project linked below.

# Dependencies

This project uses glslc to compile the shaders. It must exist on your path otherwise compilation will likely fail.

# Build the shaders

glslc -c  assets/shaders/shader.frag assets/shaders/shader.vert


In my emacs configuration:

	C-c d (aieis/vk-compile-dir)


# References

- Original:

- Font:
  + Iosevka: https://github.com/be5invis/Iosevka
  + PixelOperator

- Vulkan-Tutorials
  + https://vulkan-tutorial.com/
  + https://docs.vulkan.org/tutorial/latest/00_Introduction.html
  + https://github.com/bwasty/vulkan-tutorial-rs

- Camera Geometry: visionbook.mit.edu/imaging_geometry.html

- STB (for true type): https://github.com/nothings/stb

- Emacs conf: https://github.com/aieis/emacs-conf
