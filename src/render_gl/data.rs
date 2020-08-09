use gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

impl f32_f32_f32 {
    /// Convenience constructor
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32 { d0, d1, d2 }
    }

    pub unsafe fn vertex_attrib_pointer(
        gl: &gl::Gl,
        location: usize,
        stride: usize,
        offset: usize,
    ) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,                                  // num components per generic vertex attribute
            gl::FLOAT,                          // Data type
            gl::FALSE,                          // Normalized (int to float conversion)
            stride as gl::types::GLint,         // Stride
            offset as *const gl::types::GLvoid, // offset of the first entry of this component
        );
    }
}

/// Create from 3-f32 tuple
impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}
