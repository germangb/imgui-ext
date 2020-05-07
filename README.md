⚠️ *imgui-ext needs a full rewrite. Since it was my first attempt at making a procedural macro, it has grown messy and difficult to maintain due to lack of planning. Use with caution.*

You can also check out [imgui-inspect-derive](https://crates.io/crates/imgui-inspect-derive), which looks neat!

# imgui-ext

[![Build Status](https://img.shields.io/travis/germangb/imgui-ext/master.svg?style=flat-square)](https://travis-ci.org/germangb/imgui-ext)
[![Cargo package](https://img.shields.io/crates/v/imgui-ext.svg?style=flat-square)](https://crates.io/crates/imgui-ext)
[![docs.rs docs](https://docs.rs/imgui-ext/badge.svg?style=flat-square)](https://docs.rs/imgui-ext)
[![Master docs](https://img.shields.io/badge/docs-master-blue.svg?style=flat-square)](https://germangb.github.io/imgui-ext/)

A derive-macro for [imgui](https://crates.io/crates/imgui).

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

let mut example = Example { /* skipped */ };

imgui::Window::new(im_str!("debug")).build(ui, || {
    use imgui_ext::UiExt;
    
    if ui.draw_gui(&mut example).turbo() {
        println!("Turbo mode value changed: {}", example.turbo);
    }
})
```

![](assets/demo.png)


## Examples

```bash
# codegen example (see examples/codegen.rs to see the macro expansion)
cargo run --example codegen

# integration with nalgebra types
cargo run --example nalgebra
```

[result]: assets/demo.png

## Limitations

* `#[derive(imgui_ext::Gui)]` is only supported for `struct`s with named fields.

## License

[MIT](LICENSE.md)
