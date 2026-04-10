zglyph
=======

**zglyph** is a small Rust library for generating and rendering 3D glyphs. It provides:

* A set of geometric primitives (cube, STL import, etc.) located in the `models` module.
* A simple software renderer that can output glyphs to the terminal using Unicode block
  characters.
* Utilities for transforming and projecting 3D points onto a 2D plane.

The library is intended for educational purposes and for quick visualisation of
3‑D shapes directly in a terminal window. It does not depend on heavyweight graphics
libraries and works on Windows, macOS and Linux.

## Building

The project uses Cargo, the Rust package manager. To build the library and the
example binaries, run:

```sh
cargo build --release
```

The compiled binary will be placed in `target/release/zglyph`.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
zglyph = { path = "." }
```

Then you can create a glyph and render it:

```rust
use zglyph::renderer::Renderer;
use zglyph::models::cube::Cube;

fn main() {
    let cube = Cube::new(1.0);
    let mut renderer = Renderer::new();
    renderer.render(&cube);
}
```

## License

This project is licensed under the MIT License – see the `LICENSE` file for details.

## Contributing

Contributions are welcome. Please open an issue or submit a pull request with a
clear description of the change. Ensure that the code builds on all supported
platforms and that any new functionality includes appropriate documentation and
tests.

---

For more information, see the source code in the `src/` directory.
