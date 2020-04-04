use gl;
use std;
use std::ffi::{CStr, CString};

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
    u_offset : gl::types::GLint,
    u_offset_value : f32,
    u_vp : gl::types::GLint,
    pub u_vp_value : glm::Mat4
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl.LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl.DetachShader(program_id, shader.id());
            }
        }


       
        let u_offset_loc =  unsafe{ 
            gl.GetUniformLocation(program_id, CString::new("offset").unwrap().into_raw())
        };
        let u_vp_loc =  unsafe{ 
            gl.GetUniformLocation(program_id, CString::new("vp").unwrap().into_raw())
        };
        



        Ok(Program { 
            gl : gl.clone(),
            id: program_id,
            u_offset : u_offset_loc,
            u_offset_value : 0.0,
            u_vp : u_vp_loc,
            u_vp_value : glm::translate(&glm::identity(), &glm::vec3(0.5, 0., 0.)) 
         })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.Uniform1f(self.u_offset, self.u_offset_value);
            self.gl.UniformMatrix4fv(self.u_vp, 1, gl::FALSE, self.u_vp_value.as_ptr());
            self.gl.UseProgram(self.id);
        }
    }

    pub fn set_offset(&mut self, value: f32) {
        self.u_offset_value = value;
    }

}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(gl: &gl::Gl, source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(gl, source, kind)?;
        Ok(Shader { 
            id,
            gl : gl.clone()
         })
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn shader_from_source(gl: &gl::Gl, source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl.CreateShader(kind) };
    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}