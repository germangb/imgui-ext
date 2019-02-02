## Code generation

Generated code doesn't contain any extra dynamic allocations:

### Example

This is what you would write:

```rust
use imgui_ext::prelude::*;

#[derive(Default, ImGuiExt)]
struct Example {
    // `catch` is an optional parameter that is used to query discrete input events (mouse clicks, value changes, etc)
    // See the docs for more info on this param.
    #[imgui(slider(label = "Slider label", min = 0.0, max = 1.0, catch = "change_foo"))]
    foo: f32,
    #[imgui(checkbox(catch = "change_bar"))]
    bar: bool,
}

let mut example = Example::default();

// init ui (regular `Ui` from the imgui crate)...
let ui: &Ui = ...;

let events = ui.imgui_ext(ui, &mut example);
if events.change_foo {
    println!("New value: {}", example.foo);
}
```

### Generated

This is what the compiler generates for you:

```rust
use imgui_ext::prelude::*;

// The original type is left intact
#[derive(Default)]
struct Example {
    foo: f32,
    bar: bool,
}

// A type is generated to hold input events.
#[derive(Default)]
struct ExampleImGuiExt {
    change_foo: bool,
    change_bar: bool,
}

// The ImGuiExt trait is implemented
impl ImGuiExt for Example {
    type Events = ExampleImGuiExt;
    fn imgui_ext(ui: &Ui, ext: &mut Self) -> Self::Events {
        use imgui::{im_str, SliderFloat};
        
        // draw the UI and collect input events
        let mut events = ExampleImGuiExt::default();
        events.change_foo = ui.slider_float(imgui::im_str!("Slider label"), &mut ext.foo, 0.0, 1.0).build();
        events.change_bar = check: ui.checkbox(imgui::im_str!("bar"), &mut ext.bar);
        
        events
    }
}

// the rest is the same...
```

