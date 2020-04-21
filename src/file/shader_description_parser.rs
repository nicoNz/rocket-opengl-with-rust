
use std::collections::HashMap;
use crate::file::json_parser::{from_file_name, JsonToFileError};
use json::{
    JsonValue
};


#[derive(Debug, Clone)]
enum ShaderDescriptionFromFileError {
    ShaderNameNotFound,
    UniformNameNotFound,
    FragmentShaderNotFound,
    VertexShaderNotFound,
    UniformsNotFound,
    UniformsArrayIsEmpty,
    UniformNotFound,
    BadTypeFormValue,
    TypeNotFoundOrNotValid,
    JsonParse
}

use std::fmt;
// This is important for other errors to wrap this one.
impl std::fmt::Display for ShaderDescriptionFromFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


// This is important for other errors to wrap this one.
impl std::error::Error for ShaderDescriptionFromFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

pub enum ShaderSource {
    File(String),
    Raw(String)
}
#[derive(Debug, PartialEq)]
pub enum UniformValue {
    Float32(f32),
    Mat4(glm::Mat4)
}

#[derive(Debug, PartialEq)]
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

impl UniformDescriptionF32 {
    pub fn new (
        name: String,
        is_param: bool,
        default_value: f32,
        min: f32,
        max: f32
    ) -> Self {
        Self {
            name,
            uniform_type: UniformType::Float32,
            is_param,
            default_value,
            min,
            max
        }
    }
}

pub struct UniformDescriptionMat4 {
    name: String,
    uniform_type: UniformType,
    is_param: bool,
    //role: Option<Role>,
    default_value : glm::Mat4
}

