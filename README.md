# imgui-ext

[![Build Status](https://img.shields.io/travis/germangb/imgui-ext/master.svg?style=flat-square)](https://travis-ci.org/germangb/imgui-ext)
[![Cargo package](https://img.shields.io/crates/v/imgui-ext.svg?style=flat-square)](https://crates.io/crates/imgui-ext)
[![docs.rs docs](https://docs.rs/imgui-ext/badge.svg?style=flat-square)](https://docs.rs/imgui-ext)
[![Master docs](https://img.shields.io/badge/docs-master-blue.svg?style=flat-square)](https://germangb.github.io/imgui-ext/)


A crate to quickly build [imgui] UIs using annotations and a custom Derive.

[imgui]: https://github.com/Gekkio/imgui-rs

## Features

* Encode UI directly on the types.
* Static code generation: [example].
* Nested UIs (see the [`imgui(nested(...))`][nested] annotation).
* Descriptive compiler errors.

[nested]: https://germangb.github.io/imgui-ext/imgui_ext/nested/index.html

[example]: ./CODEGEN.md

## Example

```rust
// You need to import the prelude
use imgui_ext::prelude::*;

#[derive(ImGuiExt)]
struct Example {
    #[imgui(slider(min = 0.0, max = 4.0))]
    x: f32,
    #[imgui(input(step = 2))]
    y: i32,
    #[imgui(drag(label = "Drag 2D"))]
    drag_2d: [f32; 2],
    #[imgui(
        checkbox(label = "Turbo mode"),
        display(label = "Is turbo enabled?"),
    )]
    turbo: bool,
}
```

#### Result:

![](assets/demo.png)

[result]: assets/demo.png

## Limitations

* `#[derive(ImGuiExt)]` is only supported for `struct`s with named fields, at the moment.
* Limited layout support.

## License

[MIT](LICENSE.md)
