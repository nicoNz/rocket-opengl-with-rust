


use gl::types::GLint;
use std::fmt::Display;
use gl;
use std;
use std::ffi::{CStr, CString};
use nalgebra_glm;
use crate::file::shader_description_parser::ShaderDescription;


pub struct Uniform {
    loc: gl::types::GLint,
    value: UniformTypedValue,
    name: String,
    role: UniformRole
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

pub fn get_cstr_from_path(path: &String) -> Result<CString, String> {
    match std::fs::read(path) {
        Ok(content) => {
            CString::new(content).or(Err(String::from("path error")))
        },
        Err(e) => Err(String::from("path error"))
    }
}









fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}