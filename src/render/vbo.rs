use gl;

// pub enum AllowedVboType {
//     Float32(VboDescription<f32>),
//     Short(VboDescription<u8>)
// }

pub trait Vbo {
    fn get_per_vertex(&self) -> i32;
    fn get_id(&self) -> gl::types::GLuint;
    fn get_data_type(&self)-> u32;
    fn get_data_size(&self) -> usize;
    fn get_n_elements(&self) -> usize;
}

pub enum AllowedBufferType {    
    F32(Vec<f32>),
    SHORT(Vec<u8>)
}

struct VboDescription<T> {
    pub attribute_buffer_name : String,
    pub attribute_buffer_data : Vec<T>,
    pub per_vertex : u8,

}


pub struct VboF32 {
    gl: gl::Gl,
    per_vertex: i32,
    pub id: gl::types::GLuint,
    n_elements: usize,
    data: Option<Vec<f32>>
}
//, chunks : Vec<u8>
impl VboF32 {
    pub fn from_vector( gl: &gl::Gl, data: &Vec<f32>, per_vertex: i32) -> Self {
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

        let per_vertex_as_usize = per_vertex as usize;

        if data.len() % per_vertex_as_usize ==! 0 {
            println!("vbo has unvalid data");
        }

        VboF32 {
            gl : gl.clone(),
            id,
            per_vertex,
            n_elements : data.len() / per_vertex_as_usize,
            data: None
        }
    }
}

impl Vbo for VboF32 {
    fn get_id(&self) -> gl::types::GLuint {
        return self.id;
    }
    fn get_per_vertex(&self) -> i32 {
        return self.per_vertex;
    }
    fn get_data_type(&self) -> u32 {
        return gl::FLOAT;
    }
    fn get_data_size(&self) -> usize {
        return std::mem::size_of::<u32>();
    }
    fn get_n_elements(&self) -> usize {
        return self.n_elements;
    }

}

/* --------------------------U8----------------------- */
pub struct VboU8 {
    gl: gl::Gl,
    per_vertex: i32,
    pub id: gl::types::GLuint,
    n_elements: usize,
    data: Option<Vec<u8>>
}
//, chunks : Vec<u8>
impl VboU8 {
    pub fn from_vector( gl: &gl::Gl, data: &Vec<u8>, per_vertex: i32) -> Self {
        let mut id : gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }

        unsafe {
            gl.BindBuffer(gl::ARRAY_BUFFER, id);
            gl.BufferData(
                gl::ARRAY_BUFFER,                                                     
                (data.len() * std::mem::size_of::<u8>()) as gl::types::GLsizeiptr, 
                data.as_ptr() as *const gl::types::GLvoid, 
                gl::STATIC_DRAW,                           
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        let per_vertex_as_usize = per_vertex as usize;

        if data.len() % per_vertex_as_usize ==! 0 {
            println!("vbo has unvalid data");
        }

        VboU8{
            gl : gl.clone(),
            id,
            per_vertex,
            n_elements : data.len() / per_vertex_as_usize,
            data: None
        }
    }
}

impl Vbo for VboU8 {
    fn get_id(&self) -> gl::types::GLuint {
        return self.id;
    }
    fn get_per_vertex(&self) -> i32 {
        return self.per_vertex;
    }
    fn get_data_type(&self) -> u32 {
        return gl::SHORT;
    }
    fn get_data_size(&self) -> usize {
        return std::mem::size_of::<u8>();
    }
    fn get_n_elements(&self) -> usize {
        return self.n_elements;
    }

}



