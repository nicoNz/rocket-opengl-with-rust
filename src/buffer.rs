use gl;

pub struct Vbo {
    gl: gl::Gl,
    id : gl::types::GLuint
    
}
//, chunks : Vec<u8>
impl Vbo {
    pub fn from_vector( gl: &gl::Gl, data : &Vec<f32>) -> Self {
        let mut id : gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }

        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, id);
            gl.BufferData(
                gl::ARRAY_BUFFER,                                                     
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, 
                data.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW,                           
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Vbo {
            gl : gl.clone(),
            id
        }
    }

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

    pub fn attach_vbo(&self, vbo : &Vbo, index : gl::types::GLuint) {
        self.bind();
        let gl = &self.gl;
        unsafe {
            gl.BindVertexArray(self.id);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo.id);
            gl.EnableVertexAttribArray(index); // this is "layout (location = 0)" in vertex shader
            gl.VertexAttribPointer(
                index,         // index of the generic vertex attribute ("layout (location = 0)")
                3,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
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



