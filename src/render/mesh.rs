use crate::render::vbo::{
    Vbo,
    VboF32,
    VboU8,
    AllowedBufferType
};
use crate::render::vao::Vao;
use crate::render::shader::Shader;
use gl;
use std::vec::Vec;
use std::boxed::Box;
use std::rc::Rc;

pub struct BufferDescription {
    pub per_vertex: i32,
    pub data: AllowedBufferType,
    pub n_elements: usize,
    pub attribute_name: String
}

pub struct MeshDescription {
    pub buffers: Vec<BufferDescription>,
    pub draw_mode : gl::types::GLuint,
}

impl AllowedBufferType {
    pub fn gen_vbo(&self, gl: &gl::Gl, per_vertex: i32) -> Box<dyn Vbo> {
        match self {
            AllowedBufferType::F32(ref vec) => {
                return Box::new(VboF32::from_vector(gl, vec, per_vertex));
            }
            AllowedBufferType::SHORT(ref vec) => {
                return Box::new(VboU8::from_vector(gl, vec, per_vertex));
            }
        }
    }
}

type VboWithAssiciatedLoc = (Box<dyn Vbo>, usize, Option<String>);

pub struct Mesh {
    //uniforms: std::collections::HashMap<String, Uniform>,
    //transform : Transform,
    vbos: Vec<VboWithAssiciatedLoc>,
    vao: Vao,
    gl: gl::Gl,
    draw_mode: gl::types::GLuint,
    n_verts: i32,
    pub shader: Rc<Shader>
} 

impl Mesh {

    pub fn new(gl: &gl::Gl, shader: &Rc<Shader>) -> Self {
        let buffers: Vec<VboWithAssiciatedLoc> = Vec::new();
        Self {
            gl : gl.clone(),
            vao : Vao::new(gl),
            vbos : buffers,
            draw_mode : gl::TRIANGLES,
            n_verts : 0,
            shader: shader.clone()
        }
    }

    pub fn from_description(gl: &gl::Gl, description: &MeshDescription, shader: &Rc<Shader>) -> Self {
        //let vbos: Vec<VboWithAssiciatedLoc> = Vec::new();
        let mut this = Self::new(gl, shader);

        for buffer in description.buffers.iter() {
            &this.set_buffer_at_named_location( 
                buffer.data.gen_vbo(
                    gl, 
                    buffer.per_vertex
                ), 
                &buffer.attribute_name
            );
        }

        this.draw_mode = description.draw_mode;

        return this;
    }

    pub fn set_buffer_at_location(&mut self, boxed_vbo:  Box<dyn Vbo>,loc: gl::types::GLuint) {

        let vbo = boxed_vbo.as_ref();
        //let vbo = &(*boxed_vbo);
        if loc == 0 {
            self.n_verts =  vbo.get_n_elements() as i32;
        }
        self.vao.attach_vbo(vbo, loc);
        self.vbos.push((boxed_vbo, loc as usize, None));
    }

    pub fn set_buffer_at_named_location(&mut self, boxed_vbo:  Box<dyn Vbo>, attribute_name: &String) {
       
        
                let vbo = boxed_vbo.as_ref();
                
                match self.shader.program.get_attribute_location(attribute_name) {
                    Ok(loc) => {
                        if loc == 0 {
                           self.n_verts =  vbo.get_n_elements() as i32;
                        }
                        self.vao.attach_vbo(vbo, loc as u32);
                        self.vbos.push((
                            boxed_vbo, 
                            loc as usize, 
                            Some(attribute_name.clone())
                        ));
                    },
                    Err(()) => {
                        panic!("Panic as no implementation was provide if the attribute location was not found \nfail to find {}", attribute_name);
                    }
                }
            
        
    }

    pub fn draw(&self) {
        self.shader.use_shader();
        self.vao.bind();
        unsafe {
            self.gl.DrawArrays(
                self.draw_mode, // mode
                0,             // starting index in the enabled arrays
                self.n_verts             // number of indices to be rendered
            );
        }
    }
}

