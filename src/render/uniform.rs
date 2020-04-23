//use std::fmt::Display;
use gl;
use nalgebra_glm::Mat4;



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
    Mat4(glm::Mat4)
}

#[derive(Debug, PartialEq, Clone)]
pub enum UniformType {
    F32,
    Mat4
}
impl UniformType {
    pub fn from_string(type_name: &String) -> Result<Self, String> {
        match type_name.as_str() {
            "float32" => Ok(Self::F32),
            "mat4" => Ok(Self::Mat4),
            _ => Err( format!("could no unform type for {}", type_name))
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::F32 => String::from("float32"),
            Self::Mat4 => String::from("mat4")
        }
    }
}


/// different roles that helps a user to define how to generate an interface to modify those values
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
pub enum UniformError {
    BadType
}

pub struct Uniform {
    loc: gl::types::GLint,
    value: UniformValue,
    pub name: String,
    role: UniformRole,
    gl: gl::Gl
}


impl Uniform {
    pub fn load_into_program(&self) {
        unsafe {
            self.value.load_into_program(&self.gl, self.loc);
        }
    }
    pub fn set_value(&self, )
}



impl UniformValue {
    pub fn load_into_program(&self, gl: &gl::Gl, loc: gl::types::GLint) {
        match self {
            UniformValue::Mat4(v) => {
                unsafe {
                    gl.UniformMatrix4fv(loc, 1, gl::FALSE, (*v).as_ptr());
                }
            },
            UniformValue::Vec3(v) => {
                unsafe {
                    gl.Uniform3fv(loc, 1, (*v).as_ptr());
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