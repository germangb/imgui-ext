use std::error::Error;

use imgui::{Context, Ui};

use super::Window;

pub fn run<F: FnMut(&mut Window, &Ui)>(
    title: &str,
    (w, h): (u32, u32),
    mut user: F,
) -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let mut window = video
        .window(title, w, h)
        .opengl()
        .resizable()
        .allow_highdpi()
        .position_centered()
        .build()?;

    let glctx = window.gl_create_context()?;
    window.gl_make_current(&glctx)?;

    let mut imgui = Context::create();
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

    let renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);
    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    let mut window_params = Window::default();

    let mut event_pump = sdl.event_pump()?;
    'mainloop: while window_params.running {
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

        let [r, g, b, a] = window_params.color;
        //let _ = window.set_opacity(a);

        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());
        let ui = imgui.frame();
        user(&mut window_params, &ui);
        imgui_sdl2.prepare_render(&ui, &window);
        renderer.render(ui);
        window.gl_swap_window();
    }
    Ok(())
}
