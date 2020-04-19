use crate::render::mesh::{MeshDescription, BufferDescription};
use crate::render::vbo::AllowedBufferType;
use std::fs;

use std::collections::HashMap;
use gl;
use std::clone::Clone;
use std::marker::Copy;

enum MakeVbosFromJsonDescriptionErrorReason {
    FailToGetVertexCount,
    VertexCountOutOfRange,
    MissingVboType,
    VboTypeNotValid,
    NoBufferData
}

pub struct MakeVbosFromJsonDescriptionError {
    reason : Option<MakeVbosFromJsonDescriptionErrorReason>,
    parent : Option<Box<MakeVbosFromJsonDescriptionError>>
}

impl MakeVbosFromJsonDescriptionError {
    fn new() -> MakeVbosFromJsonDescriptionError {
        MakeVbosFromJsonDescriptionError {
            reason : None,
            parent : None
        }
    }
    fn from_reason(reason: MakeVbosFromJsonDescriptionErrorReason) -> MakeVbosFromJsonDescriptionError {
        MakeVbosFromJsonDescriptionError {
            reason : Some(reason),
            parent : None
        }
    }
    fn from_parent(parent: MakeVbosFromJsonDescriptionError) -> MakeVbosFromJsonDescriptionError {
        MakeVbosFromJsonDescriptionError {
            reason : None,
            parent : Some(Box::<MakeVbosFromJsonDescriptionError>::new(parent))
        }
    }
    fn from(
        reason: MakeVbosFromJsonDescriptionErrorReason,
        parent: MakeVbosFromJsonDescriptionError
    ) -> MakeVbosFromJsonDescriptionError {
        MakeVbosFromJsonDescriptionError {
            reason : Some(reason),
            parent : Some(Box::<MakeVbosFromJsonDescriptionError>::new(parent))
        }
    }
}



enum AllowedType{
    Float32,
    Short
}

enum AllowedDataType {
    Float32,
    Short
}

pub enum AllowedVboType {
    Float32(VboDescription<f32>),
    Short(VboDescription<u8>)
}

pub struct VboDescription<T> {
    pub attribute_buffer_name : String,
    pub attribute_buffer_data : Vec<T>,
    pub per_vertex : u8,
    pub gl_type : gl::types::GLenum
}


pub struct VaoDescription{
    pub vbos: HashMap::<String, AllowedVboType>,
    pub draw_mode: gl::types::GLenum
}

fn get_per_vertex(content: &json::JsonValue) -> Result<u8, MakeVbosFromJsonDescriptionError>  {
    content["perVertex"].as_u8()
    .ok_or(MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount))
    .and_then(|v|{
        if v < 1 || v > 4 {
            return Err(MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::VertexCountOutOfRange))
        };
        Ok(v)
    })
}

fn get_data_type(content: &json::JsonValue) -> Result<gl::types::GLuint, MakeVbosFromJsonDescriptionError> {
    content["type"].as_str()
    .ok_or(MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount))
    .and_then(|v|{
        match v {
            "float32" => Ok(gl::FLOAT),
            "short" =>  Ok(gl::SHORT),
            _ => Err(
                MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::VboTypeNotValid)
            )
        }
    })
}



fn get_float_buffer_data(content: &json::JsonValue) -> Result<Vec::<f32>, MakeVbosFromJsonDescriptionError> {
    let mut array_data = Vec::<f32>::new();
    for member in content["data"].members() {
        //println!("{:?}", member);
        if let Some(v) = member.as_f32() {
            array_data.push(v);
        }
    }
    
    if array_data.is_empty() {
        return Err(MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::NoBufferData))
    };

    Ok(array_data)
}

fn get_short_buffer_data(content: &json::JsonValue) -> Result<Vec::<u8>, MakeVbosFromJsonDescriptionError> {
    let mut array_data = Vec::<u8>::new();
    for member in content["data"].members() {
        //println!("{:?}", member);
        if let Some(v) = member.as_u8() {
            array_data.push(v);
        }
    }
    
    if array_data.is_empty() {
        return Err(MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::NoBufferData))
    };

    Ok(array_data)
}

fn get_data(
    content: &json::JsonValue, 
    buffer_name: &str,
    per_vertex: u8, 
    data_type: gl::types::GLuint
) -> Result<AllowedBufferType, MakeVbosFromJsonDescriptionError> {

    match data_type {
        gl::FLOAT => {
            get_float_buffer_data(content)
            .and_then(|array_data| {
                return Ok(
                    AllowedBufferType::F32 ( array_data )
                )
            }) 
        },
        gl::SHORT => {
            get_short_buffer_data(content)
            .and_then(|array_data| {
                return Ok(
                    AllowedBufferType::SHORT( array_data )
                )
            })
        }
        _ =>  {           
            return Err(MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount))
        }
    }

}

fn make_vbo_description(buffer_name: &str, content: &json::JsonValue) -> Result<(AllowedBufferType, usize), MakeVbosFromJsonDescriptionError> {



    if let Ok(per_vertex) = get_per_vertex(content) {
        if let Ok(data_type) = get_data_type(content) {
            if let Ok(data) = get_data(content, buffer_name, per_vertex, data_type) {
                return Ok((data, per_vertex as usize))
            } 
        }
    }

    return Err(MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::VboTypeNotValid))
}

//type MakeBufferResult = Result<HashMap::<String, AllowedVboType>, MakeVbosFromJsonDescriptionError>;

fn make_vao_description(v : &json::JsonValue) -> Result<MeshDescription, MakeVbosFromJsonDescriptionError>{
    
    let mut vbos: Vec::<BufferDescription> = Vec::new();
    

    for (buffer_name, buffer_content) in v["buffers"].entries() {
        match make_vbo_description(buffer_name, buffer_content) {
            Ok((buffer, per_vertex)) => vbos.push(
                match buffer {
                    AllowedBufferType::F32(buffer) =>{
                        BufferDescription {
                            attribute_name : buffer_name.to_string(),
                            data : AllowedBufferType::F32(buffer),
                            n_elements : 3,
                            per_vertex : per_vertex as i32,
                        }
                    },
                    AllowedBufferType::SHORT(buffer) =>{
                        BufferDescription {
                            attribute_name : buffer_name.to_string(),
                            data : AllowedBufferType::SHORT(buffer),
                            n_elements : 3,
                            per_vertex : per_vertex as i32,
                        }
                    }
                }
            ),
            Err(e) => {
                return Err(e)
            }
        };
    }

    let draw_mode = match v["drawMode"].as_str().or(Some(&"triangles")) {
        Some("triangles") => gl::TRIANGLES,
        Some("points") => gl::POINTS,
        _ => gl::TRIANGLES,
    };

    Ok(MeshDescription {
        buffers : vbos,
        draw_mode
    })
}


pub fn get_array_data(file_path: String) -> Result<MeshDescription, MakeVbosFromJsonDescriptionError> {


    if let Ok(f) = fs::read_to_string(file_path).unwrap().parse::<String>() {
        //println!("{}", &f );
        if let Ok(json) = json::parse(&f) {
            return make_vao_description(&json);
        } else {
            return Err(MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount))
        }
    }
    Err(MakeVbosFromJsonDescriptionError::from_reason(MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount))
}