impl UniformDescriptionMat4 {
    pub fn new (
        name: String,
        is_param: bool,
    ) -> Self {
        Self {
            name,
            uniform_type: UniformType::Float32,
            is_param,
            default_value : glm::identity(),
        }
    }
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

type Uniforms = Vec<UniformDescription>;
// impl Uniforms {
//     pub fn new() -> Self {
//         Self(Vec::<UniformDescription>::new())
//     }
//     // fn push(&mut self, uniform: UniformDescription) {
//     //     self.push(uniform);
//     // }
// }

// impl std::iter::FromIterator<UniformDescription> for Uniforms {
//     fn from_iter<I: IntoIterator<Item=UniformDescription>>(iter: I) -> Self {
//         let mut c: Uniforms = Uniforms::new();
//         for i in iter {
//             c.0.push(i);
//         }
//         c
//     }
// }



pub struct ShaderDescription {
    pub name: String,
    pub fragment_shader_raw: Option<String>,
    pub vertex_shader_raw: Option<String>,
    pub fragment_shader_file: Option<String>,
    pub vertex_shader_file: Option<String>,
    pub uniforms: Uniforms,
}





fn get_shader_name(json: &JsonValue) -> Result<String, ShaderDescriptionFromFileError> {
    match json["name"].as_str() {
        Some(name) => Ok(String::from(name)),
        None => Err(ShaderDescriptionFromFileError::ShaderNameNotFound)
    }
}

fn get_uniform_name(json: &JsonValue) -> Result<String, ShaderDescriptionFromFileError> {
    match json["name"].as_str() {
        Some(name) => Ok(String::from(name)),
        None => Err(ShaderDescriptionFromFileError::UniformNameNotFound)
    }
}

enum ShaderType {
    Vertex,
    Fragment
}

impl ShaderType {
    pub fn as_string(&self) -> String {
        match self {
            Self::Fragment => String::from("frag"),
            Self::Vertex => String::from("vert")
        }
    }
}

use std::error::Error;

/// return in a Result, the path and the raw content of a givent jsonObject in separate options
/// will return an error if any of the path or content were found
fn get_shader_source(json: &JsonValue, shader_type: ShaderType) -> Result<(Option<String>, Option<String>), ShaderDescriptionFromFileError> {
    match json[shader_type.as_string()].as_str() {
        Some(shader_file_name) => {
            match std::fs::read_to_string(shader_file_name) {
                Ok(content) => Ok(
                    (
                        Some(String::from(shader_file_name)),
                        Some(content),
                    )
                ),
                
                Err(e) => {
                    println!("failt to find source {:?}", e);
                    println!("not found  {:?}", shader_file_name);
                    Err(ShaderDescriptionFromFileError::FragmentShaderNotFound)
                }
            }
        },
        None => Err(ShaderDescriptionFromFileError::FragmentShaderNotFound)
    }

}



fn get_type(json: &JsonValue) -> Result<UniformType, ShaderDescriptionFromFileError> {
    match json["type"].as_str() {
        Some(uniform_type) => {
            UniformType::from_string( &String::from(uniform_type)).or(Err(ShaderDescriptionFromFileError::TypeNotFoundOrNotValid))
        },
        None => {
            Err(ShaderDescriptionFromFileError::TypeNotFoundOrNotValid)
        }
    }
}

fn get_is_param(json: &JsonValue) -> Result<bool, ShaderDescriptionFromFileError> {
    match json["isParam"].as_bool() {
        Some(v) => Ok(v),
        None => Err(ShaderDescriptionFromFileError::TypeNotFoundOrNotValid)
    } 
}

fn get_f32_from_field_name(json: &JsonValue, name: &str) -> Result<f32, ShaderDescriptionFromFileError> {
    
    match json[name].as_f32() {
        Some(num) => Ok(num),
        _ => Err(ShaderDescriptionFromFileError::BadTypeFormValue)
    }
}

fn get_uniform(json: &JsonValue) -> Result<UniformDescription, ShaderDescriptionFromFileError> {
    
    match get_type(json)? {
        UniformType::Float32 => {
            return Ok (
                Box::new(
                    UniformDescriptionF32::new(
                        get_uniform_name(json)?,
                        get_is_param(json)?,
                        get_f32_from_field_name(json, "defaultValue")?,
                        get_f32_from_field_name(json, "min")?,
                        get_f32_from_field_name(json, "max")?,
                    ),

                )
            )
        },
        UniformType::Mat4 => {
            return Ok (
                Box::new(
                    UniformDescriptionMat4::new(
                        get_uniform_name(json)?,
                        get_is_param(json)?,
                    )
                )
            )
        }
    }
    // Ok(
    //     UniformDescription {
    //         name : get_name(json)?,
    //         uniform_type : UniformType::from_string(json["type"].to_str()),
    //         is_param,
    //         default_value
    //     }
    // )
}



fn get_uniforms(json: &JsonValue) -> Result<Uniforms, ShaderDescriptionFromFileError> {
    let uniforms = &json["uniforms"];
    if(uniforms.is_array()) {
        
        if uniforms.len() > 0 {
            match uniforms {
                JsonValue::Array(uniforms_as_json_array) => {
                 
                        //uniforms.into_iter().map(|json|{get_uniform(&json)}).collect()
                        let mut uniforms = Uniforms::new();
                        for uniform_as_json_object in uniforms_as_json_array {
                            let uniform = get_uniform(&uniform_as_json_object)?;
                            uniforms.push(uniform);
                        }
                        Ok(uniforms)
                },
                _ => return Err(ShaderDescriptionFromFileError::UniformsNotFound)
            }
        } else {
            return Err(ShaderDescriptionFromFileError::UniformsNotFound);
        }


    } else if uniforms.is_null() {
        return Err(ShaderDescriptionFromFileError::UniformsNotFound);
    } else {
        return Err(ShaderDescriptionFromFileError::UniformsNotFound);
    }
}



pub type ShaderDescriptionFromFileResult = Result<ShaderDescription, ShaderDescriptionFromFileError>;

impl ShaderDescription {
    pub fn from_file(address: &String) -> ShaderDescriptionFromFileResult {
        match from_file_name(address) {
           
            Ok(json)=>{

                let (fragment_shader_file, fragment_shader_raw) = get_shader_source(&json, ShaderType::Fragment)?;
                let (vertex_shader_file, vertex_shader_raw) = get_shader_source(&json, ShaderType::Vertex)?;
                Ok(
                    Self {
                        name : get_shader_name(&json)?,
                        fragment_shader_file,
                        fragment_shader_raw,
                        vertex_shader_file,
                        vertex_shader_raw,
                        uniforms : get_uniforms(&json)? 
                    }
                )
            },
            Err(e) => Err(ShaderDescriptionFromFileError::JsonParse)
            
        }

    }
}


#[allow(dead_code)]


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::error::Error;
    #[test]
    fn summon() -> Result<(), ShaderDescriptionFromFileError> {
        match ShaderDescription::from_file(&String::from("myshader.json")) {
            Ok(v) => {
                assert_eq!(v.name, "my shader");
                assert_eq!(v.uniforms[0].get_name(), "VP");
                assert_eq!(v.uniforms[0].get_uniform_type(), UniformType::Mat4);
                assert_eq!(v.uniforms[0].is_param(), false);

                assert_eq!(v.uniforms[1].get_name(), "intensity");
                assert_eq!(v.uniforms[1].get_uniform_type(), UniformType::Float32);
                assert_eq!(v.uniforms[1].is_param(), true);
                assert_eq!(v.uniforms[1].get_default_value(), UniformValue::Float32(0.5));
                assert_eq!(v.uniforms[1].get_min(), Some(UniformValue::Float32(0.0)));
                assert_eq!(v.uniforms[1].get_max(), Some(UniformValue::Float32(1.0)));

                Ok(())
            }
            Err(e) => {
                println!("err {}", e);
                println!("err source {:?}", e.source());
                Err(e)
            }
        }
    }

}

/*
{
    "name" : "my shader",
    "frag" : "triangle.frag",
    "vert" : "triangle.vert",
    "uniforms" : [
        {
            "name" : "VP",
            "type" : "mat4",
            "role" : "VP",
            "isParam" : false
        },
        {
            "name" : "intensity",
            "type" : "float32",
            "isParam" : true,
            "defaultValue" : 0.5,
            "min" : 0.0,
            "max" : 1.0
        }

    ]
}
*/