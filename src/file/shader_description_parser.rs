
use std::collections::HashMap;
use crate::file::json_parser::{from_file_name, JsonToFileError};
use json::{
    JsonValue
};


#[derive(Debug, Clone)]
enum ShaderDescriptionFromFileError {
    NameNotFound,
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
    name: String,
    fragment_shader: ShaderSource,
    vertex_shader: ShaderSource,
    uniforms: Uniforms,
}





fn get_name(json: &JsonValue) -> Result<String, ShaderDescriptionFromFileError> {
    match json["name"].as_str() {
        Some(name) => Ok(String::from(name)),
        None => Err(ShaderDescriptionFromFileError::NameNotFound)
    }
}

enum ShaderType {
    Vertex,
    Fragment
}

fn get_shader_source(json: &JsonValue, shader_type: ShaderType) -> Result<ShaderSource, ShaderDescriptionFromFileError> {
    Ok(ShaderSource::Raw( String::from("Hello")))
}

// {
//     "name" : "VP",
//     "type" : "mat4",
//     "role" : "VP",
//     "isParam" : false
// },
// {
//     "name" : "intensity",
//     "type" : "float32",
//     "isParam" : true,
//     "defaultValue" : 0.5,
//     "min" : 0.0,
//     "max" : 1.0
// }

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
                        get_name(json)?,
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
                        get_name(json)?,
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
                
                Ok(
                    Self {
                        name : get_name(&json)?,
                        fragment_shader : get_shader_source(&json, ShaderType::Fragment)?,
                        vertex_shader : get_shader_source(&json, ShaderType::Vertex)?,
                        uniforms : get_uniforms(&json)? 
                    }
                )
            },
            Err(e) => Err(ShaderDescriptionFromFileError::JsonParse)
            
        }

    }
}


#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn summon() {
        ShaderDescription::from_file(&String::from("my_shader"));
    }

}
