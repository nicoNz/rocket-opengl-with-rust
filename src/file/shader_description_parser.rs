use std::collections::HashMap;

pub enum ShaderSource {
    File(String),
    Raw(String)
}

pub enum UniformValue {
    Float32(f32),
    Mat4(glm::Mat4)
}

pub enum UniformType {
    Float32,
    Mat4
}

pub enum UniformParserError {
    FailToCreateUniformTypeFromString
}

impl UniformType {
    pub fn from_string(type_name: &String) -> Result<Self, UniformParserError> {
        match type_name.as_str() {
            "float32" => Ok(Self::Float32),
            "mat4" => Ok(Self::Mat4),
            _ => Err(UniformParserError::FailToCreateUniformTypeFromString)
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Float32 => String::from("float32"),
            Self::Mat4 => String::from("mat4")
        }
    }
}

pub enum Role {
    VP,
}

pub struct UniformDescriptionF32 {
    name: String,
    uniform_type: UniformType,
    is_param: bool,
    //role: Option<Role>,
    default_value: f32,
    min: f32,
    max: f32
}

pub struct UniformDescriptionMat4 {
    name: String,
    uniform_type: UniformType,
    is_param: bool,
    //role: Option<Role>,
    default_value: glm::Mat4,
}

trait InnerUniformDescription {
    fn get_name(&self) -> &String;
    fn get_uniform_type(&self) -> UniformType;
    fn is_param(&self) -> bool;
    fn get_default_value(&self) -> UniformValue;
    fn get_min(&self) -> Option<UniformValue>;
    fn get_max(&self) -> Option<UniformValue>;
}

impl InnerUniformDescription for UniformDescriptionF32 {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_uniform_type(&self) -> UniformType {
        UniformType::Float32
    }
    fn is_param(&self) -> bool {
        self.is_param
    }
    fn get_default_value(&self) -> UniformValue {
        UniformValue::Float32(self.default_value)
    }
    fn get_min(&self) -> Option<UniformValue> {
        Some(UniformValue::Float32(self.min))
    }
    fn get_max(&self) -> Option<UniformValue> {
        Some(UniformValue::Float32(self.max))
    }
}

impl InnerUniformDescription for UniformDescriptionMat4 {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_uniform_type(&self) -> UniformType {
        UniformType::Mat4
    }
    fn is_param(&self) -> bool {
        self.is_param
    }
    fn get_default_value(&self) -> UniformValue {
        UniformValue::Mat4(self.default_value)
    }
    fn get_min(&self) -> Option<UniformValue> {
        None
    }
    fn get_max(&self) -> Option<UniformValue> {
        None
    }
}



type UniformDescription = Box<dyn InnerUniformDescription>;

type Uniforms = HashMap<i16, UniformDescription>;


pub struct ShaderDescription {
    name: String,
    fragment_shader: ShaderSource,
    vertex_shader: ShaderSource,
    uniforms: Uniforms,
}


fn get_json() -> Result<JSON> {

}

fn get_name() -> String {

}

enum ShaderType {
    Vertex,
    Fragment
}

fn get_shader(shader_type: ShaderType) -> Result<ShaderSource, Box<dyn std::error::Error>> {
    
}

fn get_uniforms() -> Result<Uniforms, Box<dyn std::error::Error>> {
    
}

impl ShaderDescription {
    pub fn from_file(address: &String) -> Result<Self, Box<dyn std::error::Error>> {
        match get_json() {
            Ok(json)=>{
                Ok(
                    Self {
                        name : get_name(),
                        fragment_shader : get_shader(ShaderType::Fragment)?,
                        vertex_shader : get_shader(ShaderType::Vertex)?,
                        uniforms : get_uniforms()? 
                    }
                )
            },
            Err(e) => Err(e)
        }

    }
}