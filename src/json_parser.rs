use std::fs;

use std::collections::HashMap;
use gl;

pub struct MakeVbosFromJsonDescriptionError {
    reason : MakeVbosFromJsonDescriptionErrorReason
}

enum MakeVbosFromJsonDescriptionErrorReason {
    FailToGetVertexCount,
    VertexCountOutOfRange,
    MissingVboType,
    VboTypeNotValid,
    NoBufferData
}

enum AllowedType{
    Float32,
    Short
}

enum AllowedDataType {
    Float32,
    Short
}

enum AllowedVboType {
    Float32(VboDescription<f32>),
    Short(VboDescription<u8>)
}

struct VboDescription<T> {
    attribute_buffer_name : String,
    attribute_buffer_data : Vec<T>,
    per_vertex : u8,
    gl_type : gl::types::GLenum
}


pub struct VaoDescription{
    vbos: HashMap::<String, AllowedVboType>,
    draw_mode: gl::types::GLenum
}

fn get_per_vertex(content: &json::JsonValue) -> Result<u8, MakeVbosFromJsonDescriptionError>  {
    content["perVertex"].as_u8().ok_or(MakeVbosFromJsonDescriptionError{
        reason : MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount
    }).and_then(|v|{
        if v < 1 || v > 4 {
            return Err(MakeVbosFromJsonDescriptionError {
                reason : MakeVbosFromJsonDescriptionErrorReason::VertexCountOutOfRange
            })
        };
        Ok(v)
    })
}
fn get_data_type(content: &json::JsonValue) -> Result<gl::types::GLuint, MakeVbosFromJsonDescriptionError> {
    content["type"].as_str().ok_or(MakeVbosFromJsonDescriptionError{
        reason : MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount
    }).and_then(|v|{
        match v {
            "float32" => Ok(gl::FLOAT),
            "short" =>  Ok(gl::SHORT),
            _ => Err(
                MakeVbosFromJsonDescriptionError {
                    reason : MakeVbosFromJsonDescriptionErrorReason::VboTypeNotValid
                }
            )
        }
    })
}



fn get_float_buffer_data(content: &json::JsonValue) -> Result<Vec::<f32>, MakeVbosFromJsonDescriptionError> {
    let mut array_data = Vec::<f32>::new();
    for member in content["data"].members() {
        println!("{:?}", member);
        if let Some(v) = member.as_f32() {
            array_data.push(v);
        }
    }
    
    if array_data.is_empty() {
        return Err(MakeVbosFromJsonDescriptionError {
            reason : MakeVbosFromJsonDescriptionErrorReason::NoBufferData
        })
    };

    Ok(array_data)
}

fn get_short_buffer_data(content: &json::JsonValue) -> Result<Vec::<u8>, MakeVbosFromJsonDescriptionError> {
    let mut array_data = Vec::<u8>::new();
    for member in content["data"].members() {
        println!("{:?}", member);
        if let Some(v) = member.as_u8() {
            array_data.push(v);
        }
    }
    
    if array_data.is_empty() {
        return Err(MakeVbosFromJsonDescriptionError {
            reason : MakeVbosFromJsonDescriptionErrorReason::NoBufferData
        })
    };

    Ok(array_data)
}

fn get_data(
    content: &json::JsonValue, 
    buffer_name: &str,
    per_vertex: u8, 
    data_type: gl::types::GLuint
) -> Result<AllowedVboType, MakeVbosFromJsonDescriptionError> {

    match data_type {
        gl::FLOAT => {
            get_float_buffer_data(content)
            .and_then(|array_data| {
                return Ok(
                    AllowedVboType::Float32 ( 
                        VboDescription {
                            attribute_buffer_name : buffer_name.to_string(),
                            attribute_buffer_data : array_data,
                            per_vertex : per_vertex,
                            gl_type : data_type
                        }
                    )
                )
            }) 
        },
        gl::SHORT => {
            get_short_buffer_data(content)
            .and_then(|array_data| {
                return Ok(
                    AllowedVboType::Short (
                        VboDescription {
                            attribute_buffer_name : buffer_name.to_string(),
                            attribute_buffer_data : array_data,
                            per_vertex : per_vertex,
                            gl_type : data_type
                        }
                    )
                )
            })
        }
        _ =>  {           
            return Err(
                MakeVbosFromJsonDescriptionError {
                    reason : {
                        MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount
                    }
                }
            )
        }
    }

}

fn make_vbo_description(buffer_name: &str, content: &json::JsonValue) -> Result<AllowedVboType, MakeVbosFromJsonDescriptionError> {



    if let Ok(per_vertex) = get_per_vertex(content) {
        if let Ok(data_type) = get_data_type(content) {
            if let Ok(data) = get_data(content, buffer_name, per_vertex, data_type) {
                return Ok(data)
            } 
        }
    }

    return Err(
        MakeVbosFromJsonDescriptionError {
            reason : MakeVbosFromJsonDescriptionErrorReason::VboTypeNotValid
        }
    )
}

//type MakeBufferResult = Result<HashMap::<String, AllowedVboType>, MakeVbosFromJsonDescriptionError>;

fn make_vao_description(v : &json::JsonValue) -> Result<VaoDescription, MakeVbosFromJsonDescriptionError>{
    
    let mut vbos: HashMap::<String, AllowedVboType> = HashMap::new();
    

    for (buffer_name, buffer_content) in v["buffers"].entries() {
        match make_vbo_description(buffer_name, buffer_content) {
            Ok(vbo_description) => vbos.insert(buffer_name.to_string(), vbo_description),
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

    Ok(VaoDescription {
        vbos,
        draw_mode
    })
}


pub fn get_array_data() -> Result<VaoDescription, MakeVbosFromJsonDescriptionError> {


    if let Ok(f) = fs::read_to_string("vertdata.json").unwrap().parse::<String>() {
        println!("{}", &f );
        if let Ok(json) = json::parse(&f) {
            return make_vao_description(&json);
        } else {
            return Err(
                MakeVbosFromJsonDescriptionError {
                    reason : {
                        MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount
                    }
                }
            )
        }
    }
    Err(
        MakeVbosFromJsonDescriptionError {
            reason : {
                MakeVbosFromJsonDescriptionErrorReason::FailToGetVertexCount
            }
        }
    )
}


