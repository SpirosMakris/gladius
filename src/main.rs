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

struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

use std::ffi::{CStr, CString};

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        // Failure to compile, let's get the error message

        // 1. Get length of error message and create appropriate buffer & CString
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        // 2. Ask OpenGL to write the shader info log into our error value
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        // And finally we can return the error
        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    // Convert buffer to CString (re-uses allocation and appends 0 at end)
    unsafe { CString::from_vec_unchecked(buffer) }
}
