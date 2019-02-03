use std::error::Error;

use imgui::{ImGui, Ui};

pub fn run<F: FnMut(&Ui)>(
    title: &str,
    (w, h): (u32, u32),
    mut user: F,
) -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let window = video
        .window(title, w, h)
        .opengl()
        .position_centered()
        .build()?;

    let glctx = window.gl_create_context()?;
    window.gl_make_current(&glctx)?;

    let mut imgui = ImGui::init();
    //imgui.set_ini_filename(Some(imgui::ImString::new("example/imgui.ini")));
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui);

    let renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);
    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    let mut event_pump = sdl.event_pump()?;
    'mainloop: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            match &event {
                &sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Close,
                    ..
                }
                | &sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
            if imgui_sdl2.ignore_event(&event) {
                continue;
            }
        }

        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            //gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let ui = imgui_sdl2.frame(&window, &mut imgui, &event_pump.mouse_state());
        user(&ui);
        renderer.render(ui);

        window.gl_swap_window();
    }
    Ok(())
}
