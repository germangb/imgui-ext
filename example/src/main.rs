use imgui::{im_str, ImGuiInputTextFlags};

use imgui_ext::prelude::*;

mod support;

#[derive(Default, ImGuiExt)]
struct Form {
    #[imgui(text(catch = "user"))]
    user: imgui::ImString,
    #[imgui(text(flags = "passwd_flags"))]
    passwd: imgui::ImString,
}

const fn passwd_flags() -> ImGuiInputTextFlags {
    ImGuiInputTextFlags::Password
}

#[derive(Default, ImGuiExt)]
struct FormExample {
    #[imgui(nested)]
    login_form: Form,
    #[imgui(
        button(label = "Login", catch = "click"),
        button(label = "Clear", catch = "clear"),
        separator(),
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
        button(label = "Click me!", catch = "click"),
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
        bullet(text = "Be nice to others."),
        bullet(text = "Don't repeat your password"),
        bullet(text = "Kill all humans."),
        bullet(slider(min = 0.0, max = 1.0))
    )]
    foo: f32,
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
    #[imgui(text(size = "size"), button(label = "submit", catch = "submit"))]
    comment: imgui::ImString,
}

const fn size() -> (f32, f32) {
    (200.0, 100.0)
}

#[derive(ImGuiExt, Default)]
struct Progress {
    #[imgui(progress)]
    progress: f32,
    #[imgui(progress)]
    _progress: f32,

    count: u32,
}

#[derive(ImGuiExt, Default)]
struct Image {
    //#[imgui(image)] TODO getting the wrong error
    #[imgui(image(size = "img_size"))]
    texture: usize,
    #[imgui(image(size = "img_size", tint = "img_tint", border = "img_border"))]
    texture_tint: usize,
}

const fn img_size() -> (f32, f32) {
    (512.0, 64.0)
}
const fn img_tint() -> (f32, f32, f32, f32) {
    (1.0, 0.0, 1.0, 1.0)
}
const fn img_border() -> (f32, f32, f32, f32) {
    (1.0, 1.0, 1.0, 1.0)
}

#[derive(ImGuiExt, Default)]
struct ExampleDocs {
    #[imgui(slider(min = 0.0, max = 4.0))]
    x: f32,
    #[imgui(input(step = 2))]
    y: i32,
    #[imgui(drag(label = "Drag 2D"))]
    drag_2d: [f32; 2],
    #[imgui(checkbox(label = "Turbo mode"), display(label = "Is turbo enabled?"))]
    turbo: bool,
}

fn main() {
    let mut image = Image::default();
    let mut sliders = Sliders::default();
    let mut example_docs = ExampleDocs::default();
    let mut progress = Progress::default();
    let mut bullet = Bullet::default();
    let mut comment = Comment::default();
    let mut demo = FormExample::default();
    let mut buttons = Buttons::default();
    let mut labels = Labels {
        foo: 42.0,
        bar: (2.0, false, 8),
        baz: "hello world".to_string(),
    };
    demo.login_form.user.reserve(64);
    demo.login_form.user.push_str("admin");
    demo.login_form.passwd.reserve(64);
    demo.login_form.passwd.push_str("p4ssw0rd");
    comment.name.reserve(64);
    comment.email.reserve(128);
    comment.comment.reserve(512);

    support::run("Demo", (800, 600), |ui| {
        ui.window(im_str!("README example")).build(|| {
            ui.imgui_ext(&mut example_docs);
        });

        ui.window(im_str!("Sliders")).build(|| {
            ui.imgui_ext(&mut sliders);
        });

        ui.window(im_str!("Progress")).build(|| {
            progress.count += 1;
            progress.progress = (progress.count % 60) as f32 / 60.0;
            progress._progress = ((progress.count % 240) as f32 - 120.0).abs() / 120.0;
            ui.imgui_ext(&mut progress);
        });

        ui.window(im_str!("Bullet")).build(|| {
            ui.imgui_ext(&mut bullet);
        });

        ui.window(im_str!("Text")).build(|| {
            ui.imgui_ext(&mut comment);
        });

        ui.window(im_str!("Nested")).build(|| {
            if ui.imgui_ext(&mut demo).clear {
                demo.login_form.user.clear();
                demo.login_form.passwd.clear();
            }
        });

        ui.window(im_str!("Button")).build(|| {
            if ui.imgui_ext(&mut buttons).click {
                buttons.count += 1;
            }
        });

        ui.window(im_str!("Labels")).build(|| {
            ui.imgui_ext(&mut labels);
        });

        ui.window(im_str!("Image")).build(|| {
            image.texture = 1;
            image.texture_tint = 1;
            ui.imgui_ext(&mut image);
        })

        //let mut texture = Texture { texture };
    });
}
