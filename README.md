# Jeri

This is a streamlined graphics library designed for rendering basic shapes such as triangles,
rectangles, circles, and text. It operates without any external dependencies.

The core functionality of the library is to create and manipulate a vector of RGBA pixels. Users
have the flexibility to decide how to utilize this output, whether for generating images, displaying
on a screen, or other applications.

The library is inspired by [olive.c](https://github.com/tsoding/olive.c/).

## Examples

Look at the `/tests/renderer.rs` to understand how to use the library, in there there are a few
examples rendered in '.png' using the [png](https://github.com/image-rs/image-png) crate.
