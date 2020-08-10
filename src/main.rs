pub mod render_gl;
pub mod resources;

use resources::Resources;
use std::path::Path;

use anyhow::{Error, Result};

use render_gl::data;

#[macro_use]
extern crate render_gl_derive;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::f32_f32_f32,
    clr: data::f32_f32_f32,
}

// impl Vertex {
//     fn vertex_attrib_pointers(gl: &gl::Gl) {
//         let stride = std::mem::size_of::<Self>(); // Stride (byte offset between consecutive attributes)
//         let location = 0; // layout (location = 0)
//         let offset = 0; // Offset of the first component

//         // Specify layout
//         // Position (0)
//         unsafe {
//             data::f32_f32_f32::vertex_attrib_pointer(gl, location, stride, offset);
//         }

//         // Vertex Color (1)

//         let location = 1; // layout (location = 1)
//         let offset = offset + std::mem::size_of::<data::f32_f32_f32>();

//         unsafe {
//             data::f32_f32_f32::vertex_attrib_pointer(gl, location, stride, offset);
//         }
//     }
// }

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

    let shader_program = render_gl::Program::from_res(&gl, &res, "shaders/triangle")?;

    // Set up VBO (Vertex Buffer Object)
    // #[rustfmt::skip]
    // let vertices: Vec<f32> = vec![
    //     // positions
    //      0.5, -0.5, 0.0,    1.0, 0.0, 0.0,  // Bottom right
    //     -0.5, -0.5, 0.0,    0.0, 1.0, 0.0,  // Bottom left
    //      0.0,  0.5, 0.0,    0.0, 0.0, 1.0,  // Top
    // ];

    #[rustfmt::skip]
    let vertices: Vec<Vertex> = vec![
        //  positions               // colors
        Vertex{ pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },  // Bottom right
        Vertex{ pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() },  // Bottom left
        Vertex{ pos: (0.0,  0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },  // Top
    ];

    // Create VBO
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }

    // Upload our data to the VBO
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,                                                          // target
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    // Setup VAO (Vertex Array Object)
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }

    // Define the VAO
    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        Vertex::vertex_attrib_pointers(&gl);

        // And unbind VBO & VAO
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    // Setup shared state for window
    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // Main loop
    let mut event_pump = sdl.event_pump().map_err(|message| Error::msg(message))?;

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
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw triangle
        shader_program.set_used();
        unsafe {
            gl.BindVertexArray(vao);
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // Starting index in the enabled array
                3,             // NUmber of indices to be rendered
            )
        }

        // Present rendered buffer
        window.gl_swap_window();

        std::thread::sleep(std::time::Duration::from_millis(17));
    }

    Ok(())
}
