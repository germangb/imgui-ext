# imgui-ext

Experimental crate to quickly build [imgui] UIs using annotations and a custom Derive. (Built on top of the [**imgui**] crate).

To try it, point directly to this repo in your `Cargo.toml`:
```toml
[dependencies]
imgui_ext = { git = "https://github.com/germangb/imgui-ext" }
```

For documentation on usage and the currently available tags, see this [**example**].

## Usage example

```rust
#[derive(ImGuiExt)]
struct Demo {
    #[imgui(slider(min = 0.0, max = 4.0))]
    x: f32,
    #[imgui(input(step = 2))]
    y: i32,
    #[imgui(drag(label = "Drag 2D"))]
    drag_2d: [f32; 2],
    #[imgui(label = "Turbo mode")]
    turbo: bool,
}
```

Result:

![ui result][result]

## Limitations

* `#[derive(ImGuiExt)]` is only available for `struct`s, at the moment.
* There is no API to find out if a particular input has been triggered yet.
* Implemented for just a handful of primitive field types (numbers, arrays, and strings mostly).
* No straight forward way to annotate fields with arbitrary types *yet* (there will be an API for this in the near future).

----

## License

[MIT]

[imgui]: https://github.com/ocornut/imgui
[**imgui**]: https://github.com/Gekkio/imgui-rs
[**example**]: examples/example.rs
[result]: assets/demo.png
[MIT]: LICENSE.md