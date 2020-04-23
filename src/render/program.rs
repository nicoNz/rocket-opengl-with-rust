
use gl;
use gl::types::GLint;

use crate::render::raw_shader::RawShader;
use crate::render::uniform::Uniform;
use crate::file::shader_description_parser::UniformDescription;
use crate::render::gl_error::create_whitespace_cstring_with_len;

use std::ffi::{CStr};

use crate::file::shader_description_parser::ShaderDescription;
use crate::render::uniform::UniformValue;
use crate::file::util::*;

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}


impl Program {


    // pub fn register_uniforms() {
        
    // }

    // pub fn register_uniform(&mut self, uniform_description: UniformDescription) -> GLint {
    //     let loc = self.get_uniform_location(uniform_description.name);
    //     match loc {
    //         Ok(loc)=> {
    //             println!("name {} found at loc {}", location_name, loc);
    //             if !self.uniforms.insert(
    //                 loc, 
    //                 Uniform::from_uniform_description(uniform_description)
    //             ).is_none() {
    //                 println!("Error while trying to get location of {}, key {} already exist",location_name, loc)
    //             }
    //             loc
    //         },
    //         Err(())=>panic!("panic because the behavious when a location is not found is not implemented")
    //     }
    // }
    
    pub fn from_shaders(gl: &gl::Gl, shaders: &[RawShader]) -> Result<Program, String> {
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
       

        Ok(Program {
            gl : gl.clone(),
            id: program_id,
        })
    }

    
    pub fn get_attribute_location(&self, attribute_name: &String) -> Result<GLint, ()> {
        let mut string = attribute_name.clone();
        string.push('\0');

        match CStr::from_bytes_with_nul(&string.into_bytes()) {
            Ok(cstr) => {
                let loc = unsafe {self.gl.GetAttribLocation(self.id, cstr.as_ptr())};
                if loc >= 0 {
                    return Ok(loc);
                } else {
                    println!("location not found for attribute named {}", attribute_name);
                    Err(())
                }
            }
            _ => {
                println!("uniform named {} is not a formatted as a C string", attribute_name);
                Err(())
            }
        }
    }

    pub fn get_uniform_location(&self, uniform_name: &String) -> Result<GLint, ()>{
        let mut string = uniform_name.clone();
        string.push('\0');

        match CStr::from_bytes_with_nul(&string.into_bytes()) {
            Ok(cstr) => {
                let loc =  unsafe {self.gl.GetUniformLocation(self.id, cstr.as_ptr()) };
                    
                if loc >= 0 {
                    return Ok(loc);
                } else {
                    println!("location not found for uniform named {}", uniform_name);
                    Err(())
                }
            }
            _ => {
                println!("uniform named {} is not a formatted as a C string", uniform_name);
                Err(())
            }
        }

    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            // self.gl.Uniform1f(self.u_offset, self.u_offset_value);
            // self.gl.UniformMatrix4fv(self.u_vp, 1, gl::FALSE, self.u_vp_value.as_ptr());
            let gl = &self.gl;
            for uniform in self.uniforms.values() {
                uniform.load_into_program(gl);
                //self.gl.Uniform1f(self.u_offset, self.u_offset_value);
            }
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

