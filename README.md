# imgui-ext

[![Build Status](https://img.shields.io/travis/germangb/imgui-ext/master.svg?style=flat-square)](https://travis-ci.org/germangb/imgui-ext)
[![Cargo package](https://img.shields.io/crates/v/imgui-ext.svg?style=flat-square)](https://crates.io/crates/imgui-ext)
[![docs.rs docs](https://docs.rs/imgui-ext/badge.svg?style=flat-square)](https://docs.rs/imgui-ext)
[![Master docs](https://img.shields.io/badge/docs-master-blue.svg?style=flat-square)](https://germangb.github.io/imgui-ext/)


A crate to quickly build [imgui](https://github.com/Gekkio/imgui-rs) UIs using annotations and a `derive` macro.

```rust
#[derive(imgui_ext::Gui)]
struct Example {
    #[imgui(slider(min = 0.0, max = 4.0))]
    x: f32,
    #[imgui(input(step = 2))]
    y: i32,
    #[imgui(drag(label = "Drag 2D"))]
    drag_2d: [f32; 2],
    #[imgui(checkbox(label = "Turbo mode"))]
    turbo: bool,
}
```

![](assets/demo.png)


## Examples

```bash
# example UI
cargo run --example ui

# integration with n-algebra types
cargo run --example nalgebra
```

[result]: assets/demo.png

## Limitations

* `#[derive(imgui_ext::Gui)]` is only supported for `struct`s with named fields.

## License

[MIT](LICENSE.md)
