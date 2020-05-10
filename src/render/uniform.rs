//use std::fmt::Display;
//use nalgebra_glm::Mat4;
use gl;
use crate::render::program::Program;
use crate::file::shader_description_parser::UniformDescription;

pub struct UniformData {
    location: gl::types::GLuint,
    label: String,
    name: String,
    data_type: UniformType,
    value: UniformValue
}

impl UniformData {
    pub fn set_f32(&mut self, value: f32) -> Result<(), UniformError> {
        match self.data_type {
            UniformType::F32 => {
                self.value = UniformValue::F32(value);
                Ok(())
            },
            _ => Err(UniformError::BadType)
        }        
    }
}

#[derive(Debug, PartialEq)]
pub enum UniformValue {
    F32(f32),
    Vec3(glm::Vec3),
    Mat4(glm::Mat4)
}

#[derive(Debug, PartialEq, Clone)]
pub enum UniformType {
    F32,
    Mat4,
    Vec3
}
impl UniformType {
    pub fn from_string(type_name: &String) -> Result<Self, String> {
        match type_name.as_str() {
            "float32" => Ok(Self::F32),
            "mat4" => Ok(Self::Mat4),
            "vec3" => Ok(Self::Vec3),

            _ => Err( format!("could no unform type for {}", type_name))
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::F32 => String::from("float32"),
            Self::Mat4 => String::from("mat4"),
            Self::Vec3 => String::from("vec3")
        }
    }
}


/// different roles that helps a user to define how to generate an interface to modify those values
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UniformRole {
    ColorRGB,
    ColorRGBA,
    Int,
    Float,
    Camera,
    Transform,
    Point2D,
    Point3D,
    Bool
}

#[derive(Debug)]
pub enum UniformError {
    BadType,
    NotFoundInBoundShader(String)
}

pub struct Uniform {
    loc: gl::types::GLint,
    value: UniformValue,
    pub name: String,
    role: UniformRole,
    gl: gl::Gl
}


impl Uniform {
    pub fn new(
        loc: gl::types::GLint,
        value: UniformValue,
        name: String,
        role: UniformRole,
        gl: gl::Gl
    ) -> Self {
        Self {
            loc,
            value,
            name,
            role,
            gl
        }
    }
    pub fn from_description_and_program(uniform_description: &UniformDescription, program: &Program) -> Result<Self, String> {
        let name = uniform_description.get_name();
        let loc = match program.get_uniform_location(name) {
            Ok(loc)=>loc,
            Err(())=>{
                return Err(name.to_string())
            }
        };
        println!("uniform crate with name {} at loc {}", name, loc);
        Ok(Self::new (
            loc, 
            uniform_description.get_default_value_as_uniform_value(), 
            name.clone(),
            uniform_description.get_role(), 
            program.gl.clone()
        ))
    }

    pub fn load_into_program(&self) {
        unsafe {
            self.value.load_into_program(&self.gl, self.loc);
        }
    }
    pub fn set_value(&mut self, value: UniformValue) {
        self.value = value;
        self.load_into_program();
    }
}



impl UniformValue {
    pub fn load_into_program(&self, gl: &gl::Gl, loc: gl::types::GLint) {
        match self {
            UniformValue::Mat4(v) => {
                unsafe {
                    gl.UniformMatrix4fv(loc, 1, gl::FALSE, (*v).as_ptr());
                }
            },
            // UniformValue::Vec3(v) => {
            //     unsafe {
            //         gl.Uniform3fv(loc, 1, (*v).as_ptr());
            //     }
            // }
            UniformValue::F32(v) => {
                unsafe {
                    gl.Uniform1f(loc, *v);
                }
            }
            UniformValue::Vec3(v) => {
                unsafe {
                    gl.Uniform3fv(loc, 1,  (*v).as_ptr());
                }
            }
        }
    }
}



// #[derive(PartialEq, Eq, Hash, Copy, Clone)]
// pub enum UniformKey {
//     DirectionnalLightDirection,
//     DirectionnalLightColor,
//     M,
//     V,
//     P,
//     VP,
// }

// impl Display for UniformKey {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             UniformKey::DirectionnalLightColor => write!(f, "Directionnal Light Color"),
//             UniformKey::DirectionnalLightDirection => write!(f, "Directionnal Light Direction"),
//             UniformKey::M => write!(f, "Model Matrix"),
//             UniformKey::V => write!(f, "View Matrix"),
//             UniformKey::P => write!(f, "Projection Matrix"),
//             UniformKey::VP => write!(f, "ViewProjection Matrix"),
//         }
//     }
// }