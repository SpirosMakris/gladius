mod bindings {
    // Encapsulate the generated Gl struct into the bindings
    // module so we can namespace it and use it as an inner field
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
pub use bindings::Gl as InnerGl;    // Re-export inner with new name

use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl Gl {
    /// Forward the `load_with` constructor, which will create
    /// original `Gl` but then wrap it in `Rc`
    pub fn load_with<F>(loadfn: F) -> Self 
        where F: FnMut(&'static str) -> *const types::GLvoid      
    {
        Gl {
            inner: Rc::new(bindings::Gl::load_with(loadfn))
        }
    }
}

// Since we don't want to use `gl.inner` everywhere to access 
// wrapped value, we use `Deref` to forward call to inner impl
impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &bindings::Gl {
        &self.inner
    }
}
