use gl;
// use crate::file::vbo_description_parser::{
//     VaoDescription, 
//     AllowedVboType
// };

//use crate::VboF32;
use crate::render::vbo::Vbo;

enum VboBuildDescription {
    
}

pub struct Vao {
    gl: gl::Gl,
    id : gl::types::GLuint
}

impl Vao {
    pub fn new( gl: &gl::Gl,) -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut id);
        }
     
        Vao {
            gl :  gl.clone(),
            id
        }
        
    }



    pub fn attach_vbo(&self,  vbo: &dyn Vbo, index : gl::types::GLuint, ) {
        self.bind();
        let gl = &self.gl;
        let per_vertex = vbo.get_per_vertex();
        unsafe {
            gl.BindVertexArray(self.id);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo.get_id());
            gl.EnableVertexAttribArray(index); // this is "layout (location = 0)" in vertex shader
            gl.VertexAttribPointer(
                index,         // index of the generic vertex attribute ("layout (location = 0)")
                per_vertex,         // the number of components per generic vertex attribute
                vbo.get_data_type(), // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (per_vertex as usize * vbo.get_data_size()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(),                                     // offset of the first component
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }
        
    }
    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.id);
        }
    }
}



