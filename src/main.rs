pub mod render_gl;

fn main() {
    println!("Starting up..");

    let sdl = sdl2::init().unwrap();

    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_ctx = window.gl_create_context().unwrap();

    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    'main: loop {
        // Handle user input here
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();

        std::thread::sleep(std::time::Duration::from_millis(17));
    }
}

