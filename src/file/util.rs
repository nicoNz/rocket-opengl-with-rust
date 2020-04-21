use std::ffi::CString;

pub fn get_cstr_from_file_name() {

}

pub fn get_cstring_from_file_name() {

}

pub fn get_str_from_file_name() {

}

pub fn get_string_from_file_name() {
    
}



pub fn get_cstr_from_path(path: &String) -> Result<CString, String> {
    match std::fs::read(path) {
        Ok(content) => {
            CString::new(content).or(Err(String::from("path error")))
        },
        Err(e) => Err(String::from("path error"))
    }
}
// write methodes ?