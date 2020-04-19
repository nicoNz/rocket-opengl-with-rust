use json::JsonValue;
use std::fs;
use std::error::Error;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum JsonToFileError {
    FileError(IoError),
    CastError(std::convert::Infallible),
    ParseError(json::Error)
}



impl std::convert::From<IoError> for JsonToFileError {
    fn from(error: IoError) -> Self {
        JsonToFileError::FileError(error)
    }
}

impl std::convert::From<std::convert::Infallible> for JsonToFileError {
    fn from(error: std::convert::Infallible) -> Self {
        JsonToFileError::CastError(error)
    }
}

impl std::convert::From<json::Error> for JsonToFileError {
    fn from(error: json::Error) -> Self {
        JsonToFileError::ParseError(error)
    }
}

type FileToJsonResult = Result<JsonValue, JsonToFileError>;

pub fn from_file_name(file_name: &String) -> FileToJsonResult {
    let file = fs::read_to_string(file_name)?;
    let string_content = file.parse::<String>()?;
    let json = json::parse(&string_content)?;
    Ok(json)
}
//Box<dyn Error>> 