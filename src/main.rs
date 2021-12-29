mod show;
mod state;
mod util;
mod window;

fn create_display(
    event_loop: &glutin::event_loop::EventLoop<()>,
) -> (
    glutin::WindowedContext<glutin::PossiblyCurrent>,
    glow::Context,
) {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 400.0,
            height: 600.0,
        })
        .with_title("pls");

    let gl_window = unsafe {
        glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true)
            .build_windowed(window_builder, event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };

    let gl = unsafe { glow::Context::from_loader_function(|s| gl_window.get_proc_address(s)) };

    unsafe {
        use glow::HasContext as _;
        gl.enable(glow::FRAMEBUFFER_SRGB);
    }

    (gl_window, gl)
}

fn main() -> anyhow::Result<()> {
    let qualifier = ""; // NOTE: something like com.mydomain
    let organisation = ""; // NOTE: Try Jumping
    let application = "pls";

    println!("Hostname: {:?}", hostname::get());

    let test_config_dir = std::path::PathBuf::from("test");
    let config_dir = if cfg!(feature = "test") {
        test_config_dir
    } else {
        directories::ProjectDirs::from(qualifier, organisation, application)
            .map(|d| d.config_dir().to_owned())
            .unwrap_or(test_config_dir)
    };
    let config_path = config_dir.join("pls.toml");

    println!("Config location: {:?}", config_path);
    let mut state = state::State::new(&config_path)?;

    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let (gl_window, gl) = create_display(&event_loop);

    let mut egui = egui_glow::EguiGlow::new(&gl_window, &gl);

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            egui.begin_frame(gl_window.window());

            let mut theme = egui::Visuals::light();
            theme.widgets.inactive.fg_stroke.color = egui::Color32::BLACK;
            theme.widgets.inactive.bg_stroke.color = egui::Color32::from_gray(192);
            theme.widgets.inactive.bg_stroke.width = 2.0;
            egui.ctx().set_visuals(theme);
            egui::CentralPanel::default().show(egui.ctx(), |ui| {
                window::show(&mut state, ui);
            });

            let (needs_repaint, shapes) = egui.end_frame(gl_window.window());

            *control_flow = if needs_repaint {
                gl_window.window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                let color = egui::Rgba::from_rgb(0.1, 0.3, 0.2);
                unsafe {
                    use glow::HasContext as _;
                    gl.clear_color(color[0], color[1], color[2], color[3]);
                    gl.clear(glow::COLOR_BUFFER_BIT);
                }

                // draw things behind egui here

                egui.paint(&gl_window, &gl, shapes);

                // draw things on top of egui here

                gl_window.swap_buffers().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                if egui.is_quit_event(&event) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                if let glutin::event::WindowEvent::Resized(physical_size) = event {
                    gl_window.resize(physical_size);
                }

                egui.on_event(&event);

                gl_window.window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }
            glutin::event::Event::LoopDestroyed => {
                egui.destroy(&gl);
            }

            _ => (),
        }
    });
}
