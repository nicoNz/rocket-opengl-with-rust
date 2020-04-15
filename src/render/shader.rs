


use gl::types::GLint;
use std::fmt::Display;
use gl;
use std;
use std::ffi::{CStr, CString};
use nalgebra_glm;

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
    uniforms: std::collections::HashMap<GLint, Uniform>,
    // u_offset : gl::types::GLint,
    // u_offset_value : f32,
    // u_vp : gl::types::GLint,
    // pub u_vp_value : glm::Mat4,
}

pub struct Uniform {
    loc: gl::types::GLint,
    value: UniformTypedValue,
    name: String,
    role: Role
}

pub enum UniformRole {
    Color,
    Int,
    Float,
    Camera,
    Transform,
    Point2D,
    Point3D
}

impl Uniform {
    pub fn load_into_program(&self, gl: &gl::Gl) {
        unsafe {
            self.value.load_into_program(gl, self.loc);
        }
    }
}

pub enum UniformTypedValue {
    Mat4(Box<nalgebra_glm::Mat4>),
    Vec3(Box<nalgebra_glm::Vec3>)
}

impl UniformTypedValue {
    pub fn load_into_program(&self, gl: &gl::Gl, loc: gl::types::GLint) {
        match self {
            UniformTypedValue::Mat4(v) => {
                unsafe {
                    gl.UniformMatrix4fv(loc, 1, gl::FALSE, (*v).as_ptr());
                }
            },
            UniformTypedValue::Vec3(v) => {
                unsafe {
                    gl.Uniform3fv(loc, 1, (*v).as_ptr());
                }
            }
        }
    }
}

// pub enum UniformRole {
    // DirectionnalLightDirection(UniformType::Mat4),
    // DirectionnalLightColor(Box<nalgebra_glm::Vec3>),
    // M(UniformType),
    // V(Box<nalgebra_glm::Mat4>),
    // P(Box<nalgebra_glm::Mat4>),
    // Color(UniformType),
    // VP(UniformType),
//}

// impl UniformRole {
//     pub fn get_key(&self) -> UniformKey {
//         match self {
//             // UniformRole::DirectionnalLightDirection(_) => UniformKey::DirectionnalLightDirection,
//             // UniformRole::DirectionnalLightColor(_) => UniformKey::DirectionnalLightColor,
//             UniformRole::M(_) => UniformKey::M,
//             // UniformRole::V(_) => UniformKey::V,
//             // UniformRole::P(_) => UniformKey::P,
            
//             UniformRole::VP(_) => UniformKey::VP
//         }
//     }

//     fn to_uniform_name_as_string(&self) -> String {
//         match self {
//             // UniformRole::DirectionnalLightColor(_) => String::from("u_directionnal_light_color"),
//             // UniformRole::DirectionnalLightDirection(_) => String::from("u_directionnal_light_direction"),
//             UniformRole::M(_) => String::from("M"),
//             // UniformRole::V(_) => String::from("V"),
//             // UniformRole::P(_) => String::from("P"),
//             UniformRole::VP(_) => String::from("VP")
//         }
//     }
//     fn load_into_program(&self, gl: &gl::Gl, loc: gl::types::GLint)  {
//         match self {
//             // UniformRole::DirectionnalLightColor(_) => String::from("u_directionnal_light_color"),
//             // UniformRole::DirectionnalLightDirection(_) => String::from("u_directionnal_light_direction"),
//             UniformRole::M(u) => u.load_into_program(gl, loc),
//             // UniformRole::V(_) => String::from("V"),
//             // UniformRole::P(_) => String::from("P"),
//             UniformRole::VP(u) => u.load_into_program(gl, loc),
//         }
//     }

// }


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum UniformKey {
    DirectionnalLightDirection,
    DirectionnalLightColor,
    M,
    V,
    P,
    VP,
}

impl Display for UniformKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UniformKey::DirectionnalLightColor => write!(f, "Directionnal Light Color"),
            UniformKey::DirectionnalLightDirection => write!(f, "Directionnal Light Direction"),
            UniformKey::M => write!(f, "Model Matrix"),
            UniformKey::V => write!(f, "View Matrix"),
            UniformKey::P => write!(f, "Projection Matrix"),
            UniformKey::VP => write!(f, "ViewProjection Matrix"),
        }
    }
}

pub struct Material {

}


impl Program {
    pub fn set_uniform(&mut self, loc: GLint, value: UniformTypedValue) {
        
        if let Some(v) = self.uniforms.get_mut(&loc) {
            v.value = value;
        } else {
            println!("key {} did not exist", loc);
        };
        //std::collections::HashMap<UniformRole, UniformRole>
    }

    pub fn register_uniform(&mut self, location_name: &String, value: UniformTypedValue, role: Role) -> GLint {
        let loc = self.get_location(location_name);
        if !self.uniforms.insert(
            loc, 
            Uniform {
                value,
                loc,
                name: location_name.clone(),
                role 
            }
        ).is_none() {
            println!("key {} already exist", loc)
        }
        loc
    }
    
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
       
        // let u_offset_loc =  unsafe{ 
        //     gl.GetUniformLocation(program_id, CString::new("offset").unwrap().into_raw())
        // };
        // let u_vp_loc =  unsafe{ 
        //     gl.GetUniformLocation(program_id, CString::new("vp").unwrap().into_raw())
        // };

        

        Ok(Program { 
            uniforms : std::collections::HashMap::new(),
            gl : gl.clone(),
            id: program_id,
            // u_offset : u_offset_loc,
            // u_offset_value : 0.0,
            // u_vp : u_vp_loc,
            // u_vp_value : glm::translate(&glm::identity(), &glm::vec3(0.5, 0., 0.)) 
        })
    }


    pub fn get_location(&self, attribute_name: &String) -> i32 {
        let mut string = attribute_name.clone();
        string.push('\0');

        match CStr::from_bytes_with_nul(&string.into_bytes()) {
            Ok(cstr) => {
                unsafe {
                    self.gl.GetAttribLocation(self.id, cstr.as_ptr())
                }
            }
            _ => {
                return 0;
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