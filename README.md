# imgui-ext

Experimental crate to quickly build [imgui] UIs using annotations and a custom Derive. (Built on top of the [**imgui**] crate).

To try it, point directly to this repo in your `Cargo.toml`:
```toml
[dependencies]
imgui_ext = { git = "https://github.com/germangb/imgui-ext" }
```

## Usage example

```rust
// You need to import the prelude
use imgui_ext::prelude::*;

#[derive(ImGuiExt)]
struct Demo {
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

Result:

![ui result][result]

## Limitations

* `#[derive(ImGuiExt)]` is only available for `struct`s at the moment.
* There is no API to find out if a particular input has been triggered yet.
* Limited layout options.

## License

[MIT]

[imgui]: https://github.com/ocornut/imgui
[**imgui**]: https://github.com/Gekkio/imgui-rs
[**example**]: example/src/ui.rs
[result]: assets/demo.png
[MIT]: LICENSE.md
