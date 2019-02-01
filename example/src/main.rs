use imgui_ext::prelude::*;
use imgui_ext::{imgui_ext, Events};

use imgui::ImGuiInputTextFlags;

mod support;

#[derive(Default, ImGuiExt)]
struct Form {
    #[imgui(input)]
    user: imgui::ImString,
    #[imgui(input(flags = "passwd_flags"))]
    passwd: imgui::ImString,
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
        checkbox(label = "Remember login?", catch = "rem")
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

fn size() -> (f32, f32) {
    (64.0, 24.0)
}

fn main() {
    let mut demo = Example::default();

    demo.login_form.user = imgui::ImString::with_capacity(64);
    demo.login_form.passwd = imgui::ImString::with_capacity(64);

    support::run("Demo", (640, 480), |ui| {
        let events = imgui_ext!(ui, &mut demo);

        if events.click {
            println!("click!");
        }

        if events.rem {
            println!("rem!");
        }
    });
}
