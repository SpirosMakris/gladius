use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;
// use thiserror::Error;
use anyhow::{Error, Result};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::f32_f32_f32,
    clr: data::f32_f32_f32,
}

pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer, // _ to disable warning about not used vbo
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Triangle> {
        // initialization code that uses resources to load
        // data for a triangle and wrap this data in a
        // Triangle struct or return an Error

        let program = render_gl::Program::from_res(&gl, &res, "shaders/triangle")?;

        // Set up VBO (Vertex Buffer Object)

        #[rustfmt::skip]
        let vertices: Vec<Vertex> = vec![
            //  positions               // colors
            Vertex{ pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },  // Bottom right
            Vertex{ pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() },  // Bottom left
            Vertex{ pos: (0.0,  0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },  // Top
        ];

        // Create VBO

        let vbo = buffer::ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();


        // Setup VAO (Vertex Array Object)
        let vao = buffer::VertexArray::new(&gl);
        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao,
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        // function that renders the triangle based on loaded data
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // Starting index in the enabled arrays
                3,             // Number of indices to be rendered
            );
        }
    }
}
