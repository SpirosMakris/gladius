pub mod render_gl;
pub mod resources;
mod triangle;

use resources::Resources;
use std::path::Path;

use anyhow::{Error, Result};


#[macro_use]
extern crate render_gl_derive;

extern crate nalgebra;
use nalgebra as na;

fn main() {
    if let Err(e) = run() {
        // println!("@ERROR: {}", anyhow_to_string(e));
        println!("@ERROR: {}", e);
    }
}

fn run() -> Result<()> {
    println!("Starting up..");

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let sdl = sdl2::init().map_err(|message| Error::msg(message))?;
    let video = sdl.video().map_err(|message| Error::msg(message))?;

    let gl_attr = video.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window
        .gl_create_context()
        .map_err(|message| Error::msg(message))?;

    let gl = gl::Gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    println!("Size of Gl context struct: {}", std::mem::size_of_val(&gl));
    
    
    let mut viewport = render_gl::Viewport::for_window(900, 700);
    let color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    let triangle = triangle::Triangle::new(&res, &gl)?;

    // Setup shared state for window

    viewport.set_used(&gl);
    color_buffer.set_used(&gl);


    // Main loop
    let mut event_pump = sdl.event_pump().map_err(|message| Error::msg(message))?;

    'main: loop {
        // Handle user input here
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                },
                _ => {}
            }
        }

        // Render
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        triangle.render(&gl);

        // Present rendered buffer
        window.gl_swap_window();

        std::thread::sleep(std::time::Duration::from_millis(17));
    }

    Ok(())
}
