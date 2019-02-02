use imgui_ext::imgui_ext;
use imgui_ext::prelude::*;

use imgui::ImGuiInputTextFlags;

mod support;

#[derive(Default, ImGuiExt)]
struct Form {
    //#[imgui(text(catch = "user", size = "multi_size"))]
    #[imgui(text(catch = "user"))]
    user: imgui::ImString,
    #[imgui(text(flags = "passwd_flags"))]
    passwd: imgui::ImString,
}

fn multi_size() -> (f32, f32) {
    (200.0, 100.0)
}

fn passwd_flags() -> ImGuiInputTextFlags {
    ImGuiInputTextFlags::Password
}

#[derive(Default, ImGuiExt)]
struct Example {
    #[imgui(nested)]
    login_form: Form,
    #[imgui(
        button(label = "Login", size = "size", catch = "click"),
        separator,
        checkbox(label = "Remember login?", catch = "mem")
    )]
    remember: bool,
    #[imgui(
        new_line,
        bullet(text = "Be nice."),
        bullet(text = "Kill all humans."),
        bullet(text = "Don't reveal your password.")
    )]
    _bullet: (),
}

#[derive(ImGuiExt, Default)]
struct Buttons {
    #[imgui(
        button(size = "btn_size", label = "Click me!", catch = "click"),
        separator,
        display(label = "Clicks")
    )]
    count: i32,
}

#[derive(ImGuiExt)]
struct Labels {
    #[imgui]
    foo: f32,
    #[imgui(display(label = "Tuple", display = "({}, {}, {})", 0, 1, 2))]
    bar: (f32, bool, usize),
    #[imgui(label = "String param")]
    baz: String,
}

#[derive(ImGuiExt, Default)]
struct Bullet {
    #[imgui(
        bullet(text = "Be Nice"),
        bullet(text = "Kill all humans"),
        slider(min = 0.0, max = 1.0)
    )]
    foo: f32,
}

fn btn_size() -> (f32, f32) {
    (100.0, 20.0)
}

fn size() -> (f32, f32) {
    (64.0, 24.0)
}

#[derive(ImGuiExt, Default)]
struct Sliders {
    #[imgui(slider(min = 0.0, max = 1.0))]
    foo: f32,
    #[imgui(slider(min = 0, max = 16, format = "bar = %.02f"))]
    bar: [i32; 2],
}

#[derive(ImGuiExt, Default)]
struct Comment {
    #[imgui(text)]
    name: imgui::ImString,
    #[imgui(text)]
    email: imgui::ImString,
    #[imgui(text(size = "size2"), button(label = "submit"))]
    comment: imgui::ImString,
}

const fn size2() -> (f32, f32) {
    (200.0, 100.0)
}

fn main() {
    let mut bullet = Bullet::default();
    let mut comment = Comment::default();
    let mut demo = Example::default();
    let mut buttons = Buttons::default();
    let mut labels = Labels {
        foo: 42.0,
        bar: (2.0, false, 8),
        baz: "hello world".to_string(),
    };
    let mut sliders = Sliders::default();

    demo.login_form.user = imgui::ImString::with_capacity(64);
    demo.login_form.passwd = imgui::ImString::with_capacity(64);

    support::run("Demo", (640, 480), |ui| {
        /*
        let events = ui.imgui_ext(&mut buttons);
        if events.click {
            buttons.count += 1;
        }
        */

        let events = ui.imgui_ext(&mut comment);
        /*
        if events.user {
            println!("New value: {:?}", demo.login_form.user);
        }
        */

        /*
        let events = ui.imgui_ext(&mut demo.login_form);
        if events.user {
            println!("new user: {:?}", demo.login_form.user);
        }
        */
    });
}
