#![recursion_limit = "128"]

extern crate proc_macro;

extern crate syn;
#[macro_use] extern crate quote;

#[proc_macro_derive(VertexAttribPointers, attributes())]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a string representation of the type definition
    // let s = input.to_string();

    // Parse the string representation
    // let ast = syn::parse_derive_input(&s).unwrap();
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let gen = generate_impl(&ast);

    // Return the generated impl
    // gen.parse().unwrap()
    gen.into()
}

fn generate_impl(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let gen = quote! {
        impl Vertex {
            fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = std::mem::size_of::<Self>();   // Byte offset between consecutive attributes

                let location = 0;   // layout (location = 0)
                let offset = 0; // Offset of the first component

                unsafe {
                    data::f32_f32_f32::vertex_attrib_pointer(gl, location, stride, offset);
                }

                let location = 1;
                let offset = offset + std::mem::size_of::<data::f32_f32_f32>(); // offset of first component

                unsafe {
                    data::f32_f32_f32::vertex_attrib_pointer(gl, location, stride, offset);
                }
            }
        }
    };

    gen.into()
}