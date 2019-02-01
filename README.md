# imgui-ext

[![Build Status](https://travis-ci.org/germangb/imgui-ext.svg?branch=master)](https://travis-ci.org/germangb/imgui-ext)

***Warning: API from master is heavily subject to changes.***

A crate to quickly build **[imgui]** UIs using annotations and a custom Derive.

```toml
[dependencies]
imgui_ext = "0.1"
```

[imgui]: https://github.com/Gekkio/imgui-rs

## Features

* Encode UI directly on the types.
* Static code generation (no heap allocations): [example].
* Support for imgui's slider, input, checkbox, drag and button widgets.
* Support for building nested UIs (see the [`imgui(nested)`] attribute).
* Descriptive compiler errors.

[`imgui(nested)`]: ./README.md

[example]: ./CODEGEN.md

## Usage example

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
        label(label = "Is turbo enabled?"),
    )]
    turbo: bool,
}
```

#### Result:

![][result]

## Limitations

* `#[derive(ImGuiExt)]` is only supported for `struct`s with named fields, at the moment.
* Limited layout support.

## License

[MIT]

[**example**]: example/src/ui.rs
[result]: assets/demo.png
[MIT]: LICENSE.md
